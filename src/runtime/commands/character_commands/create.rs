use std::collections::HashMap;

use crate::{
    runtime::{
        runtime_client::RuntimeClient,
        context_keys
    },
    utils::misc::{
        colour_codes::ColourCode, 
        logging::create_log_message
    },
};

use serenity::{
    all::{CreateEmbedFooter, InputText},
    builder::{
        CreateActionRow, CreateButton, CreateEmbed, CreateInputText, CreateInteractionResponse, CreateInteractionResponseMessage, CreateModal, EditMessage
    }, 
    client::Context, 
    model::application::{
        ActionRowComponent, ButtonStyle, CommandInteraction, ComponentInteraction, InputTextStyle, ModalInteraction
    }
};

pub async fn run( runtime_client: &RuntimeClient, ctx: Context, interaction_data: CommandInteraction ) {
    let invoker_id = interaction_data.user.id.get();

    let response = 'response: {

        // First see if the player already has a character building process running, if so send a
        // notofication
        {
            let context_data_read = ctx.data.read().await;
            let character_building_data = context_data_read.get::<context_keys::CharacterBuildingDataKey>().expect("Key inserted at startup");

            if character_building_data.contains_key(&invoker_id) {

                let embed = CreateEmbed::new()
                    .title("You're already building a character")
                    .description("You can only have one build process running at a time. If you want to start over, cancel your previous process")
                    .colour(ColourCode::Caution.to_embed_colour());

                break 'response CreateInteractionResponseMessage::new().embed(embed);
            }
        }


        // Send stage 0 embed
        let embed = CreateEmbed::new()
            .title("Welcome to character creation!")
            .description("Press `Continue` to advance to the next steps, or `Cancel` at any point in time to stop")
            .colour(ColourCode::Location.to_embed_colour());

        let buttons = CreateActionRow::Buttons(vec![
            CreateButton::new(format!("character|create|continue|{invoker_id}|0"))
                .style(ButtonStyle::Primary)
                .label("Continue"),

            CreateButton::new(format!("character|create|cancel|{invoker_id}|0"))
                .style(ButtonStyle::Secondary)
                .label("Cancel")
        ]);

        CreateInteractionResponseMessage::new()
            .embed(embed)
            .components(vec![buttons])
    };

    let response_payload = interaction_data.create_response(
        &ctx.http,
        CreateInteractionResponse::Message(response)
    );

    if let Err(why) = response_payload.await {
        println!( "{}", create_log_message(
                format!(
                    "Failed to send response: `{}{}{}`",
                    ColourCode::Info,
                    why,
                    ColourCode::Reset
                ),
                ColourCode::Warning
        ))
    }
}


pub async fn handle_component_interaction( mut interaction_data: ComponentInteraction, ctx: Context, split_custom_id: Vec<&str> ) {
    let invoker_id = interaction_data.user.id.get();

    // If the interaction was invoked by somebody who doesn't own the building process, acknowledge
    // the interaction, not worrying about potential failures, and quit early
    if format!("{invoker_id}").as_str() != split_custom_id[3] {
        let _ = interaction_data.create_response(&ctx.http, CreateInteractionResponse::Acknowledge).await;
        return;
    }

    match split_custom_id[2] {
        "continue" => {

            match split_custom_id[4] {
                "0" => {

                    {
                        let mut context_data_write = ctx.data.write().await;
                        let character_building_data = context_data_write
                            .get_mut::<context_keys::CharacterBuildingDataKey>()
                            .expect("Key inserted at startup");

                        character_building_data.insert(invoker_id, (
                                HashMap::new(),
                                *interaction_data.message.clone()
                        ));
                    }

                    //
                    let new_embed = CreateEmbed::new()
                        .title("Lets start with the basics")
                        .description(
                            "Once you press `Continue` you will be prompted for your character's name, species, and backstory"
                        )
                        .footer(CreateEmbedFooter::new("1/5"))
                        .colour(ColourCode::Location.to_embed_colour());

                    let new_buttons = CreateActionRow::Buttons(vec![
                        CreateButton::new(format!("character|create|continue|{invoker_id}|1"))
                            .style(ButtonStyle::Primary)
                            .label("Continue"),

                        CreateButton::new(format!("character|create|cancel|{invoker_id}|1"))
                            .style(ButtonStyle::Secondary)
                            .label("Cancel")
                    ]);

                    let new_message = EditMessage::new()
                        .embed(new_embed)
                        .components(vec![new_buttons]);

                    let edit_message_payload = interaction_data.message.edit(&ctx.http, new_message);
                    if let Err(why) = edit_message_payload.await {
                        println!( "{}", create_log_message(
                                format!(
                                    "Failed to edit message: `{}{}{}`",
                                    ColourCode::Info,
                                    why,
                                    ColourCode::Reset
                                ),
                                ColourCode::Warning
                        ))
                    }

                    let acknowledge_payload = interaction_data.create_response(&ctx.http, CreateInteractionResponse::Acknowledge);
                    if let Err(why) = acknowledge_payload.await {
                        println!( "{}", create_log_message(
                                format!(
                                    "Failed to acknowledge payload: `{}{}{}`",
                                    ColourCode::Info,
                                    why,
                                    ColourCode::Reset
                                ),
                                ColourCode::Warning
                        ))
                    }
                },
                // 0

                "1" => {

                    let fields = vec![
                        CreateActionRow::InputText(CreateInputText::new(
                                InputTextStyle::Short,
                                "Name",
                                "name"
                        ).required(true)),

                        CreateActionRow::InputText(CreateInputText::new(
                                InputTextStyle::Short,
                                "Species",
                                "species"
                        ).required(true)),

                        CreateActionRow::InputText(CreateInputText::new(
                                InputTextStyle::Paragraph,
                                "Backstory",
                                "backstory"
                        ).required(true)),
                    ];

                    let new_modal = CreateModal::new("character|create|1", "Character Creation")
                        .components(fields);

                    let modal_payload = interaction_data.create_response(&ctx.http, CreateInteractionResponse::Modal(new_modal));
                    if let Err(why) = modal_payload.await {
                        println!( "{}", create_log_message(
                                format!(
                                    "Failed to send modal response: `{}{}{}`",
                                    ColourCode::Info,
                                    why,
                                    ColourCode::Reset
                                ),
                                ColourCode::Warning
                        ))
                    }
                },
                // 1

                _ => {}
            }

        },
        // "continue"
        "cancel" => {

            {
                let mut context_data_write = ctx.data.write().await;
                let character_building_data = context_data_write.get_mut::<context_keys::CharacterBuildingDataKey>().expect("Key inserted at runtime");

                character_building_data.remove(&invoker_id);
            }

            let new_embed = CreateEmbed::new()
                .title("Character Creation Cancled")
                .colour(ColourCode::Info.to_embed_colour());

            let new_message = EditMessage::new()
                .embed(new_embed)
                .components(vec![]);

            let response_payload = interaction_data.message.edit(&ctx.http, new_message);
            if let Err(why) = response_payload.await {
                println!( "{}", create_log_message(
                        format!(
                            "Failed to send response: `{}{}{}`",
                            ColourCode::Info,
                            why,
                            ColourCode::Reset
                        ),
                        ColourCode::Warning
                ))
            }


            let acknowledge_payload = interaction_data.create_response(&ctx.http, CreateInteractionResponse::Acknowledge);
            if let Err(why) = acknowledge_payload.await {
                println!( "{}", create_log_message(
                        format!(
                            "Failed to acknowledge interaction: `{}{}{}`",
                            ColourCode::Info,
                            why,
                            ColourCode::Reset
                        ),
                        ColourCode::Warning
                ))
            }
        },
        // "cancel"
        _ => {}
    }
}


