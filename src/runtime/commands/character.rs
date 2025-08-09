#![allow(clippy::single_match)]

use core::panic;

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
        ModalInteraction,
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


pub async fn handle_component_interaction( runtime_client: &RuntimeClient, interaction_data: ComponentInteraction, ctx: Context, split_custom_id: Vec<&str> ) {
    match split_custom_id[1] {
        "create" => character_commands::create::handle_component(runtime_client, interaction_data, ctx, split_custom_id).await,
        uknown_component => panic!("character uknown_component: {uknown_component}")
    }
}


pub async fn handle_modal( modal_interaction: ModalInteraction, ctx: Context, split_custom_id: Vec<&str> ) {
    match split_custom_id[1] {
        "create" => character_commands::create::handle_modal(modal_interaction, ctx, split_custom_id).await,
        unknown_modal => panic!("Unknown modal: `{unknown_modal}`")
    }
}

