use crate::assets::{tex, Tex};
use glam::{Vec2, Vec4};
use hecs::{Entity, World};

use crate::assets::Textures;
use crate::components::{
    AmmoPip, AmmoText, Animation, ArtefactIcon, BarKind, BossHud, CoinText, Color, Despawn,
    GunIcon, Hidden, Interactable, Interaction, Layer, Lit, Position, PriceTag, Prompt,
    ScreenSpace, Shop, Size, Sprite, StatusBar, Text, PIP_FULL,
};
use crate::guns::{MAX_CLIP, PIP_SIZE};
use crate::render::{Align, Anchor, FontSize, TextureId};

use super::{PRICE_COLOR, PRICE_TAG_SIZE, PROMPT_COLOR};

const PRICE_TAG_LIFT: f32 = 15.0;

const SALE_COLOR: Vec4 = Vec4::new(122.0 / 255.0, 248.0 / 255.0, 18.0 / 255.0, 1.0);
const SALE_LIFT: f32 = 14.0;

const ARTEFACT_SLOT_POSITION: Vec2 = Vec2::new(585.0, 450.0);
const ARTEFACT_FRAME: f32 = 75.0;

pub(crate) fn boss_hud(world: &mut World, name: &str, textures: &Textures) {
    let kind = BarKind::Boss;
    let spec = kind.spec();
    let size = spec_texture_size(kind) * spec.scale;
    world.spawn((
        BossHud,
        StatusBar {
            kind,
            fill: 1.0,
            delayed: 1.0,
        },
        Position(spec.position + size / 2.0),
        Size(size),
        Color(Vec4::ONE),
        Sprite::whole(textures.get(spec.texture)),
        Layer(5),
        ScreenSpace,
    ));
    world.spawn((
        BossHud,
        Position(Vec2::new(
            spec.position.x + spec.hole[0] as f32 * spec.scale + 5.0,
            spec.position.y + 15.0,
        )),
        Text::new(
            name.to_owned(),
            Vec4::new(1.0, 0.0, 0.0, 1.0),
            FontSize::Size22,
        ),
        Layer(6),
        ScreenSpace,
    ));
}

pub(crate) fn for_sale(world: &mut World, entity: Entity, shop: Shop, prompt_gap: f32) {
    let _ = world.remove_one::<Despawn>(entity);
    let _ = world.insert(
        entity,
        (
            shop,
            Interactable {
                action: Interaction::Buy,
            },
            Prompt {
                text: "Press E to buy",
                gap: prompt_gap,
                color: PROMPT_COLOR,
                size: FontSize::Size22,
            },
        ),
    );
}

pub(crate) fn price_tag(
    world: &mut World,
    owner: Entity,
    texture: Tex,
    anchor_top: Vec2,
    price: i32,
    sale: Option<u32>,
    textures: &Textures,
) {
    let size = Vec2::splat(PRICE_TAG_SIZE);
    let center = Vec2::new(anchor_top.x, anchor_top.y - size.y / 2.0 - PRICE_TAG_LIFT);
    world.spawn((
        PriceTag { owner },
        Position(center),
        Size(size),
        Color(Vec4::ONE),
        Sprite::whole(textures.get(texture)),
        Layer(3),
        Lit,
    ));
    world.spawn((
        PriceTag { owner },
        Position(center + Vec2::new(0.0, 6.0)),
        Text::centered(
            price.to_string(),
            if sale.is_some() {
                SALE_COLOR
            } else {
                PRICE_COLOR
            },
            FontSize::Size22,
        ),
        Layer(4),
    ));
    if let Some(percent) = sale {
        world.spawn((
            PriceTag { owner },
            Position(center - Vec2::new(0.0, size.y / 2.0 + SALE_LIFT)),
            Text::centered(format!("SALE {percent}%"), SALE_COLOR, FontSize::Size22),
            Layer(4),
        ));
    }
}

pub(crate) fn artefact_hud(world: &mut World, textures: &Textures) {
    let frame_size = Vec2::splat(ARTEFACT_FRAME);
    screen_sprite(
        world,
        ARTEFACT_SLOT_POSITION,
        frame_size,
        textures.get(tex::ARTEFACTHUD),
        4,
    );
    let icon = screen_sprite(
        world,
        ARTEFACT_SLOT_POSITION,
        frame_size,
        textures.get(tex::ARTEFACTS_REVEAL_MAP),
        5,
    );
    let _ = world.insert(
        icon,
        (
            ArtefactIcon,
            Position(ARTEFACT_SLOT_POSITION + frame_size / 2.0),
            Hidden,
        ),
    );
}

