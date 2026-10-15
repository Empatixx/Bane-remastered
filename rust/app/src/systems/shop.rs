use glam::Vec2;
use hecs::{Entity, World};

use crate::artefacts::ArtefactKind;
use crate::assets::Textures;
use crate::assets::tex;
use crate::audio::{AudioCommand, SoundEffect};
use crate::components::{
    Animation, ArtefactDrop, ArtefactSlot, Candle, Clip, Currency, GunDrop, Health, Hidden,
    InRange, Interactable, Interaction, Light, Loadout, Pickup, Player, PriceBox, PriceDigits,
    PriceTag, Shop, Size, SpeechBubble, SpeechPart, SpeechText, Text, Wallet,
};
use crate::dungeon::{Dungeon, FixtureKind};
use crate::guns::GunKind;
use crate::items::{ItemKind, PotionSize, Vital};
use crate::prefabs;
use crate::systems::artefacts::{pick_up_artefact, unused_artefact};
use crate::systems::items::use_item;
use crate::systems::loadout::{pick_up_gun, unused_gun};

const SPEECH_SECONDS: f32 = 3.0;
const MAX_MULTIPLIER_ROLL: usize = 4;
const MAX_HEALTH_PRICE: usize = 3;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Stock {
    Defensive,
    Ammunition,
    Offensive,
}

impl Stock {
    fn multiplier(self) -> f32 {
        match self {
            Self::Offensive => 2.0,
            Self::Ammunition => 1.0,
            Self::Defensive => 1.5,
        }
    }
}

pub(crate) const TABLE_STOCK: [Stock; 3] = [Stock::Defensive, Stock::Ammunition, Stock::Offensive];

pub(crate) fn price_for(stock: Stock, floor: u32, roll: usize) -> i32 {
    ((1 + roll % MAX_MULTIPLIER_ROLL) as f32 * floor as f32 * stock.multiplier()) as i32
}

enum Goods {
    Item(ItemKind),
    Gun(GunKind),
    Artefact(ArtefactKind),
}

fn goods_for(world: &World, stock: Stock, roll: &mut impl FnMut(usize) -> usize) -> Option<Goods> {
    match stock {
        Stock::Defensive => {
            let vital = if roll(2) == 0 {
                Vital::Health
            } else {
                Vital::Armor
            };
            Some(Goods::Item(ItemKind::Potion(vital, PotionSize::Large)))
        }
        Stock::Ammunition => Some(Goods::Item(ItemKind::Ammo(
            crate::items::AmmoBox::Universal,
        ))),
        Stock::Offensive => {
            if roll(2) == 0 {
                let gun = world
                    .query::<&Loadout>()
                    .with::<&Player>()
                    .iter()
                    .next()
                    .and_then(|loadout| unused_gun(loadout, roll(1000) as f32 / 1000.0));
                if let Some(kind) = gun {
                    return Some(Goods::Gun(kind));
                }
            }
            let slot = world
                .query::<&ArtefactSlot>()
                .with::<&Player>()
                .iter()
                .next()
                .copied()?;
            Some(Goods::Artefact(unused_artefact(&slot, roll(1000))))
        }
    }
}

fn place(
    world: &mut World,
    goods: Goods,
    center: Vec2,
    shop: Shop,
    table_bottom: f32,
    textures: &Textures,
) -> Entity {
    let (entity, height) = match goods {
        Goods::Item(kind) => (
            prefabs::item(world, kind, center, false, Vec2::ZERO, textures),
            kind.spec().size.y,
        ),
        Goods::Gun(kind) => (
            {
                let weapon = crate::systems::loadout::fresh_weapon(world, kind);
                prefabs::gun_drop(world, weapon, center, Vec2::ZERO, textures)
            },
            kind.spec().icon.y,
        ),
        Goods::Artefact(kind) => (
            prefabs::artefact_drop(world, kind, center, Vec2::ZERO, textures),
            kind.icon().y,
        ),
    };
    let gap = table_bottom - (center.y + height / 2.0) + PROMPT_GAP;
    prefabs::for_sale(world, entity, shop, gap);
    // On display, the goods stand just in front of their table.
    let _ = world.insert_one(
        entity,
        crate::components::YSort {
            foot: table_bottom - center.y + DISPLAY_NUDGE,
        },
    );
    entity
}

