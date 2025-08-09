// Attributes
//#![allow(unused_imports)]

// --== MODULE IMPORTS ==-- //
use crate::{
    runtime::{
        context_keys, runtime_client::RuntimeClient, sql_scripts
    },
    utils::misc::{
        colour_codes::ColourCode, 
        logging::create_log_message
    },
};
// ==--


// --== CRATE IMPORTS ==-- //

    // STD & core
        use core::panic;
        use std::collections::HashMap;

    // SERENITY
        use serenity::{
            builder::{
                CreateActionRow, CreateButton, CreateEmbed, CreateEmbedFooter,
                CreateInputText, CreateInteractionResponse, CreateInteractionResponseMessage,
                CreateModal, EditMessage
            }, client::Context, model::application::{
                ActionRowComponent, ButtonStyle, CommandInteraction, ComponentInteraction, InputTextStyle, ModalInteraction
            }
        };
// ==--

// --== HELPER STRUCT DEFINITIONS ==-- //

    #[derive(Debug)]
    #[allow(dead_code)]
    struct ParsedComponentCustomId<'a> {
        command: &'a str,
        sub_command: &'a str,
        label: &'a str,
        user_id: u64,
        stage: u8
    }
    impl ParsedComponentCustomId<'_> {
        pub fn from_vec(split_custom_id: Vec<&str>) -> Result<ParsedComponentCustomId, &'static str> {
            Ok(ParsedComponentCustomId {
                command:     split_custom_id.first().ok_or("`command` field missing")?,
                sub_command: split_custom_id.get(1).ok_or("`sub_command` field missing")?,
                label:       split_custom_id.get(2).ok_or("`label` field missing")?,
                user_id:     split_custom_id.get(3).ok_or("`user_id` field missing")?.parse().or(Err("`couldn't parse `user_id`"))?,
                stage:       split_custom_id.get(4).ok_or("`stage` field missing")?.parse().or(Err("couldn't parse `stage`"))?
            })
        }
    }

    #[derive(Debug)]
    #[allow(dead_code)]
    struct ParsedModalCustomId<'a> {
        command: &'a str,
        sub_command: &'a str,
        stage: u8
    }
    impl ParsedModalCustomId<'_> {
        pub fn from_vec(split_custom_id: Vec<&str>) -> Result<ParsedModalCustomId, &'static str> {
            Ok(ParsedModalCustomId { 
                command:     split_custom_id.first().ok_or("`command` field missing")?,
                sub_command: split_custom_id.get(1).ok_or("`sub_command` field missing")?,
                stage:       split_custom_id.get(2).ok_or("`stage` field missing")?.parse().or(Err("couldn't parse `stage`"))?
            })
        }
    }
// ==--

// --== EMBED AND MODAL STAGE RELATED INFO ==-- //

    // Here we will store the titles and descriptions for embeds depending on their stage. Along
    // with a function generatiing modal field name and IDs

    const EMBED_STAGE_TITLES: [&str; 6] = [
        "Welcome to character createion!",
        "First of all, the basics",
        "Next up, what does your character have a regard for, anything they abhore?",
        "Do they wander with others? Any other extra information?",
        "What drives them? Do they stick with a team? What's their past?",
        "Lastly, what's their class?"
    ];
    const EMBED_STAGE_DESCRIPTIONS: [&str; 6] = [
        "To build your character press  `Start`  when ready, or  `Cancel`  at any time to stop.\n \
        Character creation will occur in stages, with forms popping up whenever you click `Continue` \
        to prompt for your input. Feel free to go at your own pace.",

        "What's your character's name, species, and appearance?",

        "DESC: Large or small, whether it's a quiet moment with a cup of tea that they like, or their \
        archnemesis, which they dispise",

        "If your character is a loner, feel free to type in 'N/A'",

        "What are their Motivations? Do they align themselves to anybody/anything? What's their backstory",

        "\
        - Martial: They fight with weapons lacking magic\
        - Caster: They prefer the magical arts\
        - Half-Caster: They're a mix of both
        "
    ];

    /// Returns the field (Name, ID) for modals of a given stage, and if it should be a paragraph
    ///
    /// # Panics:
    /// This function will panic if you attempt to get field name and IDs from invalid stages
    fn get_modal_fields(stage: &u8) -> Vec<(&'static str, &'static str, bool)> {

        match stage {
            1 => vec![
                ("Name", "name", false),
                ("Species", "species", false),
                ("Appearance", "appearance", true)
            ],
            2 => vec![
                ("Likes", "likes", false),
                ("Dislikes", "dislikes", false)
            ],
            3 => vec![
                ("Companions", "companions", true),
                ("Extra", "extra", true)
            ],
            4 => vec![
                ("Motivations", "motivations", false),
                ("Alignment", "alignment", false),
                ("Backstory", "backstory", true)
            ],
            invalid_stage => panic!("Function recieved invalid stage: `{invalid_stage}`")
        }
    }
