use log::error;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

mod skills;
mod tests;
// TODO: mod api; Expose the API endpoints for character sheets
pub mod api;

// HACK: FATE character sheet with slight customization and backup support
// TODO: Export as file (JSON or YAML?)
// TODO: Character sheet data structure
#[derive(Debug, Serialize, Deserialize)]
pub struct Sheet {
    name: Arc<str>,
    owner: Arc<super::chat::User>, // When a new sheet is made, it's owned by the user
    fatepoints: (i32, u8),
    // skills: skills::Skills,
    // TODO: Roll skills
    // TODO: Aspects Vec<Title, Desc, OptionalRoll>
    // TODO: Stunts (May interact with aspects) OptionImage
    // TODO: Extras
    // TODO: stress(bool, bool, bool, bool, bool)
    // TODO: Consequences (Depends on STRESS)
    // TODO: Inventory (Layers)
    // TODO: Traits (Kind of temporary aspects)
}

// TODO: Character sheet methods
impl Sheet {
    pub fn set_name(&mut self, name: &str) -> &mut Self {
        self.name = Arc::from(name.trim());
        self
    }

    pub fn add_fatepoints(&mut self, n: i32) -> &mut Self {
        self.fatepoints.0 += n;
        self
    }

    pub fn set_fatepoints(&mut self, n: i32) -> &mut Self {
        self.fatepoints.0 = n;
        self
    }
    pub fn reset_fatepoint(&mut self) -> &mut Self {
        self.fatepoints.0 = self.fatepoints.1.into();
        self
    }
}

//
// TODO: Character sheet chat integration
// TODO: Character sheet and "scenario" integration
// TODO: Character sheet and dice integration
//
