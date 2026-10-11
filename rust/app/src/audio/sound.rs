#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum SoundEffect {
    Click,
    PistolShoot,
    EmptyGunShoot,
    PistolReload,
    WallHit,
    EnemyHit,
    PlayerHurt,
    Death,
    LowHealth,
}

impl SoundEffect {
    pub(crate) const ALL: [Self; 9] = [
        Self::Click,
        Self::PistolShoot,
        Self::EmptyGunShoot,
        Self::PistolReload,
        Self::WallHit,
        Self::EnemyHit,
        Self::PlayerHurt,
        Self::Death,
        Self::LowHealth,
    ];

    pub(crate) fn files(self) -> &'static [&'static str] {
        match self {
            Self::Click => &["menuclick.ogg"],
            Self::PistolShoot => &["guns/pistol_shoot_1.ogg", "guns/pistol_shoot_2.ogg"],
            Self::EmptyGunShoot => &["guns/empty_gun_shoot.ogg"],
            Self::PistolReload => &["guns/pistol_reload.ogg"],
            Self::WallHit => &["guns/wall_hit.ogg"],
            Self::EnemyHit => &["guns/enemy_hit.ogg"],
            Self::PlayerHurt => &["playerhurt_1.ogg", "playerhurt_2.ogg"],
            Self::Death => &["playerdeath.ogg"],
            Self::LowHealth => &["lowhealth.ogg"],
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Soundtrack {
    MainMenu,
    InGame,
    ProgressRoom,
}

impl Soundtrack {
    pub(crate) const ALL: [Self; 3] = [Self::MainMenu, Self::InGame, Self::ProgressRoom];

    pub(crate) fn file(self) -> &'static str {
        match self {
            Self::MainMenu => "intro.ogg",
            Self::InGame => "audio.ogg",
            Self::ProgressRoom => "ambient.ogg",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum AudioCommand {
    Effect(SoundEffect),
    Music(Soundtrack),
}
