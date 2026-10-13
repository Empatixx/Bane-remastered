use std::collections::BTreeMap;

use serde::{Deserialize, Deserializer, Serialize, Serializer};
use winit::keyboard::KeyCode;

const KEYS: [(&str, KeyCode, &str); 36] = [
    ("KeyA", KeyCode::KeyA, "A"),
    ("KeyB", KeyCode::KeyB, "B"),
    ("KeyC", KeyCode::KeyC, "C"),
    ("KeyD", KeyCode::KeyD, "D"),
    ("KeyE", KeyCode::KeyE, "E"),
    ("KeyF", KeyCode::KeyF, "F"),
    ("KeyG", KeyCode::KeyG, "G"),
    ("KeyH", KeyCode::KeyH, "H"),
    ("KeyI", KeyCode::KeyI, "I"),
    ("KeyJ", KeyCode::KeyJ, "J"),
    ("KeyK", KeyCode::KeyK, "K"),
    ("KeyL", KeyCode::KeyL, "L"),
    ("KeyM", KeyCode::KeyM, "M"),
    ("KeyN", KeyCode::KeyN, "N"),
    ("KeyO", KeyCode::KeyO, "O"),
    ("KeyP", KeyCode::KeyP, "P"),
    ("KeyQ", KeyCode::KeyQ, "Q"),
    ("KeyR", KeyCode::KeyR, "R"),
    ("KeyS", KeyCode::KeyS, "S"),
    ("KeyT", KeyCode::KeyT, "T"),
    ("KeyU", KeyCode::KeyU, "U"),
    ("KeyV", KeyCode::KeyV, "V"),
    ("KeyW", KeyCode::KeyW, "W"),
    ("KeyX", KeyCode::KeyX, "X"),
    ("KeyY", KeyCode::KeyY, "Y"),
    ("KeyZ", KeyCode::KeyZ, "Z"),
    ("ArrowUp", KeyCode::ArrowUp, "Up"),
    ("ArrowDown", KeyCode::ArrowDown, "Down"),
    ("ArrowLeft", KeyCode::ArrowLeft, "Left"),
    ("ArrowRight", KeyCode::ArrowRight, "Right"),
    ("Space", KeyCode::Space, "Space"),
    ("Tab", KeyCode::Tab, "Tab"),
    ("Enter", KeyCode::Enter, "Enter"),
    ("ShiftLeft", KeyCode::ShiftLeft, "Shift"),
    ("ControlLeft", KeyCode::ControlLeft, "Ctrl"),
    ("AltLeft", KeyCode::AltLeft, "Alt"),
];

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) enum Action {
    MoveUp,
    MoveDown,
    MoveLeft,
    MoveRight,
    Interact,
    Reload,
    UseArtefact,
    DropGun,
    Map,
    Dodge,
}

impl Action {
    pub(crate) const ALL: [Self; 10] = [
        Self::MoveUp,
        Self::MoveDown,
        Self::MoveLeft,
        Self::MoveRight,
        Self::Interact,
        Self::Reload,
        Self::UseArtefact,
        Self::DropGun,
        Self::Map,
        Self::Dodge,
    ];

    fn index(self) -> usize {
        Self::ALL
            .iter()
            .position(|action| *action == self)
            .unwrap_or(0)
    }

    pub(crate) fn label(self) -> &'static str {
        match self {
            Self::MoveUp => "Move up",
            Self::MoveDown => "Move down",
            Self::MoveLeft => "Move left",
            Self::MoveRight => "Move right",
            Self::Interact => "Interact",
            Self::Reload => "Reload",
            Self::UseArtefact => "Artefact",
            Self::DropGun => "Drop gun",
            Self::Map => "Map",
            Self::Dodge => "Dodge",
        }
    }

    fn id(self) -> &'static str {
        match self {
            Self::MoveUp => "moveUp",
            Self::MoveDown => "moveDown",
            Self::MoveLeft => "moveLeft",
            Self::MoveRight => "moveRight",
            Self::Interact => "interact",
            Self::Reload => "reload",
            Self::UseArtefact => "useArtefact",
            Self::DropGun => "dropGun",
            Self::Map => "map",
            Self::Dodge => "dodge",
        }
    }

    fn default_key(self) -> KeyCode {
        match self {
            Self::MoveUp => KeyCode::KeyW,
            Self::MoveDown => KeyCode::KeyS,
            Self::MoveLeft => KeyCode::KeyA,
            Self::MoveRight => KeyCode::KeyD,
            Self::Interact => KeyCode::KeyE,
            Self::Reload => KeyCode::KeyR,
            Self::UseArtefact => KeyCode::KeyF,
            Self::DropGun => KeyCode::KeyQ,
            Self::Map => KeyCode::Tab,
            Self::Dodge => KeyCode::Space,
        }
    }
}

pub(crate) fn is_bindable(code: KeyCode) -> bool {
    KEYS.iter().any(|(_, key, _)| *key == code)
}