// ==--









// --== DOCUMENTATION ==-- //
/*
    This command is used in order to for the user to add their character to the database, however instead of just
    one big modal that could overwhelm new users, we will split it up into segments. Calling the command will reveal
    a 'starting' message which will quickly explain character creation with 'Start' and 'Cancel' buttons. The 'Cancel'
    button when clicked at any stage will clear the cache. 'Start' will move to the next 'stage'.
    
    In stages 1..=4, the message that the user sees, later simply 'message', will contain two buttons 'Continue'
    and 'Cancel'. The latter will act the same, but 'Continue' will spawn a new modal. That modal will inherit the
    stage from the message it is attached to. That modal when recieved will send the next stage

    

    Stages:
    [0] Start
    [1] Name, Species, Appearance
    [2] Likes, Dislikes
    [3] Companions, Extra
    [4] Motivations, Alignment, Backstoty
    [5] Class
    [6] Confirmation


*/
// ==--










//                                      //
// --== HANDLE COMMAND INTERACTION ==-- //
//                                      //
pub async fn run( _runtime_client: &RuntimeClient, ctx: Context, interaction_data: CommandInteraction ) {
    let invoker_id = interaction_data.user.id.get();



    // --== MESGAE BUILDING ==-- //
    
        // First stage is creating the buttons we will add, each will have a specific format:
               /*   command|sub_command|button_label|invoker_id|stage    */
        let start_buttons = CreateActionRow::Buttons(vec![
            CreateButton::new(format!( "character|create|start|{invoker_id}|0" ))
                .style(ButtonStyle::Primary)
                .label("Start!"),

            CreateButton::new(format!( "character|create|cancel|{invoker_id}|0" ))
                .style(ButtonStyle::Secondary)
                .label("Cancel"),
            
        ]);

        // Next the embed
        let start_embed = CreateEmbed::new()
            .title( EMBED_STAGE_TITLES[0] )
            .description( EMBED_STAGE_DESCRIPTIONS[0] )
            .colour(ColourCode::Location.to_embed_colour());  // For a nice purple colour
        
        // And connect the two into a new `ResponseMessage` struct
        let start_message = CreateInteractionResponseMessage::new()
            .components(vec![ start_buttons ])  // It `component` is it's own row of items, as we
            .embed(start_embed);                // only need the one row, a single element vec will
                                                // do
    // ==--



    // Send the response
    let send_start_message = interaction_data.create_response(&ctx.http, CreateInteractionResponse::Message(start_message));
    if let Err(why) = send_start_message.await  { println!( "{}",
        create_log_message(
            format!(
                "Failed to send message: `{}{}{}`",
                ColourCode::Info,
                why,
                ColourCode::Reset
            ),
            ColourCode::Caution
        )
    )}
}










