use crate::guns::{GunKind, GunSpec, MAX_CLIP, Mutation};
use crate::rng::Rng;

pub(crate) const HALF_COLUMNS: i32 = 2;
pub(crate) const DEPTH: i32 = 4;
pub(crate) const STAT_KINDS: usize = 6;
pub(crate) const MUTATION_KINDS: usize = Mutation::ALL.len();
/// Bump whenever `generate` grows different trees. Saves point at nodes by their index, so a
/// tree saved by another generator is paid back and reset instead of shuffling the purchases.
pub(crate) const GENERATOR: u32 = 2;
const SIDE_BRANCHES: usize = 2;
const SIDE_DEPTH: usize = 3;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Stat {
    Damage,
    FireRate,
    Magazine,
    Reload,
    Accuracy,
    Crit,
}

impl Stat {
    pub(crate) const ALL: [Self; STAT_KINDS] = [
        Self::Damage,
        Self::FireRate,
        Self::Magazine,
        Self::Reload,
        Self::Accuracy,
        Self::Crit,
    ];

    /// Whether the bonus changes anything for the gun: accuracy on a gun without spread or more
    /// rounds in a magazine that is already as big as the clip HUD allows do nothing.
    pub(crate) fn helps(self, spec: &GunSpec) -> bool {
        match self {
            Self::Accuracy => spec.inaccuracy > 0.0,
            Self::Magazine => spec.clip_size < MAX_CLIP as i32,
            Self::Damage | Self::FireRate | Self::Reload | Self::Crit => true,
        }
    }

    /// The bonus of a common node, in percent or in rounds for the magazine.
    pub(crate) fn amount(self) -> u32 {
        match self {
            Self::Damage => 5,
            Self::FireRate => 4,
            Self::Magazine => 1,
            Self::Reload => 6,
            Self::Accuracy => 8,
            Self::Crit => 4,
        }
    }
}

/// The stats and mutations a tree may offer, as indices into `Stat::ALL` and `Mutation::ALL`.
#[derive(Debug, Clone, PartialEq, Eq)]
struct Choices {
    stats: Vec<usize>,
    mutations: Vec<usize>,
}

impl Choices {
    fn all() -> Self {
        Self {
            stats: (0..STAT_KINDS).collect(),
            mutations: (0..MUTATION_KINDS).collect(),
        }
    }

    /// Only what changes something for this gun, e.g. no accuracy for a gun that never misses.
    fn for_gun(kind: GunKind) -> Self {
        let spec = kind.spec();
        Self {
            stats: (0..STAT_KINDS)
                .filter(|id| Stat::ALL[*id].helps(&spec))
                .collect(),
            mutations: (0..MUTATION_KINDS)
                .filter(|id| Mutation::ALL[*id].helps(&spec))
                .collect(),
        }
    }

    fn stat(&self, rng: &mut Rng) -> usize {
        self.stats[rng.below(self.stats.len())]
    }

