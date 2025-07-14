use serenity::prelude::TypeMapKey;
use std::collections::hash_map::HashMap;

pub struct WakeupChannelIdKey;
impl TypeMapKey for WakeupChannelIdKey {
    type Value = u64;
}

#[allow(dead_code)]
/// Data holding the future character's traits
pub struct CharacterCreationData {
    name: Option<String>,
    species: Option<String>,
    alignment: Option<String>,
    likes: Option<String>,
    dislikes: Option<String>,
    motivations: Option<String>,
    companions: Option<String>,
    backstory: Option<String>,
    appearance: Option<String>,
    extras: Option<String>
}
impl CharacterCreationData {
    pub fn new() -> CharacterCreationData {
        CharacterCreationData {
            name: None,
            species: None,
            alignment: None,
            likes: None,
            dislikes: None,
            motivations: None,
            companions: None,
            backstory: None,
            appearance: None,
            extras: None
        }
    }
}
pub struct CharacterBuildingDataKey;
impl TypeMapKey for CharacterBuildingDataKey {
    type Value = HashMap<u64, CharacterCreationData>;
}
