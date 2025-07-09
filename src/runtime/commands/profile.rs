use crate::{
    runtime::{
        sql_scripts,
        runtime_client::RuntimeClient
    },
    utils::misc::{
        logging::create_log_message,
        colour_codes::ColourCode
    }
};

use serenity::{
    
    all::{CreateInteractionResponse, CreateInteractionResponseMessage}, builder::{
        CreateCommand,
        CreateCommandOption,

        CreateEmbed
    }, client::Context, model::application::{
        CommandInteraction,
        CommandOptionType
    }
};

pub fn build() -> CreateCommand {
    CreateCommand::new("profile")
        .description("Add or remove your discord profile from the database")
        .set_options(vec![
                CreateCommandOption::new(
                    CommandOptionType::SubCommand,
                    "register",
                    "Add your discord profile to the database"
                ),
                CreateCommandOption::new(
                    CommandOptionType::SubCommand,
                    "deregister",
                    "Remove your discord profile from the database"
                )
        ])
}


pub async fn run( runtime_client: &RuntimeClient, ctx: Context, interaction_data: CommandInteraction ) {
    let sub_command_name = &interaction_data
        .data
        .options[0] // Because this command has sub commands, we know that there exists a 0th
                    // option, and that it contains the name of the selected sub command
        .name;

    let invoker_id  = &interaction_data.user.id.get();
    let invoker_tag = &interaction_data.user.tag();


    let return_embed = 'return_embed: {



        match sub_command_name.as_str() {
            "register" => {
                let query_result = sql_scripts::discord_users::add_user(
                    &runtime_client.database_connection,
                    *invoker_id
                ).await;

                match query_result {
                    Ok(_) => {
                        println!( "{}", create_log_message(
                            format!(
                                "User: `{}{}: {}{}` added to database!",
                                ColourCode::Info,
                                invoker_tag,
                                invoker_id,
                                ColourCode::Reset
                            ),
                            ColourCode::Success
                        ));
                        break 'return_embed CreateEmbed::new()
                            .title("Successfully added you to the database!")
                            .colour(ColourCode::Success.to_embed_colour());
                    },
                    Err(why) => {
                        println!( "{}", create_log_message(
                            format!(
                                "Failed to add user: `{}{}: {}{}` to the database: `{}{}{}`",
                                ColourCode::Info,
                                invoker_tag,
                                invoker_id,
                                ColourCode::Reset,
                                ColourCode::Info,
                                why,
                                ColourCode::Reset
                            ),
                            ColourCode::Error
                        ));
                        break 'return_embed CreateEmbed::new()
                            .title("Failed to add you to the database :(")
                            .description(format!("`{why}`"))
                            .colour(ColourCode::Error.to_embed_colour());
                    }
                };
                
            },
            // is profile::register

            "deregister" => {

            },
            // is profile::deregister

            unimplemented_subcommand => println!( "{}", create_log_message(
                format!(
                    "Recieved unknown `{}profile{}` sub command: `{}{}{}`",
                    ColourCode::Info,
                    ColourCode::Reset,
                    ColourCode::Info,
                    unimplemented_subcommand,
                    ColourCode::Reset
                ),
                ColourCode::Warning
            ))
        }

        CreateEmbed::new()
            .title("unimplemented for now")
            .colour(ColourCode::Warning.to_embed_colour())
    };

    // Next up, we package the embed into a message payload
    let response_message = CreateInteractionResponseMessage::new().embed(return_embed);
    let response         = CreateInteractionResponse::Message( response_message );

    // And send it, report if errors occur
    if let Err(why) = interaction_data.create_response(&ctx.http, response).await {
        println!( "{}", create_log_message(
            format!(
                "{}profile{}: Failed to send response message: `{}{}{}`",
                ColourCode::Location,
                ColourCode::Reset,
                ColourCode::Info,
                why,
                ColourCode::Reset
            ),
            ColourCode::Warning
        ))
    }
}