const PROMPT_GAP: f32 = 20.0;
const DISPLAY_NUDGE: f32 = 0.5;
const ITEM_LIFT: f32 = 15.0;

/// The share of the price still paid on a sale, from a roll in `0..100`.
pub(crate) fn sale_for(chance: usize) -> Option<f32> {
    match chance {
        0..=4 => Some(0.25),
        5..=9 => Some(0.5),
        10..=32 => Some(0.75),
        _ => None,
    }
}

pub(crate) fn stock_rooms(
    world: &mut World,
    dungeon: &Dungeon,
    floor: u32,
    discount: bool,
    textures: &Textures,
    mut roll: impl FnMut(usize) -> usize,
) {
    for room in &dungeon.rooms {
        let Some(furnishing) = &room.furnishing else {
            continue;
        };
        let mut tables: Vec<(Vec2, Vec2)> = furnishing
            .fixtures
            .iter()
            .filter(|fixture| fixture.kind == FixtureKind::Table)
            .map(|fixture| (fixture.top_left, fixture.size))
            .collect();
        tables.sort_by(|a, b| a.0.x.total_cmp(&b.0.x));
        let sale = if discount { sale_for(roll(100)) } else { None };
        let sale_slot = roll(TABLE_STOCK.len());
        for (index, ((top_left, size), stock)) in tables.into_iter().zip(TABLE_STOCK).enumerate() {
            let center = top_left + size / 2.0 - Vec2::new(0.0, ITEM_LIFT);
            if let Some(goods) = goods_for(world, stock, &mut roll) {
                let mut shop = Shop {
                    price: price_for(stock, floor, roll(MAX_MULTIPLIER_ROLL)),
                    currency: Currency::Coins,
                };
                let mut sale_percent = None;
                if let (Some(factor), true) = (sale, index == sale_slot) {
                    shop.price = ((shop.price as f32 * factor) as i32).max(1);
                    sale_percent = Some(((1.0 - factor) * 100.0).round() as u32);
                }
                let entity = place(world, goods, center, shop, top_left.y + size.y, textures);
                prefabs::price_tag(
                    world,
                    entity,
                    tex::SHOP_PRICE_HUD,
                    Vec2::new(center.x, top_left.y),
                    shop.price,
                    sale_percent,
                    textures,
                );
            }
        }
        if let Some(pentagram) = furnishing
            .fixtures
            .iter()
            .find(|fixture| fixture.kind == FixtureKind::Pentagram)
        {
            let stock = [Stock::Offensive, Stock::Ammunition, Stock::Defensive][roll(3)];
            if let Some(goods) = goods_for(world, stock, &mut roll) {
                let shop = Shop {
                    price: 1 + roll(MAX_HEALTH_PRICE) as i32,
                    currency: Currency::Health,
                };
                let center = pentagram.top_left + pentagram.size / 2.0 - Vec2::new(0.0, ITEM_LIFT);
                let bottom = pentagram.top_left.y + pentagram.size.y;
                let entity = place(world, goods, center, shop, bottom, textures);
                prefabs::price_tag(
                    world,
                    entity,
                    tex::SACRIFICE_PRICE_HUD,
                    Vec2::new(center.x, pentagram.top_left.y),
                    shop.price,
                    None,
                    textures,
                );
            }
        }
    }
}