pub async fn handle_modal( modal_interaction: ModalInteraction, ctx: Context, split_custom_id: Vec<&str> ) {
    let invoker_id = modal_interaction.user.id.get();

    {
        let mut context_data_write = ctx.data.write().await;
        let character_building_data = context_data_write.get_mut::<context_keys::CharacterBuildingDataKey>().expect("Key inserted at startup");

        for action_row in &modal_interaction.data.components {
            // Our modal only has text fields
            let ActionRowComponent::InputText(ref input_text) = action_row.components[0]
                else { panic!("Recieved enum varaint that isn't of type `InputText`") };

            let user_data = character_building_data
                .get_mut(&invoker_id)
                .expect("At this point the user is in the process of building, hence they should be in the HashMap");

            user_data.0.insert(
                input_text.custom_id.clone(),
                input_text.value.clone().expect("All fields are marked as required")
            );


            // Edit the message
            let new_message = match split_custom_id[2] {
                "1" => {

                    let new_emed = CreateEmbed::new()
                        .title("Next up, where is your character placed in the politics of the world?")
                        .description(
                            "Now you'll be prompted about your character's motivations and alignmment (which faction, if applicable, are they a part of)"
                        )
                        .footer(CreateEmbedFooter::new("2/5"))
                        .colour(ColourCode::Location.to_embed_colour());

                    let new_buttons = CreateActionRow::Buttons(vec![
                        CreateButton::new(format!("character|create|continue|{invoker_id}|2"))
                        .style(ButtonStyle::Primary)
                        .label("Continue"),

                        CreateButton::new(format!("character|create|cancel|{invoker_id}|2"))
                        .style(ButtonStyle::Secondary)
                        .label("Cancel")
                    ]);


                    EditMessage::new()
                        .embed(new_emed)
                        .components(vec![new_buttons])
                },
                // 0

                _ => {panic!("Unimplemented {}", split_custom_id[2])}
            };

            let edit_message_payload = user_data.1.edit(&ctx.http, new_message);
            if let Err(why) = edit_message_payload.await {
                println!( "{}", create_log_message(
                        format!(
                            "Failed to edit message: `{}{}{}`",
                            ColourCode::Info,
                            why,
                            ColourCode::Reset
                        ),
                        ColourCode::Warning
                ));
                return
            }

            let acknowledge_payload = modal_interaction.create_response(&ctx.http, CreateInteractionResponse::Acknowledge);
            if let Err(why) = acknowledge_payload.await {
                println!( "{}", create_log_message(
                        format!(
                            "Failed to acknowledge modal: `{}{}{}`",
                            ColourCode::Info,
                            why,
                            ColourCode::Reset
                        ),
                        ColourCode::Warning
                ))
            }
        }
    }
}

