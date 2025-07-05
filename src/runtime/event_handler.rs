// --== MODULE IMPORTS ==-- //

    use std::fmt::Pointer;

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
            all::{CreateEmbed, Timestamp}, async_trait, builder::CreateMessage, client::{
                Context,
                EventHandler
            }, model::{
                gateway::Ready, id::ChannelId
            }
        };
// ==--


#[async_trait]
impl EventHandler for runtime_client::RuntimeClient {
    
    async fn ready( &self, ctx: Context, _ready: Ready) {

        // --== CREATE & SEND WAKEUP MESSAGE ==-- //
        
            // The startup process went swimmingly, let us not be shy and send a wakeup message. It
            // will be a simple embed whose title is simply "Magician Online", and which contains a
            // timestamp defining when Magician here came online
            
            // First of all, we need to read the channel id towards which, we will send the wakeup
            // message. It is read at startup from the config file and stored here in our Context
             let wakeup_channel_id = *ctx.data.read().await.get::<context_keys::WakeupChannelIdKey>()
                .expect("Wakeup channel id should be stored in Context");

            // Next up, we need to define the colour and timestamp to be used by our embed, and
            // additionally, the channel towards which we shall fire it
            let ( embed_colour, embed_timestamp, wakeup_channel ) = (
                ColourCode::Info.to_embed_colour(),
                Timestamp::now(),
                ChannelId::from(wakeup_channel_id)
            );

            // Then we put all of those together into an embed builder
            let wakeup_embed = CreateEmbed::new()
                .title("Magician Online")
                .colour(embed_colour)
                .timestamp(embed_timestamp);

            // And pack it into a message
            let wakeup_message = CreateMessage::new()
                .embed(wakeup_embed);

            // And send it down the wire, reporting any errors should they occur
            if let Err(why) = wakeup_channel.send_message(&ctx.http, wakeup_message).await {
                println!( "{}", create_log_message(
                    format!(
                        "`{}ready{}`: Failed to send wakeup message: `{}{}{}`",
                        ColourCode::Location,
                        ColourCode::Reset,
                        ColourCode::Info,
                        why,
                        ColourCode::Reset
                    ),
                    ColourCode::Error
                ));
            }
        // ==--

        // Oh and also, lets tell the terminal that we are online
        println!( "{}",
            create_log_message("Bot Online!", ColourCode::Info)
        );
    }
}

