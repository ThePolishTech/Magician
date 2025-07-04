use serenity::{all::{ChannelId, Context, CreateMessage, EventHandler, Ready}, async_trait};
use crate::utils::runtime_utils::{
    runtime_client,
    context_keys
};


#[async_trait]
impl EventHandler for runtime_client::RuntimeClient {
    
    async fn ready( &self, ctx: Context, _ready: Ready) {
        let wakeup_channel_id = {
            let data_read = ctx.data.read();
            let wakeup_channel_id = data_read.await.get::<context_keys::WakeupChannelIdKey>()
                .expect("Wakeup channel id should be stored in Context")
                .clone(); // To allow us to drop the lock guard as early as possible. Preformance
                          // penalty isn't that big, especially for a one off thing like this
            wakeup_channel_id
        };

        let wakeup_message = CreateMessage::new().content("Hello World!");
        let wakeup_channel = ChannelId::from(wakeup_channel_id);
        let temp = wakeup_channel.send_message(&ctx.http, wakeup_message).await;
        println!("{:?}", temp);
    }
}
