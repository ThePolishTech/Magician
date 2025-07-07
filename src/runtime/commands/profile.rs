use crate::runtime::runtime_client::RuntimeClient;

use serenity::{
    
    model::application::{
        CommandInteraction,
        CommandOptionType
    },
    builder::{
        CreateCommand,
        CreateCommandOption
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


pub fn run( _runtime_client: &RuntimeClient, _ctx: Context, interaction_data: CommandInteraction ) {
    let sub_command_name = &interaction_data
        .data
        .options[0] // Because this command has sub commands, we know that there exists a 0th
                    // option, and that it contains the name of the selected sub command
        .name;

    println!("Recived! `profile::{sub_command_name}`");
}

