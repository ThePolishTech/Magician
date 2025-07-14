use crate::{
    runtime::{
        commands::character_commands, runtime_client::RuntimeClient
    },
    utils::misc::{
        colour_codes::ColourCode, logging::create_log_message
    }
};

use serenity::{
    
    builder::{
        CreateCommand,
        CreateCommandOption,
    },
    model::application::{
        ComponentInteraction,
        CommandInteraction,
        CommandOptionType
    },
    client::Context
};

pub fn build() -> CreateCommand {
    CreateCommand::new("character")
        .description("Manage your characters")
        .set_options(vec![
                CreateCommandOption::new(
                    CommandOptionType::SubCommand,
                    "create",
                    "Construct your new character"
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
        "create" => character_commands::create::run(runtime_client, ctx, interaction_data).await,
        // In the case that an unknown sub command is recived, inform the terminal of this
        // occurance
        unimplemented_subcommand => println!( "{}", create_log_message(
                format!(
                    "{}character{}: Recieved unknown sub command: `{}{}{}`",
                    ColourCode::Location,
                    ColourCode::Reset,
                    ColourCode::Info,
                    unimplemented_subcommand,
                    ColourCode::Reset
                ),
                ColourCode::Warning
        ))
    }
}


pub async fn handle_component_interaction( interaction_data: ComponentInteraction, ctx: Context, split_custom_id: Vec<&str> ) {
    if split_custom_id.get(1).is_none() {
        println!( "{}", create_log_message(
                format!(
                    "{}commands::character::handle_component_interaction{}: Recieved malformed custom_id: `{}{}{}`",
                    ColourCode::Location,
                    ColourCode::Reset,
                    ColourCode::Info,
                    interaction_data.data.custom_id,
                    ColourCode::Reset
                ),
                ColourCode::Warning
        ));
        return;
    }

    match split_custom_id[1] {
        "create" => character_commands::create::handle_component_interaction(interaction_data, ctx, split_custom_id).await,
        // In case an unknown component interaction is recieved, notify the terminal
        unknown_component_interaction => println!( "{}", create_log_message(
                format!(
                    "{}commands::character{}: Recieved unknown component interaction: `{}{}{}`",
                    ColourCode::Location,
                    ColourCode::Reset,
                    ColourCode::Info,
                    unknown_component_interaction,
                    ColourCode::Reset
                ),
                ColourCode::Warning
        ))
    }

}