pub(crate) fn status_bars(world: &mut World, textures: &Textures) {
    for kind in BarKind::HUD {
        let spec = kind.spec();
        let texture_size = spec_texture_size(kind);
        let size = texture_size * spec.scale;
        world.spawn((
            StatusBar {
                kind,
                fill: 1.0,
                delayed: 1.0,
            },
            Position(spec.position + size / 2.0),
            Size(size),
            Color(Vec4::ONE),
            Sprite::whole(textures.get(spec.texture)),
            Layer(5),
            ScreenSpace,
        ));
    }
}

fn spec_texture_size(kind: BarKind) -> Vec2 {
    match kind {
        BarKind::Health => Vec2::new(76.0, 19.0),
        BarKind::Armor => Vec2::new(83.0, 25.0),
        BarKind::Boss => Vec2::new(80.0, 19.0),
    }
}

const SLOT_POSITION: Vec2 = Vec2::new(685.0, 450.0);

const SLOT_SIZE: Vec2 = Vec2::new(225.0, 75.0);

const GUN_ICON_SIZE: Vec2 = Vec2::new(41.0, 34.0);

const AMMO_TEXT_POSITION: Vec2 = Vec2::new(900.0, 515.0);

const AMMO_TEXT_COLOR: Vec4 = Vec4::new(226.0 / 255.0, 153.0 / 255.0, 117.0 / 255.0, 1.0);
const COIN_MARGIN: f32 = 10.0;

const COIN_SIZE: Vec2 = Vec2::new(26.0, 26.0);
const COIN_TEXT_GAP: f32 = 15.0;

fn screen_sprite(
    world: &mut World,
    top_left: Vec2,
    size: Vec2,
    texture: TextureId,
    layer: i32,
) -> Entity {
    world.spawn((
        Position(top_left + size / 2.0),
        Size(size),
        Color(Vec4::ONE),
        Sprite::whole(texture),
        Layer(layer),
        ScreenSpace,
    ))
}

pub(crate) fn weapon_hud(world: &mut World, textures: &Textures) {
    screen_sprite(
        world,
        SLOT_POSITION,
        SLOT_SIZE,
        textures.get(tex::WEAPONSLOT),
        4,
    );
    let slot_center = SLOT_POSITION + SLOT_SIZE / 2.0;
    let icon = screen_sprite(
        world,
        slot_center - GUN_ICON_SIZE / 2.0,
        GUN_ICON_SIZE,
        textures.get(tex::GUNS_PISTOL),
        5,
    );
    let _ = world.insert(icon, (GunIcon, Hidden));
    world.spawn((
        AmmoText,
        Text::new(String::new(), AMMO_TEXT_COLOR, FontSize::Size22).aligned(Align::Right),
        Position(AMMO_TEXT_POSITION),
        Layer(6),
        ScreenSpace,
    ));
    for index in 0..MAX_CLIP as i32 {
        world.spawn((
            AmmoPip { index },
            Position(Vec2::ZERO),
            Size(PIP_SIZE),
            Color(Vec4::ONE),
            Sprite::cell(textures.get(tex::AMMO_HUD), 0, 0, 2, 2),
            Animation::new(2, 2, PIP_FULL),
            Layer(5),
            ScreenSpace,
            Hidden,
        ));
    }
}

pub(crate) fn coin_hud(world: &mut World, textures: &Textures, coin: Tex, text_color: Vec4) {
    let top_left = Vec2::new(COIN_MARGIN, 540.0 - COIN_MARGIN - COIN_SIZE.y);
    screen_sprite(world, top_left, COIN_SIZE, textures.get(coin), 5);
    world.spawn((
        CoinText,
        Text::new("0".to_owned(), text_color, FontSize::Size22).anchored(Anchor::Middle),
        Position(Vec2::new(
            COIN_MARGIN + COIN_SIZE.x + COIN_TEXT_GAP,
            top_left.y + COIN_SIZE.y / 2.0,
        )),
        Layer(6),
        ScreenSpace,
    ));
}