pub(crate) fn buy(world: &mut World, textures: &Textures, sounds: &mut Vec<AudioCommand>) {
    let offer = world
        .query::<(Entity, &Shop)>()
        .with::<&InRange>()
        .iter()
        .next()
        .map(|(entity, shop)| (entity, *shop));
    let Some((entity, shop)) = offer else {
        return;
    };
    let paid = match shop.currency {
        Currency::Coins => world
            .query_mut::<&mut Wallet>()
            .with::<&Player>()
            .into_iter()
            .next()
            .is_some_and(|wallet| {
                let enough = wallet.coins >= shop.price;
                if enough {
                    wallet.coins -= shop.price;
                }
                enough
            }),
        Currency::Health => world
            .query_mut::<&mut Health>()
            .with::<&Player>()
            .into_iter()
            .next()
            .is_some_and(|health| {
                let enough = health.current > shop.price;
                if enough {
                    health.current -= shop.price;
                }
                enough
            }),
    };
    if !paid {
        return;
    }
    let _ = world.remove_one::<Shop>(entity);
    let _ = world.remove_one::<crate::components::Prompt>(entity);
    let _ = world.remove_one::<Interactable>(entity);
    match shop.currency {
        Currency::Coins => {
            sounds.push(AudioCommand::Effect(SoundEffect::Buy));
            say(world, "Enjoy your purchase!");
        }
        Currency::Health => {
            sounds.push(AudioCommand::Effect(SoundEffect::Sacrifice));
            sounds.push(AudioCommand::Effect(SoundEffect::PlayerHurt));
            disable_candles(world);
        }
    }
    let item = world.get::<&Pickup>(entity).map(|pickup| pickup.0).ok();
    if let Some(item) = item {
        if crate::systems::items::can_use(world, item) {
            use_item(world, item, sounds);
            let _ = world.despawn(entity);
        }
    } else if world.get::<&GunDrop>(entity).is_ok() {
        let _ = world.insert_one(
            entity,
            Interactable {
                action: Interaction::PickUpGun,
            },
        );
        pick_up_gun(world, textures, sounds);
    } else if world.get::<&ArtefactDrop>(entity).is_ok() {
        let _ = world.insert_one(
            entity,
            Interactable {
                action: Interaction::PickUpArtefact,
            },
        );
        pick_up_artefact(world, textures, sounds);
    }
}

pub(crate) fn update_price_tags(world: &mut World) {
    let orphaned: Vec<Entity> = world
        .query::<(Entity, &PriceTag)>()
        .iter()
        .filter(|(_, tag)| world.get::<&Shop>(tag.owner).is_err())
        .map(|(entity, _)| entity)
        .collect();
    for entity in orphaned {
        let _ = world.despawn(entity);
    }
}

pub(crate) fn disable_candles(world: &mut World) {
    let candles: Vec<Entity> = world.query::<Entity>().with::<&Candle>().iter().collect();
    for candle in candles {
        let _ = world.remove_one::<Light>(candle);
        if let Ok(mut animation) = world.get::<&mut Animation>(candle) {
            animation.play(Clip {
                row: 1,
                frames: 1,
                frame_time: 1.0,
                looping: true,
                flipped: false,
            });
        }
    }
}

const PRICE_BOX_MIN: f32 = 32.0;
const PRICE_BOX_PAD: f32 = 14.0;

pub(crate) fn fit_price_boxes(world: &mut World, fonts: &crate::render::Fonts) {
    let font = fonts.get(crate::render::FontSize::Size22);
    let widths: Vec<(Entity, f32)> = world
        .query::<(&PriceDigits, &Text)>()
        .iter()
        .map(|(digits, text)| (digits.0, font.width(&text.content)))
        .collect();
    for (price_box, size) in world.query_mut::<(&PriceBox, &mut Size)>() {
        if let Some((_, width)) = widths.iter().find(|(owner, _)| *owner == price_box.0) {
            size.0.x = (width + PRICE_BOX_PAD).max(PRICE_BOX_MIN);
        }
    }
}

pub(crate) fn say(world: &mut World, message: &'static str) {
    for bubble in world.query_mut::<&mut SpeechBubble>() {
        bubble.remaining = SPEECH_SECONDS;
        bubble.message = message;
    }
}