    fn mutation(&self, rng: &mut Rng) -> usize {
        self.mutations[rng.below(self.mutations.len())]
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct Node {
    pub(crate) column: i32,
    pub(crate) row: i32,
    pub(crate) parent: Option<usize>,
    pub(crate) branch: usize,
    pub(crate) stat: usize,
    pub(crate) mutation: Option<usize>,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub(crate) struct Tree {
    pub(crate) nodes: Vec<Node>,
}

impl Tree {
    fn at(&self, column: i32, row: i32) -> Option<usize> {
        self.nodes
            .iter()
            .position(|node| node.column == column && node.row == row)
    }

    fn origin(&self, parent: Option<usize>) -> (i32, i32) {
        parent.map_or((0, 0), |index| {
            (self.nodes[index].column, self.nodes[index].row)
        })
    }

    fn fits(&self, parent: Option<usize>, column: i32) -> bool {
        let (parent_column, parent_row) = self.origin(parent);
        let row = parent_row + 1;
        if column.abs() > HALF_COLUMNS || row > DEPTH || self.at(column, row).is_some() {
            return false;
        }
        let bent_below = self
            .at(column, row + 1)
            .is_some_and(|index| self.origin(self.nodes[index].parent).0 != column);
        if bent_below {
            return false;
        }
        if column != parent_column {
            let foreign_above = self.at(column, parent_row);
            if foreign_above.is_some() && foreign_above != parent {
                return false;
            }
            let below_parent = self.at(parent_column, row);
            if below_parent.is_some_and(|index| self.nodes[index].parent != parent) {
                return false;
            }
        }
        self.nodes.iter().all(|other| {
            if other.row != row || other.parent == parent {
                return true;
            }
            let (other_column, _) = self.origin(other.parent);
            !((parent_column < other_column && column > other.column)
                || (parent_column > other_column && column < other.column))
        })
    }

    fn add(
        &mut self,
        rng: &mut Rng,
        choices: &Choices,
        parent: Option<usize>,
        column: i32,
        branch: usize,
        mutation: Option<usize>,
    ) -> usize {
        let row = self.origin(parent).1 + 1;
        self.nodes.push(Node {
            column,
            row,
            parent,
            branch,
            stat: choices.stat(rng),
            mutation,
        });
        self.nodes.len() - 1
    }

    fn grow(
        &mut self,
        rng: &mut Rng,
        choices: &Choices,
        parent: Option<usize>,
        branch: usize,
        straight: bool,
    ) -> Option<usize> {
        let from = self.origin(parent).0;
        let mut options = [0, -1, 1];
        if !straight {
            options.rotate_left(rng.below(3));
        }
        let wanted = if straight { 1 } else { options.len() };
        let column = options[..wanted]
            .iter()
            .map(|step| from + step)
            .find(|column| self.fits(parent, *column))?;
        Some(self.add(rng, choices, parent, column, branch, None))
    }
}

/// Seeded by the name of the weapon, so adding or reordering guns never changes a tree.
pub(crate) fn generate(kind: GunKind, rerolls: u32) -> Tree {
    grow_tree(
        name_seed(kind.key()) ^ u64::from(rerolls).wrapping_mul(104_729),
        &Choices::for_gun(kind),
    )
}

/// The trees of the first versions, seeded by the place of the gun in the list; only kept to
/// pay back what those saves spent.
pub(crate) fn generate_unversioned(kind: GunKind, rerolls: u32) -> Tree {
    let gun = GunKind::ALL
        .iter()
        .position(|other| *other == kind)
        .unwrap_or(0) as u64;
    grow_tree(
        gun * 7919 + u64::from(rerolls) * 104_729 + 13,
        &Choices::all(),
    )
}

fn name_seed(name: &str) -> u64 {
    name.bytes().fold(0xcbf2_9ce4_8422_2325, |hash, byte| {
        (hash ^ u64::from(byte)).wrapping_mul(0x0100_0000_01b3)
    })
}

fn grow_tree(seed: u64, choices: &Choices) -> Tree {
    let mut rng = Rng::new(seed);
    let mut tree = Tree::default();
    let start = rng.below(3) as i32 - 1;
    let mut last = tree.add(&mut rng, choices, None, start, 0, None);
    for _ in 2..DEPTH {
        match tree.grow(&mut rng, choices, Some(last), 0, false) {
            Some(next) => last = next,
            None => break,
        }
    }
    let end = tree.nodes[last].column;
    let mutation = choices.mutation(&mut rng);
    tree.add(&mut rng, choices, Some(last), end, 0, Some(mutation));
    let sides = 1 + rng.below(SIDE_BRANCHES);
    for branch in 1..=sides {
        let open: Vec<i32> = (-HALF_COLUMNS..=HALF_COLUMNS)
            .filter(|column| tree.fits(None, *column))
            .collect();
        let near = |column: &i32, reach: i32| {
            tree.nodes
                .iter()
                .filter(|node| node.row == 1)
                .any(|node| (node.column - column).abs() <= reach)
        };
        let close: Vec<i32> = open.iter().copied().filter(|c| near(c, 1)).collect();
        let free = if close.is_empty() { open } else { close };
        if free.is_empty() {
            break;
        }
        let column = free[rng.below(free.len())];
        let first = tree.add(&mut rng, choices, None, column, branch, None);
        let depth = 1 + rng.below(SIDE_DEPTH);
        let mut tip = first;
        for _ in 1..depth {
            match tree.grow(&mut rng, choices, Some(tip), branch, false) {
                Some(next) => tip = next,
                None => break,
            }
        }
        for _ in 0..2 {
            if tree.nodes[tip].row >= 2 && rng.below(2) == 0 {
                let members: Vec<usize> = (0..tree.nodes.len())
                    .filter(|index| {
                        tree.nodes[*index].branch == branch
                            && tree.nodes[*index].row < tree.nodes[tip].row
                    })
                    .collect();
                let from = members[rng.below(members.len())];
                if let Some(fork) = tree.grow(&mut rng, choices, Some(from), branch, false) {
                    if tree.nodes[fork].row < DEPTH - 1 && rng.below(2) == 0 {
                        tree.grow(&mut rng, choices, Some(fork), branch, true);
                    }
                }
            }
        }
    }
    tree
}

#[cfg(test)]
mod tests {
    use super::*;

    /// What `generate` grew for GENERATOR 2. Saves point at nodes by index, so when this
    /// changes the trees changed: bump GENERATOR (old saves get their gold back), then update
    /// the number here.
    const FINGERPRINT: u64 = 15_746_248_693_106_410_667;

    /// The guns the fingerprint was taken over; newer guns grow their own trees and leave
    /// these alone.
    const FINGERPRINTED: usize = 7;

    fn fingerprint() -> u64 {
        GunKind::ALL[..FINGERPRINTED]
            .iter()
            .copied()
            .flat_map(|kind| (0..10).map(move |rerolls| generate(kind, rerolls)))
            .flat_map(|tree| tree.nodes)
            .flat_map(|node| {
                [
                    node.column as u64,
                    node.row as u64,
                    node.parent.map_or(u64::MAX, |parent| parent as u64),
                    node.branch as u64,
                    node.stat as u64,
                    node.mutation.map_or(u64::MAX, |id| id as u64),
                ]
            })
            .fold(0xcbf2_9ce4_8422_2325, |hash, value| {
                (hash ^ value).wrapping_mul(0x0100_0000_01b3)
            })
    }

    #[test]
    fn the_trees_only_change_together_with_the_generator_version() {
        assert_eq!(GENERATOR, 2);
        assert_eq!(
            fingerprint(),
            FINGERPRINT,
            "the weapon trees changed: bump GENERATOR and update FINGERPRINT"
        );
    }

    #[test]
    fn a_tree_does_not_depend_on_where_the_gun_sits_in_the_list() {
        assert_ne!(name_seed("pistol"), name_seed("uzi"));
        assert_eq!(
            generate(GunKind::Uzi, 0),
            grow_tree(name_seed("uzi"), &Choices::for_gun(GunKind::Uzi))
        );
    }

    fn all() -> impl Iterator<Item = Tree> {
        GunKind::ALL
            .into_iter()
            .flat_map(|kind| (0..40).map(move |rerolls| generate(kind, rerolls)))
    }

    #[test]
    fn no_tree_offers_a_node_that_does_nothing_for_its_gun() {
        for kind in GunKind::ALL {
            let spec = kind.spec();
            for rerolls in 0..40 {
                for node in generate(kind, rerolls).nodes {
                    match node.mutation {
                        Some(id) => assert!(Mutation::ALL[id].helps(&spec), "{kind:?} {node:?}"),
                        None => assert!(Stat::ALL[node.stat].helps(&spec), "{kind:?} {node:?}"),
                    }
                }
            }
        }
        let stats = |kind: GunKind| -> Vec<Stat> {
            (0..40)
                .flat_map(|rerolls| generate(kind, rerolls).nodes)
                .filter(|node| node.mutation.is_none())
                .map(|node| Stat::ALL[node.stat])
                .collect()
        };
        let mutations = |kind: GunKind| -> Vec<Mutation> {
            (0..40)
                .flat_map(|rerolls| generate(kind, rerolls).nodes)
                .filter_map(|node| node.mutation)
                .map(|id| Mutation::ALL[id])
                .collect()
        };
        assert!(!stats(GunKind::M4).contains(&Stat::Accuracy));
        assert!(!stats(GunKind::Minigun).contains(&Stat::Magazine));
        assert!(!mutations(GunKind::Railgun).contains(&Mutation::Piercing));
        assert_eq!(mutations(GunKind::Rpg), vec![Mutation::Deadeye; 40]);
    }

    #[test]
    fn the_old_unversioned_trees_still_offer_everything() {
        let offered: std::collections::BTreeSet<usize> = (0..40)
            .flat_map(|rerolls| generate_unversioned(GunKind::M4, rerolls).nodes)
            .filter(|node| node.mutation.is_none())
            .map(|node| node.stat)
            .collect();
        assert_eq!(offered.len(), STAT_KINDS);
    }

    #[test]
    fn a_tree_is_deterministic_for_a_weapon_and_a_reroll() {
        for kind in GunKind::ALL {
            assert_eq!(generate(kind, 3), generate(kind, 3));
        }
    }

    #[test]
    fn no_two_nodes_share_a_cell_and_every_node_hangs_one_row_under_its_parent() {
        for tree in all() {
            for (index, node) in tree.nodes.iter().enumerate() {
                assert!(node.column.abs() <= HALF_COLUMNS && (1..=DEPTH).contains(&node.row));
                assert_eq!(tree.at(node.column, node.row), Some(index));
                match node.parent {
                    Some(parent) => assert_eq!(tree.nodes[parent].row, node.row - 1),
                    None => assert_eq!(node.row, 1),
                }
            }
        }
    }

    #[test]
    fn there_is_exactly_one_mutation_and_it_is_the_deepest_node() {
        for tree in all() {
            let mutations: Vec<&Node> = tree
                .nodes
                .iter()
                .filter(|node| node.mutation.is_some())
                .collect();

            assert_eq!(mutations.len(), 1);
            assert_eq!(mutations[0].row, DEPTH);
            assert!(tree.nodes.iter().all(|node| node.row <= DEPTH));
            assert!(tree.nodes.iter().filter(|node| node.row == DEPTH).count() == 1);
            assert!(mutations[0].mutation.is_some_and(|id| id < MUTATION_KINDS));
        }
    }

    #[test]
    fn the_mutation_is_reached_only_through_a_full_chain_of_upgrades() {
        for tree in all() {
            let Some(end) = tree.nodes.iter().position(|node| node.mutation.is_some()) else {
                panic!("no mutation");
            };
            let mut steps = 0;
            let mut at = tree.nodes[end].parent;
            while let Some(index) = at {
                steps += 1;
                at = tree.nodes[index].parent;
            }

            assert_eq!(steps, DEPTH - 1);
        }
    }

    #[test]
    fn no_two_links_cross_between_the_same_rows() {
        for tree in all() {
            for a in &tree.nodes {
                for b in &tree.nodes {
                    if a.row != b.row || a.parent == b.parent {
                        continue;
                    }
                    let (a_from, _) = tree.origin(a.parent);
                    let (b_from, _) = tree.origin(b.parent);
                    let crossed = (a_from < b_from && a.column > b.column)
                        || (a_from > b_from && a.column < b.column);

                    assert!(!crossed, "{tree:?}");
                }
            }
        }
    }

    #[test]
    fn a_bent_link_never_runs_into_a_node_that_is_not_its_own() {
        for tree in all() {
            for node in &tree.nodes {
                let (from, row) = tree.origin(node.parent);
                if from == node.column {
                    continue;
                }
                let above = tree.at(node.column, row);
                assert!(above.is_none() || above == node.parent);
                let below = tree.at(from, node.row);
                assert!(below.is_none_or(|index| tree.nodes[index].parent == node.parent));
            }
        }
    }

    #[test]
    fn rerolling_changes_the_shape_not_just_the_icons() {
        for kind in GunKind::ALL {
            let shapes: std::collections::HashSet<Vec<(i32, i32)>> = (0..40)
                .map(|rerolls| {
                    generate(kind, rerolls)
                        .nodes
                        .iter()
                        .map(|node| (node.column, node.row))
                        .collect()
                })
                .collect();

            assert!(shapes.len() >= 8, "{kind:?}: {}", shapes.len());
        }
    }

    #[test]
    fn branches_end_at_different_distances_from_the_mutation() {
        let mut depths = std::collections::HashSet::new();
        for tree in all() {
            for branch in 1..=SIDE_BRANCHES {
                if let Some(deepest) = tree
                    .nodes
                    .iter()
                    .filter(|node| node.branch == branch)
                    .map(|node| node.row)
                    .max()
                {
                    depths.insert(deepest);
                }
            }
        }

        assert!(depths.len() >= 3, "{depths:?}");
    }
}
