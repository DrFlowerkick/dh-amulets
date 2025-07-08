// All amulet data types and interactions

use crate::app::ParamNumPlayers;
use rand::{rng, seq::SliceRandom};
use std::fmt;

#[derive(Debug, Clone)]
pub enum NumPlayers {
    Two,
    Three,
    Four,
}

impl fmt::Display for NumPlayers {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            NumPlayers::Two => write!(f, "Zwei"),
            NumPlayers::Three => write!(f, "Drei"),
            NumPlayers::Four => write!(f, "Vier"),
        }
    }
}

impl From<ParamNumPlayers> for NumPlayers {
    fn from(value: ParamNumPlayers) -> Self {
        match value.num {
            Some(4) => NumPlayers::Four,
            Some(3) => NumPlayers::Three,
            _ => NumPlayers::Two, // default to two players
        }
    }
}

#[derive(Clone, Debug)]
pub struct AmuletRemoval {
    pub amulet_type: AmuletType,
    pub count: usize,
}

#[derive(Clone, Debug)]
pub struct SetupData {
    pub num_players: NumPlayers,
    pub removals: [AmuletRemoval; 8],
}

impl SetupData {
    fn new(num_players: NumPlayers) -> Self {
        SetupData {
            num_players,
            removals: [
                AmuletRemoval {
                    amulet_type: AmuletType::Level01,
                    count: 0,
                },
                AmuletRemoval {
                    amulet_type: AmuletType::Level04,
                    count: 0,
                },
                AmuletRemoval {
                    amulet_type: AmuletType::Level06,
                    count: 0,
                },
                AmuletRemoval {
                    amulet_type: AmuletType::Level08,
                    count: 0,
                },
                AmuletRemoval {
                    amulet_type: AmuletType::Level10,
                    count: 0,
                },
                AmuletRemoval {
                    amulet_type: AmuletType::Level12,
                    count: 0,
                },
                AmuletRemoval {
                    amulet_type: AmuletType::Level16,
                    count: 0,
                },
                AmuletRemoval {
                    amulet_type: AmuletType::Level20,
                    count: 0,
                },
            ],
        }
    }

    pub fn setup(num_players: NumPlayers) -> Self {
        let mut rng = rng();
        let mut new_set = AmuletType::full_set();
        new_set.shuffle(&mut rng);
        let amulets_to_remove = match num_players {
            NumPlayers::Two => &new_set[0..16],
            NumPlayers::Three => &new_set[0..12],
            NumPlayers::Four => &new_set[0..8],
        };
        let mut setup = SetupData::new(num_players);
        for amulet in setup.removals.iter_mut() {
            let a_type_count = amulets_to_remove
                .iter()
                .filter(|a| **a == amulet.amulet_type)
                .count();
            amulet.count = a_type_count;
        }
        setup
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Debug, Hash)]
pub enum AmuletType {
    Level01,
    Level04,
    Level06,
    Level08,
    Level10,
    Level12,
    Level16,
    Level20,
}

impl AmuletType {
    fn full_set() -> Vec<Self> {
        let mut full_set: Vec<AmuletType> = Vec::with_capacity(45);
        full_set.extend_from_slice(&[AmuletType::Level01; 8]);
        full_set.extend_from_slice(&[AmuletType::Level04; 8]);
        full_set.extend_from_slice(&[AmuletType::Level06; 5]);
        full_set.extend_from_slice(&[AmuletType::Level08; 4]);
        full_set.extend_from_slice(&[AmuletType::Level10; 8]);
        full_set.extend_from_slice(&[AmuletType::Level12; 4]);
        full_set.extend_from_slice(&[AmuletType::Level16; 5]);
        full_set.extend_from_slice(&[AmuletType::Level20; 3]);
        full_set
    }

    pub fn image_path(&self) -> &'static str {
        match self {
            AmuletType::Level01 => "/images/amulets/amulet_01.png",
            AmuletType::Level04 => "/images/amulets/amulet_04.png",
            AmuletType::Level06 => "/images/amulets/amulet_06.png",
            AmuletType::Level08 => "/images/amulets/amulet_08.png",
            AmuletType::Level10 => "/images/amulets/amulet_10.png",
            AmuletType::Level12 => "/images/amulets/amulet_12.png",
            AmuletType::Level16 => "/images/amulets/amulet_16.png",
            AmuletType::Level20 => "/images/amulets/amulet_20.png",
        }
    }

    pub fn alt_text(&self) -> &'static str {
        match self {
            AmuletType::Level01 => "Amulet Level 01",
            AmuletType::Level04 => "Amulet Level 04",
            AmuletType::Level06 => "Amulet Level 06",
            AmuletType::Level08 => "Amulet Level 08",
            AmuletType::Level10 => "Amulet Level 10",
            AmuletType::Level12 => "Amulet Level 12",
            AmuletType::Level16 => "Amulet Level 16",
            AmuletType::Level20 => "Amulet Level 20",
        }
    }
}