//                                    //
// --== HANDLE MODAL INTERACTION ==-- //
//                                    //
pub async fn handle_modal( modal_interaction: ModalInteraction, ctx: Context, split_custom_id: Vec<&str> ) {
    let invoker_id = modal_interaction.user.id.get();
    let modal_id = ParsedModalCustomId::from_vec(split_custom_id)
        .expect("Modal should have a correctly formed ID");



    // First part is loading the recieved fields' data into the cache
    {
        let mut context_data_writer = ctx.data.write().await;
        let character_building_cache = context_data_writer.get_mut::<context_keys::CharacterBuildingDataKey>()
            .expect("CharacterBuildingDataKey should be inserted in main.rs");

        let user_character_building_cache = character_building_cache.get_mut(&invoker_id)
            .expect(
                "We can only read user data from a modal if they've pressed the button that gets them a modal \
                and this can only be done if they've started building the character, which they have to to get here"
            );

        // Get inputed data from the modal in the form of components
        //
        // Keep in mind that each text field takes up one entire component, so when iterating over
        // them we just have to index the 0th element
        let form_fields = &modal_interaction.data.components;
        for field_action_row in form_fields {
            
            // We need to unpack the component
            let ActionRowComponent::InputText(ref attribute) = field_action_row.components[0] else {
                // This should always be a input text, but to keep the compiler happy:
                continue  // TODO: Log this
            };

            // If for some unexpected reason the user wasn't forced to input text for this field,
            // skip it
            if attribute.value.is_none() {
                continue;  // TODO: Log this
            }

            user_character_building_cache.0.insert(
                attribute.custom_id.clone(),
                attribute.value.clone().unwrap()
            );
        }

    }  // context_data_writer lock

    let next_message = match &modal_id.stage {
        1..=3 => {

            let next_embed: usize = (modal_id.stage+1).into();

            let buttons = CreateActionRow::Buttons(vec![
                CreateButton::new(format!( "character|create|continue|{invoker_id}|{next_embed}" ))
                    .style(ButtonStyle::Primary)
                    .label("Continue"),

                CreateButton::new(format!( "character|create|cancel|{invoker_id}|{next_embed}" ))
                    .style(ButtonStyle::Secondary)
                    .label("Cancel")
            ]);

            let embed = CreateEmbed::new()
                .title( EMBED_STAGE_TITLES[next_embed] )
                .description( EMBED_STAGE_DESCRIPTIONS[next_embed] )
                .footer(CreateEmbedFooter::new(format!( "{next_embed}/5" )))
                .colour(ColourCode::Location.to_embed_colour());

            EditMessage::new()
                .components(vec![ buttons ])
                .embed(embed)

        },

        4 => {  // Next message is the one that asks the user for their character's class

            let buttons = CreateActionRow::Buttons(vec![
                CreateButton::new(format!( "character|create|martial|{invoker_id}|5" ))
                    .style(ButtonStyle::Primary)
                    .label("Martial"),

                CreateButton::new(format!( "character|create|half-caster|{invoker_id}|5" ))
                    .style(ButtonStyle::Primary)
                    .label("Half-Caster"),

                CreateButton::new(format!( "character|create|caster|{invoker_id}|5" ))
                    .style(ButtonStyle::Primary)
                    .label("Caster"),

                CreateButton::new(format!( "character|create|cancel|{invoker_id}|5" ))
                    .style(ButtonStyle::Secondary)
                    .label("Cancel")
            ]);

            let embed = CreateEmbed::new()
                .title( EMBED_STAGE_TITLES[5] )
                .description( EMBED_STAGE_DESCRIPTIONS[5] )
                .footer(CreateEmbedFooter::new("5/5"))
                .colour(ColourCode::Location.to_embed_colour());

            EditMessage::new()
                .components(vec![ buttons ])
                .embed(embed)
        }

        unknown_stage => panic!("Recieved modal from a stage that shouldn't give a modal, `{unknown_stage}`")  // TODO: Proper log
    };
    // Next up edit the original message, using the cached `Message` struct
    {
        let context_data_read = ctx.data.write().await;
        let character_building_cache = context_data_read
            .get::<context_keys::CharacterBuildingDataKey>()
            .expect("Key should be inserted in main.rs");

        // Get a reference to the `Message`
        let mut building_message = character_building_cache.get(&invoker_id)
            .expect(
                "This code should only be available when the user is building a character, \
                     therefore in this hashmap"
            )
            .1
            .clone(); // Get the message

        let edit_message = building_message.edit(&ctx.http, next_message);
        if let Err(why) = edit_message.await  { println!( "{}", create_log_message(
                format!(
                    "Failed to edit message: `{}{}{}`",
                    ColourCode::Info,
                    why,
                    ColourCode::Reset
                ),
                ColourCode::Caution
        ) )}
    }

    let send_acknowledgement = modal_interaction.create_response(&ctx.http, CreateInteractionResponse::Acknowledge);
    if let Err(why) = send_acknowledgement.await  { println!( "{}", create_log_message(
            format!(
                "Failed to send acknowledge: `{}{}{}`",
                ColourCode::Info,
                why,
                ColourCode::Reset
            ),
            ColourCode::Caution
    ) )}
}