pub(crate) fn key_label(code: KeyCode) -> &'static str {
    KEYS.iter()
        .find(|(_, key, _)| *key == code)
        .map_or("?", |(_, _, label)| label)
}

fn key_name(code: KeyCode) -> Option<&'static str> {
    KEYS.iter()
        .find(|(_, key, _)| *key == code)
        .map(|(name, _, _)| *name)
}

fn parse_key(name: &str) -> Option<KeyCode> {
    KEYS.iter()
        .find(|(known, _, _)| *known == name)
        .map(|(_, key, _)| *key)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct Bindings {
    keys: [KeyCode; 10],
}

impl Default for Bindings {
    fn default() -> Self {
        Self {
            keys: Action::ALL.map(Action::default_key),
        }
    }
}

impl Bindings {
    pub(crate) fn key(&self, action: Action) -> KeyCode {
        self.keys[action.index()]
    }

    pub(crate) fn rebind(&mut self, action: Action, code: KeyCode) {
        if !is_bindable(code) {
            return;
        }
        let previous = self.key(action);
        if let Some(other) = Action::ALL
            .into_iter()
            .find(|other| *other != action && self.key(*other) == code)
        {
            self.keys[other.index()] = previous;
        }
        self.keys[action.index()] = code;
    }

    fn has_duplicates(&self) -> bool {
        self.keys
            .iter()
            .enumerate()
            .any(|(index, key)| self.keys[index + 1..].contains(key))
    }
}

impl Serialize for Bindings {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let map: BTreeMap<&str, &str> = Action::ALL
            .into_iter()
            .filter_map(|action| key_name(self.key(action)).map(|name| (action.id(), name)))
            .collect();
        map.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for Bindings {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let map = BTreeMap::<String, String>::deserialize(deserializer)?;
        let mut bindings = Self::default();
        for action in Action::ALL {
            if let Some(code) = map.get(action.id()).and_then(|name| parse_key(name)) {
                bindings.keys[action.index()] = code;
            }
        }
        Ok(if bindings.has_duplicates() {
            Self::default()
        } else {
            bindings
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_defaults_are_the_keys_bane_uses() {
        let bindings = Bindings::default();
        assert_eq!(bindings.key(Action::MoveUp), KeyCode::KeyW);
        assert_eq!(bindings.key(Action::Interact), KeyCode::KeyE);
        assert_eq!(bindings.key(Action::Map), KeyCode::Tab);
        assert!(!bindings.has_duplicates());
    }

    #[test]
    fn rebinding_to_a_free_key_changes_only_that_action() {
        let mut bindings = Bindings::default();
        bindings.rebind(Action::Reload, KeyCode::KeyX);
        assert_eq!(bindings.key(Action::Reload), KeyCode::KeyX);
        assert_eq!(bindings.key(Action::Interact), KeyCode::KeyE);
    }

    #[test]
    fn rebinding_to_a_taken_key_swaps_the_two_actions() {
        let mut bindings = Bindings::default();
        bindings.rebind(Action::Reload, KeyCode::KeyE);
        assert_eq!(bindings.key(Action::Reload), KeyCode::KeyE);
        assert_eq!(bindings.key(Action::Interact), KeyCode::KeyR);
        assert!(!bindings.has_duplicates());
    }

    #[test]
    fn escape_and_the_weapon_slot_digits_cannot_be_bound() {
        let mut bindings = Bindings::default();
        bindings.rebind(Action::Reload, KeyCode::Escape);
        bindings.rebind(Action::Reload, KeyCode::Digit1);
        assert_eq!(bindings.key(Action::Reload), KeyCode::KeyR);
        assert!(!is_bindable(KeyCode::Escape));
        assert!(is_bindable(KeyCode::Space));
    }

    #[test]
    fn bindings_survive_a_json_round_trip() {
        let mut bindings = Bindings::default();
        bindings.rebind(Action::MoveUp, KeyCode::ArrowUp);
        let json = serde_json::to_string(&bindings).expect("serializes");
        assert!(json.contains("\"moveUp\":\"ArrowUp\""));
        let back: Bindings = serde_json::from_str(&json).expect("parses");
        assert_eq!(back, bindings);
    }

    #[test]
    fn unknown_keys_fall_back_to_the_default_and_duplicates_reset_everything() {
        let partial: Bindings =
            serde_json::from_str(r#"{"reload":"Nonsense","interact":"KeyX"}"#).expect("parses");
        assert_eq!(partial.key(Action::Reload), KeyCode::KeyR);
        assert_eq!(partial.key(Action::Interact), KeyCode::KeyX);

        let clash: Bindings =
            serde_json::from_str(r#"{"reload":"KeyE","interact":"KeyE"}"#).expect("parses");
        assert_eq!(clash, Bindings::default());
    }

    #[test]
    fn key_labels_are_short_and_readable() {
        assert_eq!(key_label(KeyCode::KeyW), "W");
        assert_eq!(key_label(KeyCode::ArrowLeft), "Left");
        assert_eq!(key_label(KeyCode::F1), "?");
    }
}
