// All amulet data types and interactions

use crate::setup::ParamNumPlayers;
use rand::{rng, seq::IndexedRandom};
use std::fmt;

#[derive(Debug, Clone, PartialEq)]
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

    pub const fn test_id(&self) -> &'static str {
        match self {
            AmuletType::Level01 => "current-level-01",
            AmuletType::Level04 => "current-level-04",
            AmuletType::Level06 => "current-level-06",
            AmuletType::Level08 => "current-level-08",
            AmuletType::Level10 => "current-level-10",
            AmuletType::Level12 => "current-level-12",
            AmuletType::Level16 => "current-level-16",
            AmuletType::Level20 => "current-level-20",
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct SetupId(u32);

impl SetupId {
    const BIT_LAYOUT: [u32; 8] = [4, 4, 3, 3, 4, 3, 3, 2];
    const MAX_VALUES: [usize; 8] = [8, 8, 5, 4, 8, 4, 5, 3];

    pub fn encode(setup: &SetupData) -> Option<Self> {
        let total_count = setup.removals.iter().map(|r| r.count).sum::<usize>();
        match setup.num_players {
            NumPlayers::Two if total_count != 16 => return None,
            NumPlayers::Three if total_count != 12 => return None,
            NumPlayers::Four if total_count != 8 => return None,
            _ => {}
        }

        // Encode the setup data into a 32-bit integer
        let num_players_bits = match setup.num_players {
            NumPlayers::Two => 0,
            NumPlayers::Three => 1,
            NumPlayers::Four => 2,
        };

        let mut bits = num_players_bits;

        let mut shift = 2;
        for (i, removal) in setup.removals.iter().enumerate() {
            let value = removal.count;
            if value > SetupId::MAX_VALUES[i] {
                return None;
            }
            bits |= (value as u32) << shift;
            shift += SetupId::BIT_LAYOUT[i];
        }

        Some(Self(bits))
    }

    pub fn decode(&self) -> Option<SetupData> {
        let num_players = match self.0 & 0b11 {
            0 => NumPlayers::Two,
            1 => NumPlayers::Three,
            2 => NumPlayers::Four,
            _ => return None, // Invalid player count
        };

        let mut setup = SetupData::new(num_players.clone());

        let mut shift = 2;
        let mut total_count = 0;
        for i in 0..8 {
            let mask = (1 << SetupId::BIT_LAYOUT[i]) - 1;
            let count = ((self.0 >> shift) & mask) as usize;
            if count > SetupId::MAX_VALUES[i] {
                // Invalid count for this amulet type
                return None;
            }
            setup.removals[i].count = count;
            total_count += count;
            shift += SetupId::BIT_LAYOUT[i];
        }

        // Validate total count against expected values for each player count
        match num_players {
            NumPlayers::Two if total_count != 16 => None,
            NumPlayers::Three if total_count != 12 => None,
            NumPlayers::Four if total_count != 8 => None,
            _ => Some(setup),
        }
    }

    pub fn to_hex_string(&self) -> String {
        format!("{:07X}", self.0)
    }

    pub fn from_hex_string(s: &str) -> Option<Self> {
        if s.len() > 7 {
            return None;
        }
        u32::from_str_radix(s, 16).ok().map(Self)
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

    fn setup_test_data(counts: [usize; 8], num_players: NumPlayers) -> SetupData {
        let removals = ALL_AMULET_TYPES
            .iter()
            .zip(counts.iter())
            .map(|(&ty, &count)| AmuletRemoval {
                amulet_type: ty,
                count,
            })
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();

        SetupData {
            num_players,
            removals,
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

    // Test cases for SetupId encoding and decoding
    #[test]
    fn test_valid_encoding_and_decoding_two_player() {
        let original = setup_test_data([4, 3, 2, 1, 3, 1, 1, 1], NumPlayers::Two);
        /*
        Field	        Value	Bits	Shift	Binary	Bits of bit field
        Num Players 	0	    2	    0	    00	    Bits 0–1 (LSB)
        Level01	        4	    4	    2	    0100	Bits 2–5
        Level04	        3	    4	    6	    0011	Bits 6–9
        Level06	        2	    3	    10	    010	    Bits 10–12
        Level08	        1	    3	    13	    001	    Bits 13–15
        Level10	        3	    4	    16	    0011	Bits 16–19
        Level12	        1	    3	    20	    001	    Bits 20–22
        Level16	        1	    3	    23	    001	    Bits 23–25
        Level20	        1	    2	    26	    01	    Bits 26–27 (MSB)
        --> MSB 01 001 001 0011 001 010 0011 0100 00 LSB
        --> MSB 0100 1001 0011 0010 1000 1101 0000 LSB
        --> 0b0100_1001_0011_0010_1000_1101_0000
        */
        let expected: u32 = 0b0100_1001_0011_0010_1000_1101_0000;
        let expected = SetupId(expected);
        let expected_hex = expected.to_hex_string();

        // Encode the original setup data
        let encoded = SetupId::encode(&original).expect("Should encode");
        let hex = encoded.to_hex_string();

        assert_eq!(hex, expected_hex);

        let parsed = SetupId::from_hex_string(&hex).expect("Should parse hex");
        let decoded = parsed.decode().expect("Should decode");

        assert_eq!(decoded.num_players, original.num_players);
        for i in 0..8 {
            assert_eq!(
                decoded.removals[i].count, original.removals[i].count,
                "Mismatch at index {i}"
            );
        }
    }

    #[test]
    fn test_valid_encoding_and_decoding_three_player() {
        let original = setup_test_data([2, 2, 2, 1, 2, 1, 1, 1], NumPlayers::Three);
        /*
        Field	        Value	Bits	Shift	Binary	Bits of bit field
        Num Players 	1	    2	    0	    01	    Bits 0–1 (LSB)
        Level01	        2	    4	    2	    0010	Bits 2–5
        Level04	        2	    4	    6	    0010	Bits 6–9
        Level06	        2	    3	    10	    010	    Bits 10–12
        Level08	        1	    3	    13	    001	    Bits 13–15
        Level10	        2	    4	    16	    0010	Bits 16–19
        Level12	        1	    3	    20	    001	    Bits 20–22
        Level16	        1	    3	    23	    001	    Bits 23–25
        Level20	        1	    2	    26	    01	    Bits 26–27 (MSB)
        --> MSB 01 001 001 0010 001 010 0010 0010 01 LSB
        --> MSB 0100 1001 0010 0010 1000 1000 1001 LSB
        --> 0b0100_1001_0010_0010_1000_1000_1001
        */
        let expected: u32 = 0b0100_1001_0010_0010_1000_1000_1001;
        let expected = SetupId(expected);
        let expected_hex = expected.to_hex_string();

        let encoded = SetupId::encode(&original).expect("Should encode");
        let hex = encoded.to_hex_string();

        assert_eq!(hex, expected_hex);

        let parsed = SetupId::from_hex_string(&hex).expect("Should parse hex");
        let decoded = parsed.decode().expect("Should decode");

        assert_eq!(decoded.num_players, original.num_players);
        for i in 0..8 {
            assert_eq!(
                decoded.removals[i].count, original.removals[i].count,
                "Mismatch at index {i}"
            );
        }
    }

    #[test]
    fn test_valid_encoding_and_decoding_four_player() {
        let original = setup_test_data([1, 1, 1, 1, 1, 1, 1, 1], NumPlayers::Four);
        /*
        Field	        Value	Bits	Shift	Binary	Bits of bit field
        Num Players 	2	    2	    0	    10	    Bits 0–1 (LSB)
        Level01	        1	    4	    2	    0101	Bits 2–5
        Level04	        1	    4	    6	    0001	Bits 6–9
        Level06	        1	    3	    10	    001	    Bits 10–12
        Level08	        1	    3	    13	    001	    Bits 13–15
        Level10	        1	    4	    16	    0001	Bits 16–19
        Level12	        1	    3	    20	    001	    Bits 20–22
        Level16	        1	    3	    23	    001	    Bits 23–25
        Level20	        1	    2	    26	    01	    Bits 26–27 (MSB)
        --> MSB 01 001 001 0001 001 001 0001 0001 10 LSB
        --> MSB 0100 1001 0001 0010 0100 0100 0110 LSB
        --> 0b0100_1001_0001_0010_0100_0100_0110
        */
        let expected: u32 = 0b0100_1001_0001_0010_0100_0100_0110;
        let expected = SetupId(expected);
        let expected_hex = expected.to_hex_string();

        let encoded = SetupId::encode(&original).expect("Should encode");
        let hex = encoded.to_hex_string();

        assert_eq!(hex, expected_hex);

        let parsed = SetupId::from_hex_string(&hex).expect("Should parse hex");
        let decoded = parsed.decode().expect("Should decode");

        assert_eq!(decoded.num_players, original.num_players);
        for i in 0..8 {
            assert_eq!(
                decoded.removals[i].count, original.removals[i].count,
                "Mismatch at index {i}"
            );
        }
    }

    #[test]
    fn test_invalid_counts() {
        // Count too high for Level06 (index 2, max 5)
        let invalid = setup_test_data([0, 0, 6, 0, 0, 0, 0, 0], NumPlayers::Two);
        assert!(SetupId::encode(&invalid).is_none());

        // Count too high for Level20 (index 7, max 3)
        let invalid = setup_test_data([0, 0, 0, 0, 0, 0, 0, 4], NumPlayers::Two);
        assert!(SetupId::encode(&invalid).is_none());

        // Total Count too low for Two Players (should be 16)
        let invalid = setup_test_data([8, 7, 0, 0, 0, 0, 0, 0], NumPlayers::Two);
        assert!(SetupId::encode(&invalid).is_none());

        // Total Count too high for Three Players (should be 12)
        let invalid = setup_test_data([6, 7, 0, 0, 0, 0, 0, 0], NumPlayers::Two);
        assert!(SetupId::encode(&invalid).is_none());
    }

    #[test]
    fn test_invalid_player_bits() {
        // manipulate bits: set num players to 3 (invalid, because only 0, 1, 2 are valid)
        let id = SetupId(0b11);
        assert!(id.decode().is_none());
    }

    #[test]
    fn test_invalid_count_bits() {
        /* count to high for Level04
        Field	        Value	Bits	Shift	Binary	Bits of bit field
        Num Players 	0	    2	    0	    00	    Bits 0–1 (LSB)
        Level01	        1	    4	    2	    0001	Bits 2–5
        Level04	        9	    4	    6	    1001	Bits 6–9
        Level06	        1	    3	    10	    001	    Bits 10–12
        Level08	        1	    3	    13	    001	    Bits 13–15
        Level10	        1	    4	    16	    0001	Bits 16–19
        Level12	        1	    3	    20	    001	    Bits 20–22
        Level16	        1	    3	    23	    001	    Bits 23–25
        Level20	        1	    2	    26	    01	    Bits 26–27 (MSB)
        --> MSB 01 001 001 0001 001 001 1001 0001 00 LSB
        --> MSB 0100 1001 0001 0010 0110 0100 0100 LSB
        --> 0b0100_1001_0001_0010_0110_0100_0100
        */
        let invalid: u32 = 0b0100_1001_0001_0010_0110_0100_0100;
        let invalid = SetupId(invalid);
        assert!(invalid.decode().is_none());

        /* total count of 17 to high for two players
        Field	        Value	Bits	Shift	Binary	Bits of bit field
        Num Players 	0	    2	    0	    00	    Bits 0–1 (LSB)
        Level01	        3	    4	    2	    0011	Bits 2–5
        Level04	        8	    4	    6	    1000	Bits 6–9
        Level06	        1	    3	    10	    001	    Bits 10–12
        Level08	        1	    3	    13	    001	    Bits 13–15
        Level10	        1	    4	    16	    0001	Bits 16–19
        Level12	        1	    3	    20	    001	    Bits 20–22
        Level16	        1	    3	    23	    001	    Bits 23–25
        Level20	        1	    2	    26	    01	    Bits 26–27 (MSB)
        --> MSB 01 001 001 0001 001 001 1000 0011 00 LSB
        --> MSB 0100 1001 0001 0010 0110 0000 1100 LSB
        --> 0b0100_1001_0001_0010_0110_0000_1100
        */
        let invalid: u32 = 0b0100_1001_0001_0010_0110_0000_1100;
        let invalid = SetupId(invalid);
        assert!(invalid.decode().is_none());
    }

    #[test]
    fn test_invalid_hex_string() {
        // too long
        assert!(SetupId::from_hex_string("12345678").is_none());

        // invalid chars
        assert!(SetupId::from_hex_string("GHIJKL").is_none());
    }
}
