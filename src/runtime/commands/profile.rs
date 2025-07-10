use crate::{
    runtime::{
        runtime_client::RuntimeClient,
        commands::profile_commands
    },
    utils::misc::{
        logging::create_log_message,
        colour_codes::ColourCode
    }
};

use serenity::{
    
    builder::{
        CreateCommand,
        CreateCommandOption,
    },
    model::application::{
        CommandInteraction,
        CommandOptionType
    },
    client::Context
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


    // Based on the sub command's name, delegate to the correct execution unit
    match sub_command_name.as_str() {
        "register" => profile_commands::register::run( &runtime_client, ctx, interaction_data ).await,    
        "deregister" => profile_commands::deregister::run( &runtime_client, ctx, interaction_data ).await,
        // In the case that an unknown sub command is recived, inform the terminal of this
        // occurance
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
}

