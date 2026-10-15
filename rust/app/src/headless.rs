//! Runs the game without a window and saves what it would show, for screenshots on machines
//! (or in sessions) where nobody should see a window pop up.
//!
//! ```text
//! BANE_START=dungeon BANE_THEME=moss cargo run -- --shot moss.png --seconds 8
//! cargo run -- --shot run.png --shots 4 --interval 0.5 --hold D:3-5 --fire 4-6 --mouse 700,270
//! ```
//!
//! The game steps at a fixed 60 updates per second, so the same arguments (and `BANE_SEED`)
//! give the same pictures. Times count from the moment the assets are loaded; the loading
//! screen still shows for its usual half second after that.

use std::error::Error;
use std::ops::ControlFlow;
use std::path::{Path, PathBuf};

use glam::Vec2;
use winit::event::{ElementState, MouseButton};
use winit::keyboard::KeyCode;

use crate::assets::{Assets, Loader};
use crate::game::Game;
use crate::profile::SharedProfile;
use crate::render::Gpu;
use crate::settings::Settings;
use crate::storage;
use crate::viewport::TARGET;

const STEP: f32 = 1.0 / 60.0;
const DEFAULT_SECONDS: f32 = 6.0;
const DEFAULT_SIZE: (u32, u32) = (1920, 1080);

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct Options {
    path: PathBuf,
    seconds: f32,
    shots: usize,
    interval: f32,
    size: (u32, u32),
    mouse: Vec2,
    holds: Vec<Hold>,
    fire: Vec<(f32, f32)>,
}

/// A key held down from one moment to another.
#[derive(Debug, Clone, Copy, PartialEq)]
struct Hold {
    key: KeyCode,
    from: f32,
    to: f32,
}

const USAGE: &str = "headless: bane --shot <file.png> [--seconds S] [--shots N --interval S] \
[--size WxH] [--mouse X,Y] [--hold KEY:FROM-TO]... [--fire FROM-TO]...";

impl Options {
    /// `None` when the arguments do not ask for a headless run.
    pub(crate) fn parse(args: &[String]) -> Option<Result<Self, String>> {
        args.iter()
            .any(|arg| arg == "--shot")
            .then(|| Self::parse_all(args).map_err(|error| format!("{error}\n{USAGE}")))
    }

    fn parse_all(args: &[String]) -> Result<Self, String> {
        let mut options = Self {
            path: PathBuf::new(),
            seconds: DEFAULT_SECONDS,
            shots: 1,
            interval: 0.5,
            size: DEFAULT_SIZE,
            mouse: TARGET / 2.0,
            holds: Vec::new(),
            fire: Vec::new(),
        };
        let mut args = args.iter();
        while let Some(flag) = args.next() {
            let mut value = || {
                args.next()
                    .map(String::as_str)
                    .ok_or_else(|| format!("{flag} needs a value"))
            };
            match flag.as_str() {
                "--shot" => options.path = PathBuf::from(value()?),
                "--seconds" => options.seconds = number(value()?)?,
                "--shots" => {
                    options.shots = value()?
                        .parse()
                        .map_err(|_| "--shots takes a whole number".to_owned())?;
                }
                "--interval" => options.interval = number(value()?)?,
                "--size" => {
                    let (width, height) = pair(value()?, 'x')?;
                    options.size = (width as u32, height as u32);
                }
                "--mouse" => {
                    let (x, y) = pair(value()?, ',')?;
                    options.mouse = Vec2::new(x, y);
                }
                "--hold" => {
                    let raw = value()?;
                    let (key, span) = raw
                        .split_once(':')
                        .ok_or_else(|| format!("--hold {raw}: write KEY:FROM-TO"))?;
                    let (from, to) = pair(span, '-')?;
                    options.holds.push(Hold {
                        key: key_code(key).ok_or_else(|| format!("unknown key {key}"))?,
                        from,
                        to,
                    });
                }
                "--fire" => options.fire.push(pair(value()?, '-')?),
                other => return Err(format!("unknown argument {other}")),
            }
        }
        if options.path.as_os_str().is_empty() {
            return Err("--shot needs a file".to_owned());
        }
        if options.shots == 0 || options.size.0 == 0 || options.size.1 == 0 {
            return Err("--shots and --size must be above zero".to_owned());
        }
        Ok(options)
    }

