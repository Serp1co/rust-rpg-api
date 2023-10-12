use serde::{Deserialize, Serialize};
use utoipa::{ToResponse, ToSchema};

#[derive(Serialize, Deserialize, ToSchema, ToResponse, Clone)]
pub struct PlayerSheet {
    pub id: String,
    pub name: String,
    pub surname: String,
    pub race: RaceSheet,
    pub experience: u32,
    pub level: u8,
    pub stats_primary: PlayerStatsPrimary,
    pub stats_derived: PlayerStatsDerived,
}
impl PlayerSheet {
    pub fn is_alive(&self) -> bool {
        self.stats_derived.health_physic > 0 && self.stats_derived.health_mental > 0
    }

    pub fn new(
        name: String,
        surname: String,
        race: RaceSheet,
        experience: u32,
        level: u8,
        stats_primary: PlayerStatsPrimary,
        optional_stats_derived: Option<PlayerStatsDerived>,
    ) -> PlayerSheet {
        let mut final_primary_stats: PlayerStatsPrimary = stats_primary;
        final_primary_stats.fortitude += race.stats_modifiers_primary.fortitude;
        final_primary_stats.dexterity += race.stats_modifiers_primary.dexterity;
        final_primary_stats.intelligence += race.stats_modifiers_primary.intelligence;
        final_primary_stats.occultism += race.stats_modifiers_primary.occultism;
        final_primary_stats.sociality += race.stats_modifiers_primary.sociality;

        let final_stats_derived: PlayerStatsDerived = match optional_stats_derived {
            Some(stat_derived) => stat_derived,
            None => PlayerStatsDerived::new(final_primary_stats.clone()),
        };

        PlayerSheet {
            id: uuid::Uuid::new_v4().to_string(),
            name,
            surname,
            race,
            experience,
            level,
            stats_primary: final_primary_stats.clone(),
            stats_derived: final_stats_derived.clone(),
        }
    }
}

#[derive(Serialize, Deserialize, ToSchema, ToResponse, Clone)]
pub struct PlayerStatsPrimary {
    pub fortitude: i8,
    pub dexterity: i8,
    pub intelligence: i8,
    pub occultism: i8,
    pub sociality: i8,
}

#[derive(Serialize, Deserialize, ToSchema, ToResponse, Clone)]
pub struct PlayerStatsDerived {
    pub health_physic: i8,
    pub health_mental: i8,
    pub atk_physic: i8,
    pub atk_magic: i8,
    pub def_physic: i8,
    pub def_magic: i8,
}
impl PlayerStatsDerived {
    fn health_formula(primary: i8) -> i8 {
        primary * 2
    }
    fn atk_formula(primary: i8, secondary: i8) -> i8 {
        (primary as f32 + secondary as f32 * 0.5).floor() as i8
    }
    fn def_formula(primary: i8, secondary: i8) -> i8 {
        ((primary as f32 * 0.5) + (secondary as f32 * 0.5) + 10.0).floor() as i8
    }

    pub fn new(stats_primary: PlayerStatsPrimary) -> PlayerStatsDerived {
        PlayerStatsDerived {
            health_physic: PlayerStatsDerived::health_formula(stats_primary.fortitude),
            health_mental: PlayerStatsDerived::health_formula(stats_primary.intelligence),
            atk_physic: PlayerStatsDerived::atk_formula(
                stats_primary.dexterity,
                stats_primary.fortitude,
            ),
            atk_magic: PlayerStatsDerived::atk_formula(
                stats_primary.occultism,
                stats_primary.intelligence,
            ),
            def_physic: PlayerStatsDerived::def_formula(
                stats_primary.dexterity,
                stats_primary.fortitude,
            ),
            def_magic: PlayerStatsDerived::def_formula(
                stats_primary.occultism,
                stats_primary.intelligence,
            ),
        }
    }
    pub fn default() -> PlayerStatsDerived {
        PlayerStatsDerived {
            health_physic: 1,
            health_mental: 1,
            atk_physic: 0,
            atk_magic: 0,
            def_physic: 0,
            def_magic: 0,
        }
    }
}

#[derive(Serialize, Deserialize, ToSchema, ToResponse, Clone)]
pub struct RaceSheet {
    pub id: u8,
    pub name: String,
    pub stats_modifiers_primary: PlayerStatsPrimary,
}

#[derive(Serialize, Deserialize, ToSchema, ToResponse, Clone)]
pub struct Player {
    pub user: String,
    pub player_sheet: PlayerSheet,
}
impl Player {
    pub fn status(&self) -> String {
        if self.player_sheet.is_alive() {
            return String::from("Alive");
        }
        String::from("Dead")
    }
}