//                                        //
// --== HANDLE COMPONENT INTERACTION ==-- //
//                                        //
pub async fn handle_component( runtime_client: &RuntimeClient, component_interaction: ComponentInteraction, ctx: Context, split_custom_id: Vec<&str> ) {
    let invoker_id  = component_interaction.user.id.get();
    let invoker_tag = component_interaction.user.tag();
    let component_id = ParsedComponentCustomId::from_vec(split_custom_id)
        .expect("Component should have a correctly formed ID");
    
    // First of all, we only want the user who initialised the character building proccess (aka
    // proccess) to be the one interacting with it. So each time we recieve an interaction we will
    // match the interacting user (invoker) with the button's saved user ID. If different we shall
    // acknowledge the interaction and return early
    if invoker_id != component_id.user_id {
        let send_acknowledgement = component_interaction.create_response(&ctx.http, CreateInteractionResponse::Acknowledge);
        // Lets not even bother logging any errors, not worth the lines of code
        let _ignored = send_acknowledgement.await;
    }



    // Following this, from this point onwards we are making the assumption that any and all
    // component interactions we recieve must be buttons. This is because all of the components
    // addded to the message are all buttons
    


    // Each button has its own unique interaction, dependant on it's label. Henceforth we shall
    // match on it. Let us keep in mind that the implementation of these interactions will get
    // quite long, so lets annotate the ending brackets of each case and split them via their
    // whitespaces.
    //
    // However all of these implementations will send some form of interaction. Henceforth we shall
    // have the match expression return an `InteractionResponse` struct
    let interaction_response = match component_id.label {
        "cancel" => {

            {
                let mut context_data_writer = ctx.data.write().await;
                let character_building_cache = context_data_writer
                    .get_mut::<context_keys::CharacterBuildingDataKey>()
                    .expect("Key inserted in main.rs");

                character_building_cache.remove(&invoker_id);
            }

            let new_embed_buttons = CreateActionRow::Buttons(vec![
                CreateButton::new(format!( "character|create|dismiss|{invoker_id}|{}", component_id.stage ))
                    .style(ButtonStyle::Danger)
                    .label("Dismiss")
            ]);

            let new_embed = CreateEmbed::new()
                .title("Character Creation Cancelled")
                .colour(ColourCode::Info.to_embed_colour());

            CreateInteractionResponse::UpdateMessage(
                CreateInteractionResponseMessage::new()
                    .components(vec![new_embed_buttons])
                    .embed(new_embed)
            )

        }, // "cancel"



        "dismiss" => {
            // This isn't a direct response as we are deleting the base message. In this case we
            // don't have a specific response to give so we will early return
            let delete_message = component_interaction.message.delete(&ctx.http);
            if let Err(why) = delete_message.await {  println!( "{}", 
                create_log_message(
                    format!(
                        "Failed to delete message: `{}{}{}`",
                        ColourCode::Info,
                        why,
                        ColourCode::Reset
                    ),
                    ColourCode::Caution
                )
            )}
            return;
        }, // "dismiss"



        "start" => 'start_button_response: {

            // Here's the state we are in: The user called this called the base `character_create`
            // command and has pressed 'Start!' to start building their character. But we need to
            // check if they're already in the process of building one, and if so, break with an
            // error message

            // We need to access data locked behind a mutex here. Just to be safe, it's best
            // practice to do so in a scope in order to drop the lock as soon as possible.
            // We'll do that here
            {
                let mut context_data_writer = ctx.data.write().await;  // As if we don't break early
                                                                       // we'll be inserting some data
                let character_building_cache =
                    context_data_writer.get_mut::<context_keys::CharacterBuildingDataKey>()
                    .expect("Character Building Cache key should be inerted in main.rs");



                // Here we preform our check for pre-existing building proccesses asigned to the
                // user who clicked 'start'
                if character_building_cache.contains_key(&invoker_id) {
                    // If the execution reached this case, it means the user has attempted to start
                    // a new character building proccess, therefore we need to notify them that we
                    // cannot have two running at once.
                    //
                    // To start off we need to construct our error embed, then create a new 
                    // InteractionResponseMessage that will hold it, and that will clear all
                    // buttons
                    let error_buttons = CreateActionRow::Buttons(vec![
                        CreateButton::new(format!( "character|create|dismiss|{invoker_id}|0" ))
                            .style(ButtonStyle::Danger)
                            .label("Dismiss")

                    ]);

                    let error_embed = CreateEmbed::new()
                        .title("You're already building a character")
                        .description(
                            "Building more than one character at a time is not currently supported. \n\
                            If this becomes a common grievance, please open an issue on the bot's github repo"
                        )
                        .colour(ColourCode::Error.to_embed_colour());

                    break 'start_button_response CreateInteractionResponse::UpdateMessage(
                        CreateInteractionResponseMessage::new()
                            .components(vec![ error_buttons ])
                            .embed(error_embed)
                        );
                }



                // Ok, if we've reached this point it means we are good to start building the
                // user's character. To start lets initialise a cache where we will hold all of
                // their character's information, along with a clone of the message the start
                // button is attached to. This is to allow modal interactions to edit the message
                // to the next stage upon submission
                character_building_cache.insert(invoker_id,
                    ( HashMap::new(), *component_interaction.message.clone() )
                );
            
            }  // context_data_writer lock



            // Now we must move towards the next stage, in this instance 'Stage 1'
            let stage_1_buttons = CreateActionRow::Buttons(vec![
                CreateButton::new(format!( "character|create|continue|{invoker_id}|1" ))
                    .style(ButtonStyle::Primary)
                    .label("Continue"),

                CreateButton::new(format!( "character|create|cancel|{invoker_id}|1" ))
                    .style(ButtonStyle::Secondary)
                    .label("Cancel")

            ]);

            let stage_1_embed = CreateEmbed::new()
                .title( EMBED_STAGE_TITLES[1] )
                .description( EMBED_STAGE_DESCRIPTIONS[1] )
                .footer(CreateEmbedFooter::new("1/5"))
                .colour(ColourCode::Location.to_embed_colour());

            CreateInteractionResponse::UpdateMessage(
                CreateInteractionResponseMessage::new()
                    .components(vec![ stage_1_buttons ])
                    .embed(stage_1_embed)
            )

        }, // "start"



        "continue" => {

            // We need to send a modal


            let mut text_fields = vec![];

            // Vec<(field_name, field_id, is_paragraph)>
            let field_data_vec = get_modal_fields(&component_id.stage);

            for (field_name, field_id, is_paragraph) in field_data_vec {

                let style = if is_paragraph {
                    InputTextStyle::Paragraph
                } else {
                    InputTextStyle::Short
                };

                text_fields.push( CreateActionRow::InputText(
                    CreateInputText::new(style, field_name, field_id)
                        .required(true)
                ))
            }


            let modal = CreateModal::new(format!( "character|create|{}", component_id.stage ), "Character Building")
                .components(text_fields);



            CreateInteractionResponse::Modal(modal)

        }, // "continue"



        class if ["martial", "half-caster", "caster"].contains(&class) => {

            {
                let mut context_data_writer = ctx.data.write().await;
                let character_building_cache = context_data_writer
                    .get_mut::<context_keys::CharacterBuildingDataKey>()
                    .expect("Key inserted at main.rs");

                character_building_cache.get_mut(&invoker_id)
                    .expect("User is in the process of building their character")
                    .0
                    .insert("class".to_string(), class.to_string());
            }

            let buttons = CreateActionRow::Buttons(vec![
                CreateButton::new(format!( "character|create|finish|{invoker_id}|6" ))
                    .style(ButtonStyle::Success)
                    .label("Finish"),

                CreateButton::new(format!( "character|create|cancel|{invoker_id}|6" ))
                    .style(ButtonStyle::Secondary)
                    .label("Cancel")
            ]);

            let embed = CreateEmbed::new()
                .title("Finish")
                .description("Finish")
                .colour(ColourCode::Location.to_embed_colour());

            CreateInteractionResponse::UpdateMessage( CreateInteractionResponseMessage::new()
                .components(vec![ buttons ])
                .embed(embed)
            )
        }, // class



        "finish" => {

            // First we need to cast the cache to a `Character` struct, then use it to insert a new
            // character into the database on the user's behalf

            let new_message = {
                let mut context_data_writer = ctx.data.write().await;
                let character_building_cache = context_data_writer
                    .get_mut::<context_keys::CharacterBuildingDataKey>()
                    .expect("Key inserted in main.rs");

                let users_cache = character_building_cache
                    .get_mut(&invoker_id)
                    .expect("User will be in cache when finalising character creation");



                let built_character = sql_scripts::characters::Character::from_hashmap_cache(&users_cache.0);
                match built_character {
                    Ok(character) => {

                        let insert_character_query = sql_scripts::characters::insert_character(
                            &runtime_client.database_connection,
                            invoker_id,
                            character.clone()
                        );
                        match insert_character_query.await {
                            Ok(()) => {

                                // Log the success
                                println!("{}", create_log_message(
                                        format!(
                                            "`{}{}:#{}{}` Has inserted a new character `{}{}{}`!",
                                            ColourCode::Info,
                                            invoker_tag,
                                            invoker_id,
                                            ColourCode::Reset,
                                            ColourCode::Info,
                                            character.name,
                                            ColourCode::Reset
                                        ),
                                        ColourCode::Success
                                ));

                                let finish_embed = CreateEmbed::new()
                                    .title(format!( "{} Has been successfully added to your characters!", character.name))
                                    .description("You may now use them!")
                                    .colour(ColourCode::Success.to_embed_colour());

                                let finish_buttons = CreateActionRow::Buttons(vec![
                                    CreateButton::new(format!( "character|create|dismiss|{invoker_id}|7" ))
                                    .style(ButtonStyle::Success)
                                    .label("Dismiss")
                                ]);

                                character_building_cache.remove(&invoker_id); // We no longer need
                                                                              // this

                                CreateInteractionResponseMessage::new()
                                    .embed(finish_embed)
                                    .components(vec![ finish_buttons ])
                            }
                            Err(why) => {
                                println!("{}", create_log_message(
                                        format!(
                                            "Failed to insert a character to the database: `{}{}{}`",
                                            ColourCode::Info,
                                            why,
                                            ColourCode::Reset
                                        ),
                                        ColourCode::Error
                                ));



                                let insert_error_embed = CreateEmbed::new()
                                    .title("An error occured while inserting your character. Try again?")
                                    .description(format!("Error: \n`{why}`"))
                                    .colour(ColourCode::Error.to_embed_colour());

                                CreateInteractionResponseMessage::new()
                                    .embed(insert_error_embed)
                            }
                        }
                    },
                    Err(why) => {
                        // This case happens when there's an issue parsing the character cache into
                        // the character struct
                        let character_cast_error_embed = CreateEmbed::new()
                            .title(
                                "Failed to prepare character for database insert. This shouldn't happen in production, consider yourself \
                                lucky in your lack of luck"
                            )
                            .description(format!("Error: \n`{why}`"))
                            .colour(ColourCode::Error.to_embed_colour());

                        CreateInteractionResponseMessage::new()
                            .embed(character_cast_error_embed)
                    }
                }
            };
            CreateInteractionResponse::UpdateMessage(new_message)

        }, // "finish"



        unknown_label => panic!(
            "{}character_create::handle_component{}: Recieved unknown buttom label: {}{unknown_label}{}",
            ColourCode::Location, ColourCode::Reset,
            ColourCode::Info,     ColourCode::Reset
        )
    }; // let edited_message = match component_id.label {...};



    let edit_message = component_interaction.create_response(&ctx.http, interaction_response);
    if let Err(why) = edit_message.await  { println!( "{}", 
        create_log_message(
            format!(
                "Failed to edit message: `{}{}{}`",
                ColourCode::Info,
                why,
                ColourCode::Reset
            ),
            ColourCode::Caution
        )

    )}
}

