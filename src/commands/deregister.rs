use rusqlite::OptionalExtension;
use serenity::{
    all::{CreateInteractionResponse, CreateModal}, builder::{
        CreateCommand, CreateEmbed,
        CreateInteractionResponseMessage
    }, client::{self, Context}, model::{application::CommandInteraction, connection}
};
use crate::utils::{
    create_log_message, LogLevel,
    DatabaseConnectionContainer,
    EmbedColours
};
use crate::sql_scripts::discord_users;





pub fn build() -> CreateCommand {
    CreateCommand::new("deregister")
        .description("Remove yourself from the database")
}


/*
let mut client_data = ctx.data.write().await;

        let database_connection = match client_data.get_mut::<DatabaseConnectionContainer>()
*/


pub async fn run( interaction_data: &CommandInteraction, ctx: &Context ) -> Option<CreateInteractionResponse> {

    let invoking_user_id = interaction_data.user.id.get();
    

    // We need to do a few things
    // 1) Check if user exists in DB, if not, notify and quit early
    // 2) Remove the DiscordUsers entry by matching the command envoker's Discord ID

    let response_embed = {
        
        // --== LOCK THE DATABASE CONNECTION FOR FURTHER USE ==-- //

            // We put it in a block to keep it from blocking us from sending a response on `interaction_data`

            // Get the `Arc<Mutex<...>>` we passed into `client.data`. And because we passed it we
            // know it WILL be there, as a result we can just `.expect()` the `Option<...>` that
            // `.get_mut()` gives us.
            let mut client_data = ctx.data.write().await;
            let connection_guard = client_data
                .get_mut::<DatabaseConnectionContainer>()
                .expect("Database connection should be passed in during creation of client");

            // Rust's Mutex<...> have a feature called poisoning, where if a thread panics while
            // holding a mutex guard, it will become "poisoned". This is a flag that signals to us
            // that the data inside could be invalid as a result.
            //
            // However, even if that were to occur in this program, our database connection will
            // still always remain valid. As a result if we *do* recieve a poisoned mutex, we will
            // send a warning to the terminal, then clear the poison
            if connection_guard.is_poisoned() {
                println!( "{}", create_log_message(
                    "DatabaseConnectionContainer Mutex was poisoned, proceeding to clear poison flag",
                    LogLevel::Warning
                ));
                connection_guard.clear_poison();
            }

            // Irrelevant to whether the mutex was or wasn't poisoned, it at this point will not be
            // considered as such. As a result after `.lock()`ing we will always obtain an `Ok(...)`
            // variant. Posioning is the only reason why `.lock()` can return an `Err(...)` variant.
            // At least as far as Rust 1.83.0 Documentation is concerned.
            let database_connection = connection_guard
                .lock()
                .expect("Poisoned flag should already have been cleared");
        // ==--

        // --== TEST DATABASE ENTRIES ==-- //
        
            // Before a person can deregister themselves, we need to make sure they don't have any
            // characters in the databse, and that they even have a profile in the first place

        // --== PROFILE TEST ==-- //    
            // This slash command can be called by any user, even if they're not even in the
            // database. So here we'll test to see if the user is in the database.
            let mut testing_statement = database_connection
                .prepare(discord_users::SELECT_BY_ID)
                .expect("This should not fail. Only doing so unexpectedly, for no fault of our own");
            
            let query_result = testing_statement.query_row( [&invoking_user_id], |row| row.get::<usize, u64>(0) )
                .optional()
                .expect("This should not fail. Only doing so unexpectedly, for no fault of our own");
            
            // If our query returns a `None`, that means that the invoking user is not in our
            // database. In that case, we should abort further execution. Not forgetting to notify
            // the invoker of the state of things.
            if query_result.is_none() {
                
                let info_embed = CreateEmbed::new()
                    .title("You're not in the database")
                    .description("That's alright! With nothing to remove, we'll just do nothing more")
                    .colour(EmbedColours::ERROR);

                return Some(
                    CreateInteractionResponse::Message(  CreateInteractionResponseMessage::new().embed(info_embed)  )
                );
            }
        // ==--

        // --== REMOVE PROFILE ==-- //
        
            // This shouldn't ever be a problem as we have handled the edge case of the user not
            // being in the database. I don't know any way this could error
            let _ = database_connection.execute( discord_users::REMOVE_ENTRY, [&invoking_user_id] );

            let invoking_user_tag = interaction_data.user.tag();
            // Finally, We'll just notify that a profile was removed and who's
            println!( "{}", create_log_message(
                    format!( "Removed {invoking_user_tag}'s profile from the database" ).as_str(),
                    LogLevel::Info
                )
            );
        // ==--


        // --== FINAL INFO ==-- //
        
            // Finally we create the embed that'll tell the user that invoked this command that
            // their profile has been removed, but their characters are still there.
            //
            // One small issue though, the current database schema doesn't allow for dangeling
            // characters, that can be changed simply though.
            CreateEmbed::new()
                .title("Your profile has been successfully removed from the database")
                .description("Your characters, if there were any, are still in the database")
                .colour(EmbedColours::GOOD)
        // ==--
    };
    
    Some(
        CreateInteractionResponse::Message(  CreateInteractionResponseMessage::new().embed(response_embed)  )
    )
}
