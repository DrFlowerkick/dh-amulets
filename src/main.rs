use axum::{
    extract::Path,
    routing::get,
    Router,
    response::IntoResponse,
};
use askama::Template;
use std::fmt;
use rand::prelude::*;
use tower_http::services::ServeDir;



#[shuttle_runtime::main]
async fn axum() -> shuttle_axum::ShuttleAxum {
    let router = Router::new()
        .route("/setup/:num_of_players", get(get_amulets))
        .nest_service("/", ServeDir::new("static/"));

    Ok(router.into())
}

async fn get_amulets(
    Path(num_players): Path<u8>,
) -> impl IntoResponse {
    let mut amulet_setup = AmuletSetup::new(num_players);
    amulet_setup.setup();
    amulet_setup
}

#[derive(Debug)]
enum NumPlayers {
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

impl Default for NumPlayers {
    fn default() -> Self {
        NumPlayers::Two
    }
}

#[derive(Copy, Clone, PartialEq)]
enum AmuletTypes {
    Level01,
    Level04,
    Level06,
    Level08,
    Level10,
    Level12,
    Level16,
    Level20,
}

impl AmuletTypes {
    fn iter() -> impl Iterator<Item = AmuletTypes> {
        IterAmuletTypes::new()
    }

    fn full_set() -> Vec<Self> {
        let mut full_set: Vec<AmuletTypes> = Vec::with_capacity(45);
        full_set.append(&mut vec![AmuletTypes::Level01; 8]);
        full_set.append(&mut vec![AmuletTypes::Level04; 8]);
        full_set.append(&mut vec![AmuletTypes::Level06; 5]);
        full_set.append(&mut vec![AmuletTypes::Level08; 4]);
        full_set.append(&mut vec![AmuletTypes::Level10; 8]);
        full_set.append(&mut vec![AmuletTypes::Level12; 4]);
        full_set.append(&mut vec![AmuletTypes::Level16; 5]);
        full_set.append(&mut vec![AmuletTypes::Level20; 3]);
        full_set
    }
}

struct IterAmuletTypes {
    a_type: AmuletTypes,
    finished: bool,
}

impl IterAmuletTypes {
    fn new() -> Self {
        IterAmuletTypes {
            a_type: AmuletTypes::Level01,
            finished: false
        }
    }
}

impl Iterator for IterAmuletTypes {
    type Item = AmuletTypes;

    fn next(&mut self) -> Option<Self::Item> {
        if self.finished {
            return None;
        }
        let result = self.a_type;
        self.a_type = match self.a_type {
            AmuletTypes::Level01 => AmuletTypes::Level04,
            AmuletTypes::Level04 => AmuletTypes::Level06,
            AmuletTypes::Level06 => AmuletTypes::Level08,
            AmuletTypes::Level08 => AmuletTypes::Level10,
            AmuletTypes::Level10 => AmuletTypes::Level12,
            AmuletTypes::Level12 => AmuletTypes::Level16,
            AmuletTypes::Level16 => AmuletTypes::Level20,
            AmuletTypes::Level20 => {
                self.finished = true;
                AmuletTypes::Level20
            },
        };
        Some(result)
    }
}

#[derive(Template, Default)]
#[template(path = "remove_amulets.html")]
struct AmuletSetup {
    num_players: NumPlayers,
    level_1: usize,
    level_4: usize,
    level_6: usize,
    level_8: usize,
    level_10: usize,
    level_12: usize,
    level_16: usize,
    level_20: usize,
}

impl AmuletSetup {
    fn new(num_players: u8) -> Self {
        let num_players: NumPlayers = match num_players {
            4 => NumPlayers::Four,
            3 => NumPlayers::Three,
            _ => NumPlayers::Two,
        };
        let mut result = Self::default();
        result.num_players = num_players;
        result
    }

    fn setup(&mut self) {
        let mut rng = thread_rng();
        let mut new_set = AmuletTypes::full_set();
        new_set.shuffle(&mut rng);
        let amulets_to_remove = match self.num_players {
            NumPlayers::Two => &new_set[0..16],
            NumPlayers::Three =>  &new_set[0..12],
            NumPlayers::Four =>  &new_set[0..8],
        };
        for a_type in AmuletTypes::iter() {
            let a_type_count = amulets_to_remove.iter().filter(|a| **a == a_type).count();
            match a_type {
                AmuletTypes::Level01 => self.level_1 = a_type_count,
                AmuletTypes::Level04 => self.level_4 = a_type_count,
                AmuletTypes::Level06 => self.level_6 = a_type_count,
                AmuletTypes::Level08 => self.level_8 = a_type_count,
                AmuletTypes::Level10 => self.level_10 = a_type_count,
                AmuletTypes::Level12 => self.level_12 = a_type_count,
                AmuletTypes::Level16 => self.level_16 = a_type_count,
                AmuletTypes::Level20 => self.level_20 = a_type_count,
            }
        }
    }
}