    /// The moments a picture is taken, the last one at `seconds`.
    fn moments(&self) -> Vec<f32> {
        (0..self.shots)
            .map(|index| self.seconds - (self.shots - 1 - index) as f32 * self.interval)
            .collect()
    }

    fn file(&self, index: usize) -> PathBuf {
        if self.shots == 1 {
            return self.path.clone();
        }
        let stem = self
            .path
            .file_stem()
            .map_or_else(|| "shot".into(), |stem| stem.to_string_lossy());
        let extension = self
            .path
            .extension()
            .map_or_else(|| "png".into(), |extension| extension.to_string_lossy());
        self.path
            .with_file_name(format!("{stem}_{:02}.{extension}", index + 1))
    }

    fn holding(&self, key: KeyCode, time: f32) -> bool {
        self.holds
            .iter()
            .any(|hold| hold.key == key && (hold.from..hold.to).contains(&time))
    }

    fn firing(&self, time: f32) -> bool {
        self.fire
            .iter()
            .any(|(from, to)| (*from..*to).contains(&time))
    }
}

fn number(raw: &str) -> Result<f32, String> {
    raw.trim()
        .parse()
        .map_err(|_| format!("{raw} is not a number"))
}

fn pair(raw: &str, separator: char) -> Result<(f32, f32), String> {
    let (a, b) = raw
        .split_once(separator)
        .ok_or_else(|| format!("{raw}: write two numbers split by '{separator}'"))?;
    Ok((number(a)?, number(b)?))
}

fn key_code(name: &str) -> Option<KeyCode> {
    let upper = name.to_ascii_uppercase();
    let letters = [
        KeyCode::KeyA,
        KeyCode::KeyB,
        KeyCode::KeyC,
        KeyCode::KeyD,
        KeyCode::KeyE,
        KeyCode::KeyF,
        KeyCode::KeyG,
        KeyCode::KeyH,
        KeyCode::KeyI,
        KeyCode::KeyJ,
        KeyCode::KeyK,
        KeyCode::KeyL,
        KeyCode::KeyM,
        KeyCode::KeyN,
        KeyCode::KeyO,
        KeyCode::KeyP,
        KeyCode::KeyQ,
        KeyCode::KeyR,
        KeyCode::KeyS,
        KeyCode::KeyT,
        KeyCode::KeyU,
        KeyCode::KeyV,
        KeyCode::KeyW,
        KeyCode::KeyX,
        KeyCode::KeyY,
        KeyCode::KeyZ,
    ];
    let digits = [
        KeyCode::Digit0,
        KeyCode::Digit1,
        KeyCode::Digit2,
        KeyCode::Digit3,
        KeyCode::Digit4,
        KeyCode::Digit5,
        KeyCode::Digit6,
        KeyCode::Digit7,
        KeyCode::Digit8,
        KeyCode::Digit9,
    ];
    let mut chars = upper.chars();
    if let (Some(single), None) = (chars.next(), chars.next()) {
        if single.is_ascii_uppercase() {
            return letters.get((single as u8 - b'A') as usize).copied();
        }
        if single.is_ascii_digit() {
            return digits.get((single as u8 - b'0') as usize).copied();
        }
    }
    match upper.as_str() {
        "SPACE" => Some(KeyCode::Space),
        "ESCAPE" | "ESC" => Some(KeyCode::Escape),
        "TAB" => Some(KeyCode::Tab),
        "ENTER" => Some(KeyCode::Enter),
        "SHIFT" => Some(KeyCode::ShiftLeft),
        _ => None,
    }
}

fn all_keys(options: &Options) -> Vec<KeyCode> {
    let mut keys: Vec<KeyCode> = options.holds.iter().map(|hold| hold.key).collect();
    keys.dedup();
    keys
}

