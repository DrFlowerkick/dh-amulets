// All amulet data types and interactions

use crate::app::ParamNumPlayers;
use rand::{rng, seq::IndexedRandom};
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
        let amulets_to_remove: usize = match num_players {
            NumPlayers::Two => 16,
            NumPlayers::Three => 12,
            NumPlayers::Four => 8,
        };
        AmuletType::full_set()
            .choose_multiple(&mut rng, amulets_to_remove)
            .fold(Self::new(num_players), |mut setup, amulet| {
                #[cfg(test)]
                assert_eq!(setup.removals[*amulet as usize].amulet_type, *amulet);
                setup.removals[*amulet as usize].count += 1;
                setup
            })
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

    pub const fn image_path(&self) -> &'static str {
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

    pub const fn alt_text(&self) -> &'static str {
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

#[cfg(test)]
mod tests {
    use super::*;

    const ALL_AMULET_TYPES: [AmuletType; 8] = [
        AmuletType::Level01,
        AmuletType::Level04,
        AmuletType::Level06,
        AmuletType::Level08,
        AmuletType::Level10,
        AmuletType::Level12,
        AmuletType::Level16,
        AmuletType::Level20,
    ];

    impl AmuletType {
        const fn max_num(&self) -> usize {
            match self {
                AmuletType::Level01 => 8,
                AmuletType::Level04 => 8,
                AmuletType::Level06 => 5,
                AmuletType::Level08 => 4,
                AmuletType::Level10 => 8,
                AmuletType::Level12 => 4,
                AmuletType::Level16 => 5,
                AmuletType::Level20 => 3,
            }
        }
    }

    #[test]
    fn test_full_set() {
        let full_set = AmuletType::full_set();
        for amulet in ALL_AMULET_TYPES {
            assert_eq!(
                full_set.iter().filter(|a| **a == amulet).count(),
                amulet.max_num()
            );
        }
    }

    #[test]
    fn test_setup_two_player() {
        let two_player_setup = SetupData::setup(NumPlayers::Two);
        assert!(matches!(two_player_setup.num_players, NumPlayers::Two));
        assert_eq!(
            two_player_setup
                .removals
                .iter()
                .map(|r| r.count)
                .sum::<usize>(),
            16
        );
        for amulet in ALL_AMULET_TYPES {
            assert!(
                two_player_setup
                    .removals
                    .iter()
                    .filter(|r| r.amulet_type == amulet)
                    .count()
                    <= amulet.max_num()
            );
        }
    }

    #[test]
    fn test_setup_three_player() {
        let two_player_setup = SetupData::setup(NumPlayers::Three);
        assert!(matches!(two_player_setup.num_players, NumPlayers::Three));
        assert_eq!(
            two_player_setup
                .removals
                .iter()
                .map(|r| r.count)
                .sum::<usize>(),
            12
        );
        for amulet in ALL_AMULET_TYPES {
            assert!(
                two_player_setup
                    .removals
                    .iter()
                    .filter(|r| r.amulet_type == amulet)
                    .count()
                    <= amulet.max_num()
            );
        }
    }

    #[test]
    fn test_setup_four_player() {
        let two_player_setup = SetupData::setup(NumPlayers::Four);
        assert!(matches!(two_player_setup.num_players, NumPlayers::Four));
        assert_eq!(
            two_player_setup
                .removals
                .iter()
                .map(|r| r.count)
                .sum::<usize>(),
            8
        );
        for amulet in ALL_AMULET_TYPES {
            assert!(
                two_player_setup
                    .removals
                    .iter()
                    .filter(|r| r.amulet_type == amulet)
                    .count()
                    <= amulet.max_num()
            );
        }
    }
}
