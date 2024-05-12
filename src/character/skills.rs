use super::*;
use std::collections::HashMap;
use std::sync::Arc;

// DONE: List of skills that can be modified by user later
// TODO: Handle possible crash cases
#[derive(Debug)]
pub struct Skills(pub HashMap<Arc<str>, i32>);

macro_rules! skill {
    ($name:expr) => {
        (Arc::from($name), 0)
    };
}

/// Skill chainable methods for manipulation of skills
impl Skills {
    /// Adds a skill to the HashMap, may overwrite if already exists
    pub fn add_skill(&mut self, skill: &str) -> &mut Self {
        self.0.insert(Arc::from(skill), 0);
        self
    }
    /// Increments skill by 1, **should never be called on nonexistant skill**
    pub fn increment_skill(&mut self, skill: &str) -> &mut Self {
        if let Some(value) = self.0.get_mut(skill) {
            *value += 1;
            self
        } else {
            error!("Tried to increment nonexistant skill: {}", skill);
            unreachable!()
        }
    }
    /// Removes skill from HashMap
    /// **Wont check if it exists!**
    pub fn remove_skill(&mut self, skill: &str) -> &mut Self {
        self.0.remove(skill);
        self
    }
    /// Simply gets the value of a skill from the HashMap
    /// **Does not check if it exists**
    pub fn get_skill(&self, skill: &str) -> i32 {
        *self.0.get(skill).unwrap()
    }
}

impl Default for Skills {
    fn default() -> Self {
        Skills(HashMap::from([
            skill!("Talent"),
            skill!("Burglary"),
            skill!("Athletics"),
            skill!("Contacts"),
            skill!("Crafts"),
            skill!("Deceive"),
            skill!("Drive"),
            skill!("Empathy"),
            skill!("Fight"),
            skill!("Investigation"),
            skill!("Lore"),
            skill!("Perception"),
            skill!("Physique"),
            skill!("Provoke"),
            skill!("Rapport"),
            skill!("Resources"),
            skill!("Shoot"),
            skill!("Stealth"),
            skill!("Will"),
            skill!("Calculus"),
            skill!("Scouting"),
            skill!("Quick"),
            skill!("Careful"),
            skill!("Performance"),
            skill!("Survival"),
            skill!("Arcana"),
        ]))
    }
}