pub(crate) fn tick_speech(world: &mut World, dt: f32) {
    let mut visible = false;
    let mut message = "";
    let mut bubbles = Vec::new();
    for (entity, bubble) in world.query_mut::<(Entity, &mut SpeechBubble)>() {
        bubble.remaining = (bubble.remaining - dt).max(0.0);
        visible = bubble.remaining > 0.0;
        message = bubble.message;
        bubbles.push(entity);
    }
    for entity in bubbles {
        if visible {
            let _ = world.remove_one::<Hidden>(entity);
        } else {
            let _ = world.insert_one(entity, Hidden);
        }
    }
    let parts: Vec<Entity> = world
        .query::<Entity>()
        .with::<&SpeechPart>()
        .iter()
        .collect();
    for entity in parts {
        if visible {
            let _ = world.remove_one::<Hidden>(entity);
        } else {
            let _ = world.insert_one(entity, Hidden);
        }
    }
    let texts: Vec<Entity> = world
        .query::<Entity>()
        .with::<&SpeechText>()
        .iter()
        .collect();
    for entity in texts {
        if let Ok(mut text) = world.get::<&mut Text>(entity) {
            text.content = message.to_owned();
        }
        if visible {
            let _ = world.remove_one::<Hidden>(entity);
        } else {
            let _ = world.insert_one(entity, Hidden);
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::components::Armor;
    use crate::physics::Physics;

    use super::*;

    fn setup() -> World {
        let mut world = World::new();
        let mut physics = Physics::new();
        prefabs::player(
            &mut world,
            &mut physics,
            Vec2::new(100.0, 100.0),
            &Textures::default(),
        );
        world
    }

    fn offer(world: &mut World, shop: Shop, item: ItemKind) -> Entity {
        let textures = Textures::default();
        let entity = prefabs::item(
            world,
            item,
            Vec2::new(100.0, 100.0),
            false,
            Vec2::ZERO,
            &textures,
        );
        prefabs::for_sale(world, entity, shop, 40.0);
        let _ = world.insert_one(entity, InRange);
        entity
    }

    fn coins(world: &World) -> i32 {
        world
            .query::<&Wallet>()
            .with::<&Player>()
            .iter()
            .next()
            .map_or(-1, |w| w.coins)
    }

    #[test]
    fn prices_scale_with_the_floor_and_the_kind_of_goods() {
        assert_eq!(price_for(Stock::Ammunition, 1, 0), 1);
        assert_eq!(price_for(Stock::Offensive, 1, 3), 8);
        assert_eq!(price_for(Stock::Defensive, 2, 1), 6);
        assert_eq!(price_for(Stock::Defensive, 4, 3), 24);
    }

    #[test]
    fn buying_without_enough_coins_changes_nothing() {
        let mut world = setup();
        let shop = Shop {
            price: 5,
            currency: Currency::Coins,
        };
        let entity = offer(
            &mut world,
            shop,
            ItemKind::Ammo(crate::items::AmmoBox::Universal),
        );
        let mut sounds = Vec::new();

        buy(&mut world, &Textures::default(), &mut sounds);

        assert!(world.get::<&Shop>(entity).is_ok());
        assert!(sounds.is_empty());
    }

    #[test]
    fn buying_pays_removes_the_price_and_applies_a_usable_item_at_once() {
        let mut world = setup();
        for wallet in world.query_mut::<&mut Wallet>().with::<&Player>() {
            wallet.coins = 10;
        }
        for loadout in world.query_mut::<&mut Loadout>().with::<&Player>() {
            if let Some(weapon) = loadout.current_mut() {
                weapon.ammo = 0;
            }
        }
        let shop = Shop {
            price: 4,
            currency: Currency::Coins,
        };
        let entity = offer(
            &mut world,
            shop,
            ItemKind::Ammo(crate::items::AmmoBox::Universal),
        );
        let tag = world.spawn((PriceTag { owner: entity },));

        buy(&mut world, &Textures::default(), &mut Vec::new());
        update_price_tags(&mut world);

        assert_eq!(coins(&world), 6);
        assert!(!world.contains(entity));
        assert!(!world.contains(tag));
        let ammo = world
            .query::<&Loadout>()
            .with::<&Player>()
            .iter()
            .next()
            .and_then(|l| l.current().map(|w| w.ammo));
        assert_eq!(ammo, Some(60));
    }

    #[test]
    fn a_bought_potion_you_cannot_use_yet_stays_as_a_normal_pickup() {
        let mut world = setup();
        for wallet in world.query_mut::<&mut Wallet>().with::<&Player>() {
            wallet.coins = 10;
        }
        let shop = Shop {
            price: 2,
            currency: Currency::Coins,
        };
        let potion = ItemKind::Potion(Vital::Health, PotionSize::Large);
        let entity = offer(&mut world, shop, potion);

        buy(&mut world, &Textures::default(), &mut Vec::new());

        assert_eq!(coins(&world), 8);
        assert!(world.contains(entity));
        assert!(world.get::<&Shop>(entity).is_err());
        assert!(world.get::<&Pickup>(entity).is_ok());
    }

    #[test]
    fn a_bought_gun_goes_straight_into_the_hand() {
        let mut world = setup();
        for wallet in world.query_mut::<&mut Wallet>().with::<&Player>() {
            wallet.coins = 10;
        }
        let textures = Textures::default();
        let gun = prefabs::gun_drop(
            &mut world,
            crate::components::Weapon::new(GunKind::Revolver),
            Vec2::new(100.0, 100.0),
            Vec2::ZERO,
            &textures,
        );
        prefabs::for_sale(
            &mut world,
            gun,
            Shop {
                price: 3,
                currency: Currency::Coins,
            },
            30.0,
        );
        let _ = world.insert_one(gun, InRange);

        buy(&mut world, &textures, &mut Vec::new());

        let held = world
            .query::<&Loadout>()
            .with::<&Player>()
            .iter()
            .next()
            .and_then(|l| l.current().map(|w| w.kind));
        assert_eq!(held, Some(GunKind::Revolver));
    }

    #[test]
    fn a_sacrifice_costs_health_and_snuffs_the_candles() {
        let mut world = setup();
        let textures = Textures::default();
        let candle = prefabs::prop(
            &mut world,
            crate::props::Prop::Candlestick,
            Vec2::ZERO,
            crate::props::Look::DEFAULT,
            &textures,
        );
        let _ = world.insert_one(candle, Candle);
        let shop = Shop {
            price: 2,
            currency: Currency::Health,
        };
        let potion = ItemKind::Potion(Vital::Armor, PotionSize::Large);
        let entity = offer(&mut world, shop, potion);
        for armor in world.query_mut::<&mut Armor>().with::<&Player>() {
            armor.current = 0;
        }
        let before = world
            .query::<&Health>()
            .with::<&Player>()
            .iter()
            .next()
            .map(|h| h.current);

        buy(&mut world, &textures, &mut Vec::new());

        let after = world
            .query::<&Health>()
            .with::<&Player>()
            .iter()
            .next()
            .map(|h| h.current);
        assert_eq!(before.zip(after).map(|(b, a)| b - a), Some(2));
        assert!(world.get::<&Light>(candle).is_err());
        assert!(!world.contains(entity));
    }

    #[test]
    fn you_cannot_sacrifice_more_health_than_you_have() {
        let mut world = setup();
        for health in world.query_mut::<&mut Health>().with::<&Player>() {
            health.current = 1;
        }
        let shop = Shop {
            price: 2,
            currency: Currency::Health,
        };
        let entity = offer(&mut world, shop, ItemKind::Coin);

        buy(&mut world, &Textures::default(), &mut Vec::new());

        assert!(world.get::<&Shop>(entity).is_ok());
    }

    #[test]
    fn paying_with_health_never_kills_the_player() {
        let mut world = setup();
        for health in world.query_mut::<&mut Health>().with::<&Player>() {
            health.current = 2;
        }
        let shop = Shop {
            price: 2,
            currency: Currency::Health,
        };
        let entity = offer(&mut world, shop, ItemKind::Coin);

        buy(&mut world, &Textures::default(), &mut Vec::new());

        assert!(world.get::<&Shop>(entity).is_ok());
    }

    #[test]
    fn the_shopkeeper_speaks_for_three_seconds() {
        let mut world = World::new();
        prefabs::minimal_speech(&mut world, Vec2::new(500.0, 100.0));
        tick_speech(&mut world, 0.016);
        assert_eq!(
            world
                .query::<&Hidden>()
                .with::<&SpeechBubble>()
                .iter()
                .count(),
            1
        );

        say(&mut world, "Hello");
        tick_speech(&mut world, 0.016);
        assert_eq!(
            world
                .query::<&Hidden>()
                .with::<&SpeechBubble>()
                .iter()
                .count(),
            0
        );
        let said = world
            .query::<&Text>()
            .with::<&SpeechText>()
            .iter()
            .next()
            .map(|t| t.content.clone());
        assert_eq!(said.as_deref(), Some("Hello"));

        tick_speech(&mut world, 3.1);
        assert_eq!(
            world
                .query::<&Hidden>()
                .with::<&SpeechBubble>()
                .iter()
                .count(),
            1
        );
    }

    #[test]
    fn price_boxes_grow_with_long_prices_but_never_shrink_below_the_base_size() {
        use crate::render::Fonts;

        let mut world = World::new();
        let owner = world.spawn(());
        let boxed = world.spawn((PriceBox(owner), Size(Vec2::splat(32.0))));
        world.spawn((
            PriceDigits(owner),
            Text::inked("5", glam::Vec4::ONE, crate::render::FontSize::Size22),
        ));

        fit_price_boxes(&mut world, &Fonts::default());

        let width = world
            .get::<&Size>(boxed)
            .map(|size| size.0.x)
            .unwrap_or(0.0);
        assert!(width >= PRICE_BOX_MIN, "{width}");
    }
}
