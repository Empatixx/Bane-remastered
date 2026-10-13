use std::env;
use std::fs::{self, File};
use std::io::{self, ErrorKind, Write};
use std::path::{Path, PathBuf};
use std::sync::OnceLock;
use std::time::{SystemTime, UNIX_EPOCH};

use directories::ProjectDirs;
use serde::de::DeserializeOwned;

const ASSETS_VAR: &str = "BANE_ASSETS";
const DATA_VAR: &str = "BANE_DATA";
const ASSET_DIRS: [&str; 2] = ["assets", "Resources/assets"];

#[derive(Debug)]
pub(crate) enum Loaded<T> {
    Missing,
    Parsed(T),
    Corrupt,
    Unreadable,
}

pub(crate) fn assets_root() -> &'static Path {
    static ROOT: OnceLock<PathBuf> = OnceLock::new();
    ROOT.get_or_init(|| {
        let root = find_assets().unwrap_or_else(|| {
            tracing::error!("cannot find the assets folder, set {ASSETS_VAR} to its path");
            PathBuf::from("assets")
        });
        tracing::info!(path = %root.display(), "using the assets");
        root
    })
}

fn find_assets() -> Option<PathBuf> {
    if let Some(dir) = env::var_os(ASSETS_VAR) {
        return Some(PathBuf::from(dir));
    }
    let exe = env::current_exe().ok()?;
    exe.ancestors()
        .skip(1)
        .flat_map(|dir| ASSET_DIRS.map(|name| dir.join(name)))
        .find(|candidate| candidate.join("textures").is_dir())
}

pub(crate) fn data_file(name: &str) -> PathBuf {
    let dir = data_dir();
    if let Err(error) = fs::create_dir_all(&dir) {
        tracing::error!(%error, path = %dir.display(), "cannot create the data folder");
    }
    let path = dir.join(name);
    adopt_legacy_file(Path::new(name), &path);
    path
}

fn data_dir() -> PathBuf {
    if let Some(dir) = env::var_os(DATA_VAR) {
        return PathBuf::from(dir);
    }
    ProjectDirs::from("cz", "empatix", "Bane")
        .map(|dirs| dirs.data_dir().to_path_buf())
        .unwrap_or_else(|| PathBuf::from("."))
}

fn adopt_legacy_file(legacy: &Path, path: &Path) {
    if path.exists() || !legacy.is_file() || legacy == path {
        return;
    }
    match fs::copy(legacy, path) {
        Ok(_) => tracing::info!(
            from = %legacy.display(),
            to = %path.display(),
            "moved the file into the data folder"
        ),
        Err(error) => tracing::warn!(%error, from = %legacy.display(), "cannot copy the old file"),
    }
}

pub(crate) fn load_json<T: DeserializeOwned>(path: &Path) -> Loaded<T> {
    let text = match fs::read_to_string(path) {
        Ok(text) => text,
        Err(error) if error.kind() == ErrorKind::NotFound => return Loaded::Missing,
        Err(error) => {
            tracing::error!(%error, path = %path.display(), "cannot read the file, it will not be overwritten");
            return Loaded::Unreadable;
        }
    };
    match serde_json::from_str(&text) {
        Ok(value) => Loaded::Parsed(value),
        Err(error) => {
            let backup = set_aside(path);
            tracing::error!(
                %error,
                path = %path.display(),
                backup = ?backup,
                "the file is corrupt, it was set aside and starts fresh"
            );
            backup.map_or(Loaded::Unreadable, |_| Loaded::Corrupt)
        }
    }
}

fn set_aside(path: &Path) -> Option<PathBuf> {
    let stamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_or(0, |elapsed| elapsed.as_secs());
    let backup = with_suffix(path, &format!("corrupt-{stamp}"));
    fs::rename(path, &backup).ok().map(|()| backup)
}

pub(crate) fn write_atomic(path: &Path, contents: &str) -> io::Result<()> {
    let temporary = with_suffix(path, "tmp");
    let mut file = File::create(&temporary)?;
    file.write_all(contents.as_bytes())?;
    file.sync_all()?;
    drop(file);
    fs::rename(&temporary, path)
}

fn with_suffix(path: &Path, suffix: &str) -> PathBuf {
    let mut name = path.file_name().unwrap_or_default().to_os_string();
    name.push(".");
    name.push(suffix);
    path.with_file_name(name)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn temp_path(name: &str) -> PathBuf {
        env::temp_dir().join(format!("bane-storage-{name}-{}.json", std::process::id()))
    }

    #[test]
    fn the_assets_are_found_from_the_test_binary() {
        assert!(assets_root().join("textures").is_dir());
    }

    #[test]
    fn an_atomic_write_replaces_the_file_and_leaves_no_temporary_behind() {
        let path = temp_path("atomic");
        fs::write(&path, "old").expect("a temp file");

        write_atomic(&path, "new").expect("the write succeeds");

        assert_eq!(fs::read_to_string(&path).expect("the file"), "new");
        assert!(!with_suffix(&path, "tmp").exists());
        let _ = fs::remove_file(&path);
    }

    #[test]
    fn a_corrupt_file_is_set_aside_instead_of_being_lost() {
        let path = temp_path("corrupt");
        fs::write(&path, "not json").expect("a temp file");

        assert!(matches!(load_json::<u32>(&path), Loaded::Corrupt));

        assert!(!path.exists());
        let dir = path.parent().expect("a temp dir");
        let prefix = format!(
            "{}.corrupt-",
            path.file_name()
                .and_then(|n| n.to_str())
                .unwrap_or_default()
        );
        let backups: Vec<PathBuf> = fs::read_dir(dir)
            .expect("the temp dir")
            .filter_map(|entry| entry.ok().map(|entry| entry.path()))
            .filter(|candidate| {
                candidate
                    .file_name()
                    .and_then(|name| name.to_str())
                    .is_some_and(|name| name.starts_with(&prefix))
            })
            .collect();
        assert_eq!(backups.len(), 1);
        assert_eq!(
            fs::read_to_string(&backups[0]).expect("the backup"),
            "not json"
        );
        for backup in backups {
            let _ = fs::remove_file(backup);
        }
    }

    #[test]
    fn a_missing_file_is_reported_as_missing() {
        let path = temp_path("missing");
        let _ = fs::remove_file(&path);
        assert!(matches!(load_json::<u32>(&path), Loaded::Missing));
    }
}
