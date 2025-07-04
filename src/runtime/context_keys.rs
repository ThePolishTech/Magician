use serenity::prelude::TypeMapKey;

pub struct WakeupChannelIdKey;
impl TypeMapKey for WakeupChannelIdKey{
    type Value = u64;
}
