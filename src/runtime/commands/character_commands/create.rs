use crate::{
    runtime::{context_keys, runtime_client::RuntimeClient, sql_scripts},
    utils::misc::{colour_codes::ColourCode, logging::create_log_message},
};

use serenity::{
    builder::{CreateActionRow, CreateButton, CreateEmbed, CreateModal, EditMessage, CreateInputText, CreateInteractionResponse, CreateInteractionResponseMessage},
    client::Context,
    model::application::{ComponentInteraction, CommandInteraction, ButtonStyle, InputTextStyle}
};

pub async fn run(
    runtime_client: &RuntimeClient,
    ctx: Context,
    interaction_data: CommandInteraction,
) {
    let invoker_id  = interaction_data.user.id.get();

    let response = 'response: {
        // Check if the user is already building a character
        {
            let mut context_data = ctx.data.write().await;
            let character_building_hashmap = context_data.get_mut::<context_keys::CharacterBuildingDataKey>()
                .expect("Key already inserted at startup");

            if character_building_hashmap.contains_key(&invoker_id) {
                break 'response CreateInteractionResponseMessage::new().embed(
                    CreateEmbed::new()
                        .title("You're already building a character")
                        .description("Consider canceling your active build")
                        .colour(ColourCode::Caution.to_embed_colour())
                )
            }

            character_building_hashmap.insert(invoker_id, context_keys::CharacterCreationData::new());
        }

        let buttons = CreateActionRow::Buttons(vec![
            CreateButton::new(format!("character|create|continue|{invoker_id}|0"))
                .style(ButtonStyle::Primary)
                .label("Continue"),     // Buttons require an ID when created, this allows us to
                                        // embed some extra information, for instance, which
                                        // command created it, what function it serves, the ID of
                                        // the user who caused it's creation, and the step to which
                                        // it belongs to

            CreateButton::new(format!("character|create|cancel|{invoker_id}|0"))
                .style(ButtonStyle::Secondary)
                .label("Cancel")
        ]);

        CreateInteractionResponseMessage::new()
            .embed(CreateEmbed::new()
                .title("Temp")
                .colour(ColourCode::Location.to_embed_colour())
            )
            .components(vec![buttons]) 
    }; // let response_embed = {...}

    let response = CreateInteractionResponse::Message(response);
    let response_payload = interaction_data.create_response(&ctx.http, response);

    if let Err(response_send_err) = response_payload.await {
        println!( "{}", create_log_message(
                format!(
                    "{}character::create{}: Failed to send response: `{}{}{}`",
                    ColourCode::Location,
                    ColourCode::Reset,
                    ColourCode::Info,
                    response_send_err,
                    ColourCode::Reset
                ),
                ColourCode::Error
        ));
        return;
    }
}


pub async fn handle_component_interaction( mut interaction_data: ComponentInteraction, ctx: Context, split_custom_id: Vec<&str> ) {
    let invoker_id = interaction_data.user.id.get();

    if split_custom_id.get(2).is_none() || split_custom_id.get(3).is_none() || split_custom_id.get(4).is_none() {
        println!( "{}", create_log_message(
                format!(
                    "{}character::create{}: Recieved malformed interaction: `{}{}{}`",
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

    match split_custom_id[2] {
        "continue" => {
            // If somebody else but the user who invoked the parent command interacts, just
            // acknowledge but don't bother doing anything
            if split_custom_id[3].parse().unwrap_or(0) != interaction_data.user.id.get() {
                let _ = interaction_data.create_response(&ctx.http, CreateInteractionResponse::Acknowledge).await;
                return;
            }


            // --== MODAL LAUNCHING ==-- //
                
                // Launch the right modals based on the current stage
                let response_modal = match split_custom_id[4].parse::<u8>().expect("Stage should be a number") {
                    0 => {
                        let fields = vec![
                            CreateActionRow::InputText(
                                CreateInputText::new(
                                    InputTextStyle::Short,
                                    "name",
                                    "name"
                                ).required(true)
                            ),

                            CreateActionRow::InputText(
                                CreateInputText::new(
                                    InputTextStyle::Short,
                                    "species",
                                    "species"
                                ).required(true)
                            ),

                            CreateActionRow::InputText(
                                CreateInputText::new(
                                    InputTextStyle::Paragraph,
                                    "backstory",
                                    "backstory"
                                )
                            )
                        ];


                        CreateModal::new(format!("character|create|form|{invoker_id}|0"), String::from("Character Creation"))
                            .components(fields)
                    },
                    _ => return
                };
            // ==--
                let modal_send_payload = interaction_data.create_response(&ctx.http, CreateInteractionResponse::Modal(response_modal));
                let _ = modal_send_payload.await;
                //let _ = interaction_data.create_response(&ctx.http, CreateInteractionResponse::Acknowledge).await;
        },

        "cancel" => {
            let new_embed = CreateEmbed::new()
                .title("Character creation cancelled")
                .colour(ColourCode::Info.to_embed_colour());

            let new_message = EditMessage::new()
                .embed(new_embed)
                .components(vec![]);

            {
                let mut context_data = ctx.data.write().await;
                let character_building_hashmap = context_data.get_mut::<context_keys::CharacterBuildingDataKey>()
                    .expect("Key already inserted at startup");

                character_building_hashmap.remove(&invoker_id);
            }

            let acknowledge_payload  = interaction_data.create_response(&ctx.http, CreateInteractionResponse::Acknowledge);
            if let Err(why) = acknowledge_payload.await {
                println!( "{}", create_log_message(
                        format!(
                            "{}character::create::handle_component_interaction{} Failed to send acknowlegment: `{}{}{}`",
                            ColourCode::Location,
                            ColourCode::Reset,
                            ColourCode::Info,
                            why,
                            ColourCode::Reset
                        ),
                        ColourCode::Warning
                ) )

            }

            let edit_message_payload = interaction_data.message.edit(&ctx.http, new_message);
            if let Err(why) = edit_message_payload.await {
                println!( "{}", create_log_message(
                        format!(
                            "{}character::create::handle_component_interaction{} Failed to send cancellation response: `{}{}{}`",
                            ColourCode::Location,
                            ColourCode::Reset,
                            ColourCode::Info,
                            why,
                            ColourCode::Reset
                        ),
                        ColourCode::Warning
                ) )
            }
        },
        a => {
            println!("{a}")
        }
    }
}

