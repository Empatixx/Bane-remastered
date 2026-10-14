//! Guards the module graph of the crate: no new dependency cycles between top-level modules.
//!
//! The graph is read from the sources: every `crate::name` outside comments and outside the
//! trailing `#[cfg(test)]` module of a file is an edge to the module `name`. Run
//! `cargo test print_module_cycles -- --ignored --nocapture` for the full report.

use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::Path;

/// The cycles the code had when the guard came in. A new cycle fails the build; when one of
/// these shrinks or breaks up, the guard asks to tighten the list so it cannot grow back.
const KNOWN_CYCLES: &[&[&str]] = &[&["prefabs", "systems"]];

/// For every module, the modules it uses and the first place it does so.
type Graph = BTreeMap<String, BTreeMap<String, String>>;

fn module_of(relative: &Path) -> Option<String> {
    let first = relative.components().next()?.as_os_str().to_str()?;
    let name = first.strip_suffix(".rs").unwrap_or(first);
    (name != "main").then(|| name.to_owned())
}

fn used_modules(line: &str) -> Vec<String> {
    let mut found = Vec::new();
    let mut rest = line;
    while let Some(at) = rest.find("crate::") {
        let macro_path = rest[..at].ends_with('$');
        rest = &rest[at + "crate::".len()..];
        let name: String = rest
            .chars()
            .take_while(|c| c.is_ascii_alphanumeric() || *c == '_')
            .collect();
        if !macro_path && !name.is_empty() {
            found.push(name);
        }
    }
    found
}

fn scan_file(graph: &mut Graph, root: &Path, path: &Path) {
    let Some(module) = path.strip_prefix(root).ok().and_then(module_of) else {
        return;
    };
    let Ok(text) = fs::read_to_string(path) else {
        return;
    };
    let code = text.split("#[cfg(test)]\nmod tests").next().unwrap_or("");
    let edges = graph.entry(module.clone()).or_default();
    for (number, line) in code.lines().enumerate() {
        let line = line.split("//").next().unwrap_or("");
        for used in used_modules(line) {
            if used != module {
                let place = path.strip_prefix(root).unwrap_or(path).display();
                edges
                    .entry(used)
                    .or_insert_with(|| format!("{place}:{}", number + 1));
            }
        }
    }
}

fn scan(root: &Path, dir: &Path, graph: &mut Graph) {
    let Ok(entries) = fs::read_dir(dir) else {
        return;
    };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            scan(root, &path, graph);
        } else if path.extension().is_some_and(|extension| extension == "rs") {
            scan_file(graph, root, &path);
        }
    }
}

fn crate_graph() -> Graph {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("src");
    let mut graph = Graph::new();
    scan(&root, &root, &mut graph);
    graph
}

/// The strongly connected groups of more than one module (Tarjan).
fn cycles(graph: &Graph) -> Vec<BTreeSet<String>> {
    struct Search<'a> {
        graph: &'a Graph,
        index: BTreeMap<&'a str, usize>,
        low: BTreeMap<&'a str, usize>,
        stack: Vec<&'a str>,
        found: Vec<BTreeSet<String>>,
    }

    impl<'a> Search<'a> {
        fn visit(&mut self, node: &'a str) {
            let order = self.index.len();
            self.index.insert(node, order);
            self.low.insert(node, order);
            self.stack.push(node);
            for next in self.graph.get(node).into_iter().flat_map(BTreeMap::keys) {
                let next = next.as_str();
                if !self.index.contains_key(next) {
                    self.visit(next);
                    let low = self.low[node].min(self.low[next]);
                    self.low.insert(node, low);
                } else if self.stack.contains(&next) {
                    let low = self.low[node].min(self.index[next]);
                    self.low.insert(node, low);
                }
            }
            if self.low[node] == self.index[node] {
                let mut group = BTreeSet::new();
                while let Some(member) = self.stack.pop() {
                    group.insert(member.to_owned());
                    if member == node {
                        break;
                    }
                }
                if group.len() > 1 {
                    self.found.push(group);
                }
            }
        }
    }

    let mut search = Search {
        graph,
        index: BTreeMap::new(),
        low: BTreeMap::new(),
        stack: Vec::new(),
        found: Vec::new(),
    };
    for node in graph.keys() {
        if !search.index.contains_key(node.as_str()) {
            search.visit(node);
        }
    }
    search.found
}

fn describe(graph: &Graph, group: &BTreeSet<String>) -> String {
    let mut text = format!("cycle of {} modules:\n", group.len());
    for from in group {
        for (to, place) in graph.get(from).into_iter().flatten() {
            if group.contains(to) {
                text.push_str(&format!("  {from} -> {to}   ({place})\n"));
            }
        }
    }
    text
}

fn known() -> Vec<BTreeSet<String>> {
    KNOWN_CYCLES
        .iter()
        .map(|members| members.iter().map(|name| (*name).to_owned()).collect())
        .collect()
}

#[test]
fn no_new_dependency_cycles_between_modules() {
    let graph = crate_graph();
    assert!(graph.len() > 10, "the sources were not found");
    let known = known();
    let new: Vec<String> = cycles(&graph)
        .iter()
        .filter(|group| !known.iter().any(|allowed| group.is_subset(allowed)))
        .map(|group| describe(&graph, group))
        .collect();

    assert!(
        new.is_empty(),
        "new dependency cycles; break them instead of adding them to KNOWN_CYCLES:\n{}",
        new.join("\n")
    );
}

#[test]
fn the_known_cycles_are_listed_exactly() {
    let current = cycles(&crate_graph());
    let stale: Vec<String> = known()
        .into_iter()
        .filter(|allowed| !current.contains(allowed))
        .map(|allowed| format!("{allowed:?}"))
        .collect();

    assert!(
        stale.is_empty(),
        "these known cycles shrank or broke up, so tighten KNOWN_CYCLES to today's: {:?}\n\
         stale: {}",
        current,
        stale.join(", ")
    );
}

#[test]
#[ignore = "a report, run it with --ignored --nocapture"]
fn print_module_cycles() {
    let graph = crate_graph();
    for group in cycles(&graph) {
        println!("{}", describe(&graph, &group));
    }
}

#[test]
fn the_search_finds_cycles_and_ignores_chains() {
    let mut graph = Graph::new();
    let mut edge = |from: &str, to: &str| {
        graph
            .entry(from.to_owned())
            .or_default()
            .insert(to.to_owned(), String::new());
    };
    edge("a", "b");
    edge("b", "c");
    edge("c", "a");
    edge("c", "d");
    edge("d", "e");

    let found = cycles(&graph);

    assert_eq!(found.len(), 1);
    assert_eq!(
        found[0],
        ["a", "b", "c"].map(str::to_owned).into_iter().collect()
    );
}

#[test]
fn only_real_crate_paths_count() {
    assert_eq!(
        used_modules("use crate::guns::{GunKind}; crate::assets::tex"),
        ["guns", "assets"]
    );
    assert!(used_modules("pub(crate) fn x() { $crate::foo }").is_empty());
}
