//! The story's words in one place: who the bosses were before the wound took them, the pages
//! found below, and what each waking costs Elian.
//!
//! The story in short. For four hundred years the Order of the Seal prayed in the temple of
//! Velmor, and as long as one voice prayed, what slept under the altar stayed asleep. A scholar
//! from the city, Silas Vane, wanted what it could give. On the night of the feast he came in
//! with a forged letter, cut the bell rope and poured the priests a sleeping draught; the last
//! prayer stopped, the seal broke, and he was waiting with his jars. He is the Scientist. The
//! creatures that came up killed the order, and the wound took them and gave them back: the
//! bosses are the temple's own people. Elian, the novice who survived, woke the guardian in
//! the crypt, and he wakes it again every time it dies. Every waking costs him: the order's
//! oldest rule says someone must pay for it, and he never says that it is him.

use crate::enemies::EnemyKind;

/// Who a boss was before the wound took it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct Identity {
    /// Its name, over the fight.
    pub(crate) name: &'static str,
    /// Who it was, under the name.
    pub(crate) epithet: &'static str,
    /// What it says as it falls: a clue to what happened that night.
    pub(crate) last_words: &'static str,
    /// What Elian says of it, in the laboratory, once it has been put to rest.
    pub(crate) elian: &'static str,
    /// What the Scientist says of it, the first time: he never quite tells the truth.
    pub(crate) scientist: &'static str,
}

pub(crate) fn identity(kind: EnemyKind) -> Option<Identity> {
    let identity = match kind {
        EnemyKind::Spider => Identity {
            name: "Teodor",
            epithet: "the scribe, who wrote until the threads came out of his hands",
            last_words: "\"...he asked me... for the oldest page...\"",
            elian: "Teodor wrote everything down. Even at the end.",
            scientist: "A spider is a spider, partner. Do not give them names.",
        },
        EnemyKind::KingSlime => Identity {
            name: "The Faithful",
            epithet: "the people of Velmor, who came to pray and never left",
            last_words: "\"WE ONLY CAME TO PRAY\"",
            elian: "The people from the village. They were kneeling when it came.",
            scientist: "The slime... Do not ask what it was made of.",
        },
        EnemyKind::Ogre => Identity {
            name: "Cassian",
            epithet: "keeper of the gate, who let one guest in after dark",
            last_words: "\"...the letter... had Aldric's seal... I let him in...\"",
            elian: "Cassian never let anyone in after dark. Never. Who had a letter?",
            scientist: "A gatekeeper! Gates are for keeping things in, you know.",
        },
        EnemyKind::Golem => Identity {
            name: "The Angel",
            epithet: "who stood over the altar and saw everything",
            last_words: "\"THE WINE   HE POURED THE WINE\"",
            elian: "The angel saw the feast. It saw who served the wine.",
            scientist: "Temple stone. It guarded out of habit, poor thing.",
        },
        EnemyKind::Argus => Identity {
            name: "The Eye",
            epithet: "that watched the temple for its master",
            last_words: "It shows you the night: a figure in the bell tower, cutting the rope.",
            elian: "That eye watched the temple for weeks before the feast. I saw it.",
            scientist: "You closed the Eye? It was... mine. Never mind.",
        },
        EnemyKind::Hellhound => Identity {
            name: "Brother Aldric's Hound",
            epithet: "who guards the bottom, waiting for its master",
            last_words: "It stops. It only ever wanted you to go no further.",
            elian: "Aldric's dog. It went down after him. It's still waiting.",
            scientist: "The hound. Do not listen to it. Dogs lie.",
        },
        EnemyKind::Mimic => Identity {
            name: "The Offering Box",
            epithet: "four hundred years of coins for the poor",
            last_words: "\"...more... more... MORE...\"",
            elian: "The poor box from the nave. It learned to want.",
            scientist: "A chest with teeth! Bring me a tooth next time.",
        },
        _ => return None,
    };
    Some(identity)
}

/// The pages found below, each in its writer's hand, in the order they are found. Read in
/// order they say who silenced the prayers, and what waking the guardian costs.
pub(crate) const PAGES: [(&str, &str); 8] = [
    ("Teodor's chronicle", "Year 412 of the watch. The prayers held again. Below the altar it slept, as it has for four hundred years."),
    ("Teodor's chronicle", "Aldric says it feeds on silence. While one voice prays, the seal holds. So we never all sleep at once."),
    ("Teodor's chronicle", "A scholar came from the city, Silas Vane. He asked what it gives, not what it is. Aldric sent him away."),
    ("Teodor's chronicle", "Vane came back with gold for our oldest records. I hid them. Today the oldest is gone from its hiding place."),
    ("Cassian's gate ledger", "Night of the feast. One guest let in after dark, with a letter in Brother Aldric's hand. Aldric was at prayer all evening."),
    ("Oswin's bell log", "Vespers. The bell rope is cut clean through. I will ring by hand. The feast wine tastes of almonds."),
    ("Teodor's chronicle", "Everyone is so tired. Only I and the boy are awake. If the prayers stop, the seal will... I cannot keep my eyes"),
    ("Aldric's oldest record", "The guardian came UP on the first night. We laid it in stone. Whoever wakes it pays for each waking, with his own years."),
];

/// How many of the five candles about Elian have gone out, by how many times he has woken the
/// guardian: each waking costs him.
pub(crate) fn candles_out(deaths: u64) -> usize {
    match deaths {
        0..=2 => 0,
        3..=7 => 1,
        8..=14 => 2,
        15..=24 => 3,
        _ => 4,
    }
}

/// What Elian lets slip as the wakings add up, the worst first.
pub(crate) fn elian_on_his_price(deaths: u64) -> &'static [&'static str] {
    match deaths {
        0..=2 => &[],
        3..=7 => &["*cough* It's nothing. The dust down there."],
        8..=14 => &["Don't worry about me. Just come back.", "My hands shake when I pray now. It's the cold."],
        15..=24 => &[
            "Each time I wake you, it takes a little. That's all.",
            "Aldric's oldest record... you've read it, haven't you.",
        ],
        _ => &["A few more times. I can do it a few more times.", "If I fall asleep while you're down there, keep going."],
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_boss_has_a_name_and_last_words() {
        for kind in EnemyKind::ALL.into_iter().filter(|kind| kind.is_boss()) {
            let identity = identity(kind).unwrap_or_else(|| panic!("{kind:?} has no identity"));
            assert!(!identity.name.is_empty() && !identity.last_words.is_empty());
        }
    }

    #[test]
    fn the_candles_go_out_one_by_one_and_one_always_stays_lit() {
        let out: Vec<usize> = [0, 3, 8, 15, 25, 1000].into_iter().map(candles_out).collect();
        assert_eq!(out, vec![0, 1, 2, 3, 4, 4]);
    }
}
