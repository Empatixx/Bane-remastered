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
    ShotgunShoot,
    ShotgunReload,
    RevolverShoot,
    M4Shoot,
    M4Reload,
    UziShoot,
    SwitchGun,
    OpenChest,
    Coin,
    Pickup,
    Heal,
    Buy,
    Sacrifice,
    Explosion,
    SlotSpin,
    SlotWin,
    SlotLose,
    SlotCashout,
}

impl SoundEffect {
    pub(crate) const ALL: [Self; 27] = [
        Self::Click,
        Self::PistolShoot,
        Self::EmptyGunShoot,
        Self::PistolReload,
        Self::WallHit,
        Self::EnemyHit,
        Self::PlayerHurt,
        Self::Death,
        Self::LowHealth,
        Self::ShotgunShoot,
        Self::ShotgunReload,
        Self::RevolverShoot,
        Self::M4Shoot,
        Self::M4Reload,
        Self::UziShoot,
        Self::SwitchGun,
        Self::OpenChest,
        Self::Coin,
        Self::Pickup,
        Self::Heal,
        Self::Buy,
        Self::Sacrifice,
        Self::Explosion,
        Self::SlotSpin,
        Self::SlotWin,
        Self::SlotLose,
        Self::SlotCashout,
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
            Self::ShotgunShoot => &["guns/shotgun_shoot.ogg"],
            Self::ShotgunReload => &["guns/shotgun_reload.ogg"],
            Self::RevolverShoot => &["guns/revolver_shoot.ogg"],
            Self::M4Shoot => &["guns/M4_shoot.ogg"],
            Self::M4Reload => &["guns/M4_reload.ogg"],
            Self::UziShoot => &["guns/uzi_shoot.ogg"],
            Self::SwitchGun => &["guns/switchgun.ogg"],
            Self::OpenChest => &["open_chest_1.ogg", "open_chest_2.ogg", "open_chest_3.ogg"],
            Self::Coin => &["coin.ogg"],
            Self::Pickup => &["pickup.ogg"],
            Self::Heal => &["heal.ogg"],
            Self::Buy => &["buy.ogg"],
            Self::Sacrifice => &["sacrifice.ogg"],
            Self::Explosion => &["explosion.ogg"],
            Self::SlotSpin => &["slotmachine_spin.ogg"],
            Self::SlotWin => &["slotmachine_win.ogg"],
            Self::SlotLose => &["slotmachine_lose.ogg"],
            Self::SlotCashout => &["slotmachine_cashout.ogg"],
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Soundtrack {
    MainMenu,
    InGame,
    ProgressRoom,
    Boss,
}

impl Soundtrack {
    pub(crate) const ALL: [Self; 4] =
        [Self::MainMenu, Self::InGame, Self::ProgressRoom, Self::Boss];

    pub(crate) fn file(self) -> &'static str {
        match self {
            Self::MainMenu => "intro.ogg",
            Self::InGame => "audio.ogg",
            Self::ProgressRoom => "ambient.ogg",
            Self::Boss => "boss.ogg",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum AudioCommand {
    Effect(SoundEffect),
    Music(Soundtrack),
}
