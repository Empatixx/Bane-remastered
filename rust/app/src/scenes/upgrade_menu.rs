use glam::{Vec2, Vec4};
use hecs::{Entity, World};

use crate::assets::Assets;
use crate::audio::{AudioCommand, SoundEffect};
use crate::components::{
    Color, Layer, Overlay, Position, ScreenSpace, Size, Sprite, Text, UpgradeUi,
};
use crate::input::Input;
use crate::profile::SharedProfile;
use crate::render::{Align, Anchor, FontSize};
use crate::upgrades::{Purchase, Quality, Tree, Upgrade, TREE_STEP, TREE_TOP};

const TALENT: f32 = 64.0;
const ICON: f32 = 32.0;
const TREE_ICON: f32 = 64.0;
const TREE_ICON_LIFT: f32 = 75.0;
const PATH_SIZE: Vec2 = Vec2::new(16.0, 30.0);
const PATH_OFFSET: Vec2 = Vec2::new(24.0, -20.0);
const PANEL_TOP_LEFT: Vec2 = Vec2::new(675.0, 150.0);
const PANEL_SIZE: Vec2 = Vec2::new(240.0, 300.0);
const PANEL_TALENT_Y: f32 = 90.0;
const TITLE_BASELINE: f32 = 222.0;
const DESCRIPTION_TOP: f32 = 266.0;
const DESCRIPTION_STEP: f32 = 22.0;
const BUY_CENTER_X: f32 = 735.0;
const STATUS_CENTER_X: f32 = 750.0;
const BUTTON_TOP: f32 = 398.0;
const BUTTON_HEIGHT: f32 = 36.0;
const BUTTON_PADDING: f32 = 10.0;
const BUTTON_BASELINE: f32 = 424.0;
const HOVER_COLOR: Vec4 = Vec4::new(1.0, 1.0, 0.0, 1.0);
const PRICE_RIGHT_X: f32 = 886.0;
const MENU_COLOR: Vec4 = Vec4::new(222.0 / 255.0, 112.0 / 255.0, 38.0 / 255.0, 1.0);
const BLOCKED_COLOR: Vec4 = Vec4::new(0.5, 0.5, 0.5, 1.0);
const BASE_LAYER: i32 = 30;

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum Element {
    Image {
        texture: &'static str,
        top_left: Vec2,
        size: Vec2,
        cell: Option<(usize, usize)>,
        layer: i32,
    },
    Label {
        content: String,
        position: Vec2,
        color: Vec4,
        size: FontSize,
        align: Align,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum BuyState {
    Available,
    Owned,
    Locked,
}

pub(crate) fn frame_row(quality: Quality, owned: bool) -> usize {
    let base = match quality {
        Quality::Common => 0,
        Quality::Rare => 2,
        Quality::Epic => 4,
    };
    if owned {
        base
    } else {
        base + 1
    }
}

pub(crate) fn path_row(tree: Tree, locked: bool) -> usize {
    if locked {
        return 3;
    }
    match tree {
        Tree::Utility => 0,
        Tree::Offensive => 1,
        Tree::Defensive => 2,
    }
}

pub(crate) fn buy_state(profile: &SharedProfile, upgrade: Upgrade) -> BuyState {
    let snapshot = profile.snapshot();
    if snapshot.owns(upgrade) {
        BuyState::Owned
    } else if snapshot.can_buy(upgrade) {
        BuyState::Available
    } else {
        BuyState::Locked
    }
}

fn talent(upgrade: Upgrade, top_left: Vec2, owned: bool, layer: i32) -> [Element; 2] {
    let def = upgrade.def();
    [
        Element::Image {
            texture: "ingame/talent_frame.tga",
            top_left,
            size: Vec2::splat(TALENT),
            cell: Some((frame_row(def.quality, owned), 6)),
            layer,
        },
        Element::Image {
            texture: def.texture,
            top_left: top_left + Vec2::splat(TALENT / 4.0),
            size: Vec2::splat(ICON),
            cell: None,
            layer: layer + 1,
        },
    ]
}

pub(crate) fn layout(
    profile: &SharedProfile,
    selected: Option<Upgrade>,
    hover: bool,
) -> Vec<Element> {
    let snapshot = profile.snapshot();
    let mut elements = Vec::new();
    for tree in Tree::ALL {
        let column = tree.column();
        elements.push(Element::Image {
            texture: tree.icon(),
            top_left: Vec2::new(column, TREE_TOP - TREE_ICON_LIFT),
            size: Vec2::splat(TREE_ICON),
            cell: None,
            layer: BASE_LAYER,
        });
        for (index, upgrade) in tree.upgrades().into_iter().enumerate() {
            let top_left = Vec2::new(upgrade.top_left().0, upgrade.top_left().1);
            elements.extend(talent(
                upgrade,
                top_left,
                snapshot.owns(upgrade),
                BASE_LAYER,
            ));
            if index > 0 {
                elements.push(Element::Image {
                    texture: "ingame/talent_path.tga",
                    top_left: Vec2::new(column, TREE_TOP + TREE_STEP * index as f32)
                        + PATH_OFFSET
                        + Vec2::new(0.0, 0.0),
                    size: PATH_SIZE,
                    cell: Some((path_row(tree, !snapshot.owns(upgrade)), 4)),
                    layer: BASE_LAYER,
                });
            }
        }
    }
    if let Some(upgrade) = selected {
        elements.extend(panel(profile, upgrade, hover));
    }
    elements
}

fn panel(profile: &SharedProfile, upgrade: Upgrade, hover: bool) -> Vec<Element> {
    let def = upgrade.def();
    let snapshot = profile.snapshot();
    let centre_x = PANEL_TOP_LEFT.x + PANEL_SIZE.x / 2.0;
    let mut elements = vec![Element::Image {
        texture: "ingame/pausebg.tga",
        top_left: PANEL_TOP_LEFT,
        size: PANEL_SIZE,
        cell: None,
        layer: BASE_LAYER,
    }];
    elements.extend(talent(
        upgrade,
        Vec2::new(centre_x - TALENT / 2.0, PANEL_TALENT_Y),
        snapshot.owns(upgrade),
        BASE_LAYER + 2,
    ));
    elements.push(Element::Label {
        content: def.title.to_owned(),
        position: Vec2::new(centre_x, TITLE_BASELINE),
        color: MENU_COLOR,
        size: FontSize::Size22,
        align: Align::Center,
    });
    for (index, line) in def.description.iter().enumerate() {
        elements.push(Element::Label {
            content: line.trim().to_owned(),
            position: Vec2::new(centre_x, DESCRIPTION_TOP + DESCRIPTION_STEP * index as f32),
            color: MENU_COLOR,
            size: FontSize::Size16,
            align: Align::Center,
        });
    }
    let (label, color, center_x) = match buy_state(profile, upgrade) {
        BuyState::Available => (
            "Buy",
            if hover { HOVER_COLOR } else { MENU_COLOR },
            BUY_CENTER_X,
        ),
        BuyState::Owned => ("Bought", BLOCKED_COLOR, STATUS_CENTER_X),
        BuyState::Locked => ("Can't buy", BLOCKED_COLOR, STATUS_CENTER_X),
    };
    elements.push(Element::Label {
        content: label.to_owned(),
        position: Vec2::new(center_x, BUTTON_BASELINE),
        color,
        size: FontSize::Size22,
        align: Align::Center,
    });
    if buy_state(profile, upgrade) == BuyState::Available {
        elements.push(Element::Label {
            content: format!("{},-", def.price),
            position: Vec2::new(PRICE_RIGHT_X, BUTTON_BASELINE),
            color: MENU_COLOR,
            size: FontSize::Size22,
            align: Align::Right,
        });
    }
    elements
}

pub(crate) fn talent_at(point: Vec2) -> Option<Upgrade> {
    Upgrade::ALL.into_iter().find(|upgrade| {
        let (x, y) = upgrade.top_left();
        point.x >= x && point.x <= x + TALENT && point.y >= y && point.y <= y + TALENT
    })
}

pub(crate) fn on_buy_button(point: Vec2, text_width: f32) -> bool {
    let half = text_width / 2.0 + BUTTON_PADDING;
    point.x >= BUY_CENTER_X - half
        && point.x <= BUY_CENTER_X + half
        && point.y >= BUTTON_TOP
        && point.y <= BUTTON_TOP + BUTTON_HEIGHT
}

#[derive(Debug, Default)]
pub(crate) struct UpgradeMenu {
    open: bool,
    selected: Option<Upgrade>,
    hover: bool,
}

impl UpgradeMenu {
    pub(crate) fn is_open(&self) -> bool {
        self.open
    }

    pub(crate) fn toggle(&mut self, world: &mut World, assets: &Assets, profile: &SharedProfile) {
        if self.open {
            self.close(world);
        } else {
            self.open = true;
            self.selected = None;
            self.hover = false;
            self.rebuild(world, assets, profile);
        }
    }

    pub(crate) fn close(&mut self, world: &mut World) {
        self.open = false;
        self.selected = None;
        clear(world);
    }

    pub(crate) fn update(
        &mut self,
        world: &mut World,
        input: &Input,
        assets: &Assets,
        profile: &SharedProfile,
        sounds: &mut Vec<AudioCommand>,
    ) -> Option<Purchase> {
        if !self.open {
            return None;
        }
        let point = input.mouse();
        let width = assets.fonts.get(FontSize::Size22).width("Buy");
        let hover = self
            .selected
            .is_some_and(|selected| buy_state(profile, selected) == BuyState::Available)
            && on_buy_button(point, width);
        if hover != self.hover {
            self.hover = hover;
            self.rebuild(world, assets, profile);
        }
        if !input.clicked() {
            return None;
        }
        let mut outcome = None;
        if let (Some(selected), true) = (self.selected, on_buy_button(point, width)) {
            if buy_state(profile, selected) == BuyState::Available {
                let result = profile.buy(selected);
                if result == Purchase::Bought {
                    sounds.push(AudioCommand::Effect(SoundEffect::TalentUnlock));
                }
                outcome = Some(result);
            }
        } else {
            self.selected = talent_at(point);
        }
        self.rebuild(world, assets, profile);
        outcome
    }

    fn rebuild(&self, world: &mut World, assets: &Assets, profile: &SharedProfile) {
        clear(world);
        for element in layout(profile, self.selected, self.hover) {
            spawn(world, element, assets);
        }
    }
}

fn clear(world: &mut World) {
    let ui: Vec<Entity> = world
        .query::<Entity>()
        .with::<&UpgradeUi>()
        .iter()
        .collect();
    for entity in ui {
        let _ = world.despawn(entity);
    }
}

fn spawn(world: &mut World, element: Element, assets: &Assets) {
    match element {
        Element::Image {
            texture,
            top_left,
            size,
            cell,
            layer,
        } => {
            let id = assets.textures.get(texture);
            let sprite = match cell {
                Some((row, rows)) => Sprite::cell(id, 0, row, 1, rows),
                None => Sprite::whole(id),
            };
            world.spawn((
                UpgradeUi,
                Position(top_left + size / 2.0),
                Size(size),
                Color(Vec4::ONE),
                sprite,
                Layer(layer),
                Overlay,
                ScreenSpace,
            ));
        }
        Element::Label {
            content,
            position,
            color,
            size,
            align,
        } => {
            world.spawn((
                UpgradeUi,
                Position(position),
                Text {
                    content,
                    color,
                    size,
                    align,
                    anchor: Anchor::Baseline,
                },
                Layer(BASE_LAYER + 5),
                Overlay,
                ScreenSpace,
            ));
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::components::RunStats;
    use crate::systems::summarize;

    use super::*;

    fn rich_profile() -> SharedProfile {
        let profile = SharedProfile::in_memory();
        let stats = RunStats {
            fired: 500,
            hits: 400,
            kills: 300,
            items: 200,
        };
        profile.record_run(stats, &summarize(stats, 9), true);
        profile
    }

    fn images(elements: &[Element], texture: &str) -> Vec<(Vec2, Option<(usize, usize)>)> {
        elements
            .iter()
            .filter_map(|element| match element {
                Element::Image {
                    texture: found,
                    top_left,
                    cell,
                    ..
                } if *found == texture => Some((*top_left, *cell)),
                _ => None,
            })
            .collect()
    }

    #[test]
    fn the_menu_shows_three_trees_of_five_talents_with_four_paths_each() {
        let profile = SharedProfile::in_memory();
        let elements = layout(&profile, None, false);
        assert_eq!(images(&elements, "ingame/talent_frame.tga").len(), 15);
        assert_eq!(images(&elements, "ingame/talent_path.tga").len(), 12);
        assert_eq!(images(&elements, "ingame/utility_icon.tga").len(), 1);
        assert_eq!(images(&elements, "ingame/offense_icon.tga").len(), 1);
        assert_eq!(images(&elements, "ingame/defense_icon.tga").len(), 1);
    }

    #[test]
    fn frames_show_quality_and_whether_the_talent_is_owned() {
        assert_eq!(frame_row(Quality::Common, true), 0);
        assert_eq!(frame_row(Quality::Common, false), 1);
        assert_eq!(frame_row(Quality::Rare, true), 2);
        assert_eq!(frame_row(Quality::Rare, false), 3);
        assert_eq!(frame_row(Quality::Epic, true), 4);
        assert_eq!(frame_row(Quality::Epic, false), 5);
    }

    #[test]
    fn locked_paths_are_grey_and_open_ones_take_the_colour_of_their_tree() {
        assert_eq!(path_row(Tree::Utility, true), 3);
        assert_eq!(path_row(Tree::Utility, false), 0);
        assert_eq!(path_row(Tree::Offensive, false), 1);
        assert_eq!(path_row(Tree::Defensive, false), 2);
    }

    #[test]
    fn clicking_a_talent_finds_it_and_empty_space_finds_nothing() {
        assert_eq!(
            talent_at(Vec2::new(380.0, 140.0)),
            Some(Upgrade::MovementSpeed)
        );
        assert_eq!(
            talent_at(Vec2::new(480.0, 215.0)),
            Some(Upgrade::AdditionalAmmo)
        );
        assert_eq!(talent_at(Vec2::new(10.0, 10.0)), None);
        assert_eq!(talent_at(Vec2::new(430.0, 140.0)), None);
    }

    #[test]
    fn the_details_panel_names_the_talent_and_its_price_and_state() {
        let profile = rich_profile();
        let elements = layout(&profile, Some(Upgrade::Discount), false);
        let labels: Vec<String> = elements
            .iter()
            .filter_map(|element| match element {
                Element::Label { content, .. } => Some(content.clone()),
                _ => None,
            })
            .collect();
        assert!(labels.contains(&"Discount".to_owned()));
        assert!(labels.contains(&"Buy".to_owned()));
        assert!(labels.contains(&"60,-".to_owned()));

        let owned = layout(&profile, Some(Upgrade::MovementSpeed), false);
        assert!(owned
            .iter()
            .any(|e| matches!(e, Element::Label { content, .. } if content == "Bought")));

        let locked = layout(&profile, Some(Upgrade::TrapMaster), false);
        assert!(locked
            .iter()
            .any(|e| matches!(e, Element::Label { content, .. } if content == "Can't buy")));
        assert!(!locked
            .iter()
            .any(|e| matches!(e, Element::Label { content, .. } if content.ends_with(",-"))));
    }

    #[test]
    fn without_a_selection_there_is_no_details_panel() {
        let profile = SharedProfile::in_memory();
        let elements = layout(&profile, None, false);
        assert!(images(&elements, "ingame/pausebg.tga").is_empty());
    }

    #[test]
    fn the_buy_button_sits_where_bane_puts_it() {
        assert!(on_buy_button(Vec2::new(735.0, 420.0), 40.0));
        assert!(on_buy_button(Vec2::new(760.0, 420.0), 40.0));
        assert!(!on_buy_button(Vec2::new(700.0, 420.0), 40.0));
        assert!(!on_buy_button(Vec2::new(735.0, 380.0), 40.0));
    }

    #[test]
    fn buy_state_follows_ownership_and_the_parent_chain() {
        let profile = rich_profile();
        assert_eq!(buy_state(&profile, Upgrade::MovementSpeed), BuyState::Owned);
        assert_eq!(buy_state(&profile, Upgrade::Discount), BuyState::Available);
        assert_eq!(buy_state(&profile, Upgrade::TrapMaster), BuyState::Locked);
        assert_eq!(profile.buy(Upgrade::Discount), Purchase::Bought);
        assert_eq!(
            buy_state(&profile, Upgrade::TrapMaster),
            BuyState::Available
        );
    }
}
