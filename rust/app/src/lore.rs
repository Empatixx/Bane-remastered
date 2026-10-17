//! The story's words in one place: who the bosses were before the wound took them, and what
//! the wound whispers on the way down.
//!
//! The story in short. For four hundred years the brothers of the Order of the Seal prayed in
//! the temple of Velmor, one voice always awake, and what slept under the altar stayed asleep.
//! One night the brothers fell asleep at their prayers one by one; Elian, the youngest, was the
//! last awake, and then he slept too. The wound tore open and what came up killed the order.
//! Only Elian and Brother Aldric reached the crypt alive, and Elian never told anyone he had
//! fallen asleep. The player is Elian: he goes down through the Scientist's portal to close the
//! wound and bring the brothers back. The wound offers him exactly that, deeper; the bosses are
//! the brothers and the temple's people it took, and what they say as they fall gives away, a
//! piece at a time, who was on watch that night.

use crate::enemies::EnemyKind;

/// Who a boss was before the wound took it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct Identity {
    /// Its name, over the fight.
    pub(crate) name: &'static str,
    /// Who it was, under the name.
    pub(crate) epithet: &'static str,
    /// What it says as it falls: a piece of that night.
    pub(crate) last_words: &'static str,
    /// What Brother Aldric says of it in the laboratory, once it has been put to rest.
    pub(crate) aldric: &'static str,
    /// What the Scientist says of it, the first time.
    pub(crate) scientist: &'static str,
}

pub(crate) fn identity(kind: EnemyKind) -> Option<Identity> {
    let identity = match kind {
        EnemyKind::Spider => Identity {
            name: "Teodor",
            epithet: "the scribe, who wrote down every prayer",
            last_words: "\"...who was on watch... that night...?\"",
            aldric: "Teodor wrote down every prayer for forty years.",
            scientist: "A spider is a spider, novice. Do not give them names.",
        },
        EnemyKind::KingSlime => Identity {
            name: "The Faithful",
            epithet: "the people of Velmor, who came to pray and never left",
            last_words: "\"WE PRAYED ALL DAY  YOU ONLY HAD THE NIGHT\"",
            aldric: "The village folk. They were still in the nave when it came.",
            scientist: "The slime... Do not ask what it was made of.",
        },
        EnemyKind::Ogre => Identity {
            name: "Cassian",
            epithet: "keeper of the gate, who never slept on watch",
            last_words: "\"...the prayers stopped... I ran to the nave... the boy...\"",
            aldric: "Cassian never slept on watch. Not once in thirty years.",
            scientist: "A gatekeeper! Gates are for keeping things in, you know.",
        },
        EnemyKind::Golem => Identity {
            name: "The Angel",
            epithet: "who watched over the altar for four hundred years",
            last_words: "\"I SAW YOU CLOSE YOUR EYES\"",
            aldric: "That angel watched over us. It saw everything.",
            scientist: "Temple stone. It guarded out of habit, poor thing.",
        },
        EnemyKind::Argus => Identity {
            name: "The Eye",
            epithet: "of the wound, that opened when every other eye had closed",
            last_words: "It shows you the nave that night: one small figure kneeling, head bowed, asleep.",
            aldric: "An eye, you say? I dream of an eye.",
            scientist: "You closed the Eye? Remarkable. I have so many questions.",
        },
        EnemyKind::Hellhound => Identity {
            name: "Oswin's Hound",
            epithet: "who went down after its master and waits at the bottom",
            last_words: "It stops, sniffs at you, and lies down. It knows your smell.",
            aldric: "Oswin's dog went down after him. Poor beast.",
            scientist: "The hound. Do not listen to it. Dogs lie.",
        },
        EnemyKind::Mimic => Identity {
            name: "The Offering Box",
            epithet: "four hundred years of coins for the poor",
            last_words: "\"...more... more... MORE...\"",
            aldric: "The poor box from the nave. It learned to want.",
            scientist: "A chest with teeth! Bring me a tooth next time.",
        },
        _ => return None,
    };
    Some(identity)
}

/// What the wound whispers as Elian goes down onto a new floor: what he wants, offered deeper.
pub(crate) const WHISPERS: [&str; 6] = [
    "Deeper. I'll give them back.",
    "They're waiting for you below.",
    "You wanted to sleep. So sleep.",
    "It wasn't your fault. Come down and I'll prove it.",
    "Teodor is asking for you.",
    "Stay, and you'll never be alone again.",
];

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
}
