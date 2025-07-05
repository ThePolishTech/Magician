// --== MODULE IMPORTS ==-- //

    use crate::{
        runtime::{
            context_keys,
            runtime_client
        }, 
        utils::misc::{
            logging::create_log_message,
            colour_codes::ColourCode
        }
    };
// ==--

// --== CRATE IMPORTS ==-- //

    // SERENITY
        use serenity::{
            async_trait,
            model::{
                id::ChannelId,
                gateway::Ready
            },
            client::{
                Context,
                EventHandler
            },
            builder::CreateMessage,
        };
// ==--


#[async_trait]
impl EventHandler for runtime_client::RuntimeClient {
    
    async fn ready( &self, ctx: Context, _ready: Ready) {

        // --== CREATE & SEND WAKEUP MESSAGE ==-- //
            
            // First of all, we need to read the channel id towards which, we will send the wakeup
            // message
            let wakeup_channel_id = *ctx.data.read().await.get::<context_keys::WakeupChannelIdKey>()
                .expect("Wakeup channel id should be stored in Context");

            // 

            let wakeup_message = CreateMessage::new().content("Hello World!");
            let wakeup_channel = ChannelId::from(wakeup_channel_id);
            let _temp = wakeup_channel.send_message(&ctx.http, wakeup_message).await;

            println!( "{}",
                create_log_message("Bot Online!", ColourCode::Info)
            );
        // ==--
    }
}
