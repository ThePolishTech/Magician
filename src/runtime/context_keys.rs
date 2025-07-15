use serenity::{
    prelude::TypeMapKey,
    model::channel::Message
};
use std::collections::hash_map::HashMap;

pub struct WakeupChannelIdKey;
impl TypeMapKey for WakeupChannelIdKey {
    type Value = u64;
}

/// Containts a `HashMap` who's keys are the id's of users who are in the process of building a
/// character. The value of this `HashMap` is a tuple of another `HashMap` where the key `String`
/// is the name of a character's attribute, and the value is a `String` containing it's value when 
/// it it known.
/// The second element of the tuple is a `Message` struct that
/// coresponds to the base message that the bot sends when `character create` gets called that in
/// turn contains an editable embed and an editable ActionRow contianing Buttons
pub struct CharacterBuildingDataKey;
impl TypeMapKey for CharacterBuildingDataKey {
    // In order to not have to bother with editing an arbitrary message using a message- and
    // chanel_id, we will use a `Message` in order to utilise it's `edit` method for a more
    // conveniant and sure-fire approach
    type Value = HashMap<u64, (HashMap<String, String>, Message)>; 
}
