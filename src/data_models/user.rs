use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct UserLogin {
    pub username: String,
    pub password: String,
} 

#[derive(Serialize, Deserialize, Clone)]
pub struct UserSignup {
    pub username: String,
    pub password: String,
    pub email: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct PlayerSheet {
    name: String,
    surname: String,
    race: RaceType,
    stats_primary: PlayerStatsPrimary,
    stats_derived: PlayerStatsDerived,
} impl PlayerSheet {
    fn is_alive(&self) -> bool {
        return self.stats_derived.health > 0;
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct PlayerStatsPrimary {
    fortitude: i8,
    dexterity: i8,
    intelligence: i8,
    occultism: i8,
    sociality: i8,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct PlayerStatsDerived {
    health: i8,
    atk_physic: i8,
    atk_magic: i8,
    def_physic: i8,
    def_magic: i8,
    evade_physic: i8,
    evade_mental: i8,
    resist_physic: i8,
    resist_mental: i8,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct RaceType {
    id: u8,
    name: String,
    stats_modifiers_primary: PlayerStatsPrimary,
    stats_modifiers_derived: PlayerStatsDerived
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Player {
    pub user: String,
    pub player_sheet: PlayerSheet,
    pub race_type: RaceType
} 
impl Player {
    pub fn status(&self) -> String {
        if self.player_sheet.is_alive() { return String::from("Alive") }
        String::from("Dead")
    }

    pub fn get_player_sheet(&self) -> PlayerSheet {
        self.player_sheet.clone()
    }

    pub fn get_race_type(&self) -> RaceType {
        self.race_type.clone()
    }

}

pub fn create_new_player(user_login: UserLogin, player_sheet: PlayerSheet, race_type: RaceType) -> Player{
    Player { user: (user_login.username), player_sheet: (player_sheet), race_type: (race_type) }
}

