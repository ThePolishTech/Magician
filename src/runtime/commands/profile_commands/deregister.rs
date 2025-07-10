use crate::{
    runtime::{runtime_client::RuntimeClient, sql_scripts},
    utils::misc::{colour_codes::ColourCode, logging::create_log_message},
};

use serenity::{
    builder::{CreateEmbed, CreateInteractionResponse, CreateInteractionResponseMessage},
    client::Context,
    model::application::CommandInteraction,
};

pub async fn run(
    runtime_client: &RuntimeClient,
    ctx: Context,
    interaction_data: CommandInteraction,
) {
    let invoker_id = interaction_data.user.id.get();
    let invoker_tag = interaction_data.user.tag();

    let response_embed = 'response_embed: {

        // Before we can go about removing the user's profile entry we gotta verify some conditions
        // are met. Mainly that we don't attempt any actions that could lead to a primary key
        // constraint being broken

        // --== 1) CHECK IF USER IN DB ==-- //

            let detect_user_query = sql_scripts::discord_users::get_by_user_id(&runtime_client.database_connection, invoker_id);

            let is_user_in_db = match detect_user_query.await {
                Ok(query_result) => query_result.is_some(),

                Err(query_err) => {
                    // Our query to detect if the invoking user is already in the database didn't work for
                    // some reason. So let's inform the invoker and log this error

                    println!( "{}", create_log_message(
                            format!(
                                "{}profile::deregister::detect_user_query{}: Failed to check if user is in database: `{}{}{}`",
                                ColourCode::Location,
                                ColourCode::Reset,
                                ColourCode::Info,
                                query_err,
                                ColourCode::Reset
                            ),
                            ColourCode::Error
                    ));

                    break 'response_embed CreateEmbed::new()
                        .title("An unexpected error occured :(")
                        .description(format!("We were unable to check if you're already in the database\n{query_err}"))
                        .colour(ColourCode::Error.to_embed_colour());
                }
            };
            
            // Break if test failed
            if !is_user_in_db {
                break 'response_embed CreateEmbed::new()
                    .title("You're not in the database")
                    .colour(ColourCode::Info.to_embed_colour())
            }
        // ==--


            // All tests have passed, it is now safe to remove the user's profile entry


        // --== DELETE USER ==-- //
            
            let remove_user_query = sql_scripts::discord_users::remove_user(&runtime_client.database_connection, invoker_id);

            match remove_user_query.await {
                Ok(_) => {
                    println!( "{}", create_log_message(
                            format!(
                                "Sccessfully removed user `{}{}:#{}{}` from the database",
                                ColourCode::Info,
                                invoker_tag,
                                invoker_id,
                                ColourCode::Reset
                            ),
                            ColourCode::Success
                    ));

                    break 'response_embed CreateEmbed::new()
                        .title("Successfully removed you from the database")
                        .colour(ColourCode::Success.to_embed_colour())
                },
                Err(query_err) => {
                    println!( "{}", create_log_message(
                            format!(
                                "Failed to add user `{}{}:#{}{}` to the database: `{}{}{}`",
                                ColourCode::Info,
                                invoker_tag,
                                invoker_id,
                                ColourCode::Reset,
                                ColourCode::Info,
                                query_err,
                                ColourCode::Reset
                            ),
                            ColourCode::Error
                    ));

                    break 'response_embed CreateEmbed::new()
                        .title("Failed to add you to the database :(")
                        .description(format!("`{query_err}`"))
                        .colour(ColourCode::Error.to_embed_colour())
                }

            }
        // ==--

    }; // let response_embed = {...}

    let response = CreateInteractionResponse::Message(
        CreateInteractionResponseMessage::new().embed(response_embed)
    );
    let response_payload = interaction_data.create_response(&ctx.http, response);

    if let Err(response_send_err) = response_payload.await {
        println!( "{}", create_log_message(
                format!(
                    "{}profile::deregister{}: Failed to send response: `{}{}{}`",
                    ColourCode::Location,
                    ColourCode::Reset,
                    ColourCode::Info,
                    response_send_err,
                    ColourCode::Reset
                ),
                ColourCode::Error
        ));
    }

}

