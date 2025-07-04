use serenity::{all::{ChannelId, Context, CreateMessage, EventHandler, Ready}, async_trait};
use crate::runtime::{
    runtime_client,
    context_keys
};


#[async_trait]
impl EventHandler for runtime_client::RuntimeClient {
    
    async fn ready( &self, ctx: Context, _ready: Ready) {

        let wakeup_channel_id = *ctx.data.read().await.get::<context_keys::WakeupChannelIdKey>()
            .expect("Wakeup channel id should be stored in Context");


        let wakeup_message = CreateMessage::new().content("Hello World!");
        let wakeup_channel = ChannelId::from(wakeup_channel_id);
        let temp = wakeup_channel.send_message(&ctx.http, wakeup_message).await;
        //println!("{:?}", temp);
    }
}