pub(crate) fn run(options: &Options) -> Result<(), Box<dyn Error>> {
    let mut gpu = Gpu::headless(options.size.0, options.size.1)?;
    let mut assets = Assets::new()?;
    let mut loader = Loader::start(
        &mut gpu,
        &mut assets,
        &crate::components::BarKind::holed_textures(),
    )?;
    let settings = Settings::load(&storage::data_file("settings.json"));
    let profile = SharedProfile::load(&storage::data_file("profile.json"));
    let mut game = Game::silent(assets, settings, profile);
    let _ = game.take_setting_changes();
    while !loader.is_done() {
        game.load_step(&mut gpu, &mut loader)?;
    }

    let keys = all_keys(options);
    let moments = options.moments();
    let mut next = 0;
    let mut steps: u64 = 0;
    while next < moments.len() {
        let time = steps as f32 * STEP;
        drive(&mut game, options, &keys, time);
        if game.fixed_update(STEP).is_break() {
            break;
        }
        game.set_blend(0.0);
        if game.update(STEP) == ControlFlow::Break(()) {
            break;
        }
        steps += 1;
        while next < moments.len() && steps as f32 * STEP >= moments[next] {
            save(&mut gpu, &game, &options.file(next))?;
            next += 1;
        }
    }
    Ok(())
}

fn drive(game: &mut Game, options: &Options, keys: &[KeyCode], time: f32) {
    game.set_mouse(options.mouse);
    let was_firing = time >= STEP && options.firing(time - STEP);
    let firing = options.firing(time);
    if firing != was_firing {
        let state = if firing {
            ElementState::Pressed
        } else {
            ElementState::Released
        };
        game.handle_click(MouseButton::Left, state);
    }
    let input = game.input_mut();
    for &key in keys {
        let held = options.holding(key, time);
        let was_held = time >= STEP && options.holding(key, time - STEP);
        match (was_held, held) {
            (false, true) => input.press(key),
            (true, false) => input.release(key),
            _ => {}
        }
    }
}

fn save(gpu: &mut Gpu, game: &Game, path: &Path) -> Result<(), Box<dyn Error>> {
    gpu.render(wgpu::Color::BLACK, &game.frame());
    let (width, height, rgba) = gpu.capture()?;
    let image = image::RgbaImage::from_raw(width, height, rgba)
        .ok_or("the captured frame has the wrong size")?;
    image.save(path)?;
    println!("{}", path.display());
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn args(line: &str) -> Vec<String> {
        line.split_whitespace().map(str::to_owned).collect()
    }

    #[test]
    fn a_normal_start_is_not_headless() {
        assert_eq!(Options::parse(&args("")), None);
        assert_eq!(Options::parse(&args("--seconds 3")), None);
    }

    #[test]
    fn the_flags_are_read() {
        let options = Options::parse(&args(
            "--shot a.png --seconds 4 --shots 3 --interval 0.5 --size 960x540 \
             --mouse 100,200 --hold d:1-2.5 --hold Space:2-2.1 --fire 3-4",
        ))
        .expect("headless")
        .expect("valid");
        assert_eq!(options.size, (960, 540));
        assert_eq!(options.mouse, Vec2::new(100.0, 200.0));
        assert_eq!(options.moments(), vec![3.0, 3.5, 4.0]);
        assert!(options.holding(KeyCode::KeyD, 1.5));
        assert!(!options.holding(KeyCode::KeyD, 2.5));
        assert!(options.holding(KeyCode::Space, 2.05));
        assert!(options.firing(3.5) && !options.firing(4.5));
        assert_eq!(options.file(1), PathBuf::from("a_02.png"));
    }

    #[test]
    fn mistakes_are_reported() {
        for line in [
            "--shot",
            "--shot a.png --hold Q",
            "--shot a.png --hold F13:1-2",
            "--shot a.png --size 10",
            "--shot a.png --wat",
        ] {
            assert!(
                matches!(Options::parse(&args(line)), Some(Err(_))),
                "{line}"
            );
        }
    }

    #[test]
    fn one_shot_keeps_its_name() {
        let options = Options::parse(&args("--shot out/x.png"))
            .expect("headless")
            .expect("valid");
        assert_eq!(options.file(0), PathBuf::from("out/x.png"));
        assert_eq!(options.moments(), vec![DEFAULT_SECONDS]);
    }
}
