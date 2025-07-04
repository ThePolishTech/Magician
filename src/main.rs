// --== MODULE IMPORTS ==-- //
    mod utils;
    use utils::{
        startup::parse_arguments::{
            self,
            ConfigPathKey
        },
    };

    mod runtime;
    use runtime::{
        context_keys,
        runtime_client
    };

// ==--

use core::panic;
// --== CRATE IMPORTS ==-- //
    
    // STD & CORE
        use std::{
            env, fs, io, path::PathBuf, process
        };

    // SERENITY
        use serenity::{
            Client,
            model::gateway::GatewayIntents
        };

    // TERMION
        use termion::color;

    // SQLX
        use sqlx::{
            sqlite::SqliteConnectOptions, SqlitePool
        };

    // TOML
        use toml::{Table, Value};

    use crate::utils::misc::colour_codes;
// ==--

#[tokio::main]
async fn main() {

    // --== PROCESS ARGS ==-- //
    
        // Our first order of buisness is collecting arguments passed to our program. Thanks to this we
        // will be able to bake in some launch paramaters
        let inbound_arguments: Vec<String> = env::args().collect();
        let post_parse_data = parse_arguments::parse_arguments(inbound_arguments.clone());
    // ==--
    

    // --== READ CONFIG ==-- //

        // If user didn't specify a custom path to the bot config toml, attempt to find the file, make
        // sure to see if it can be openable
        print!("Reading Configuration file... ");

        let bot_config_path: PathBuf = match post_parse_data.get::<ConfigPathKey>() {
            Some(path) => path.to_path_buf(),
            None => {
                let current_dir = match env::current_dir() {
                    Ok(path) => path,
                    Err(why) => {
                        println!(
                            "{}Error{}: Unable to fetch current directory `{}{}{}`",
                            color::Red.fg_str(),
                            color::Reset.fg_str(),
                            color::LightBlue.fg_str(),
                            why,
                            color::Reset.fg_str()
                        );
                        process::exit(1)
                    }
                };

                match fs::File::open( current_dir.join("bot_config.toml") ) {
                    Err(why) => {

                        if why.kind() == io::ErrorKind::NotFound {
                            println!(
                                "{}Error{}: Missing config file, invoke with `{}config generate{}` to generate placeholder config file",
                                color::Red.fg_str(),
                                color::Reset.fg_str(),
                                color::LightBlue.fg_str(),
                                color::Reset.fg_str()
                            );
                            process::exit(1)
                        }

                        // For some reason we cannot access the bot config, let us notify the user
                        println!(
                            "{}Error{}: Cannot access config file `{}{}{}`",
                            color::Red.fg_str(),
                            color::Reset.fg_str(),
                            color::LightBlue.fg_str(),
                            why.kind(),
                            color::Reset.fg_str()
                        );
                        process::exit(1)
                    },
                    Ok(_) => current_dir.join("bot_config.toml")
                }
            }
        };

        // Read the contents of the config
        let config_data_buf = match fs::read_to_string(&bot_config_path) {
            Ok(contents) => contents,
            Err(why) => {
                println!(
                    "{}Error{}: Failed to read config file: `{}{}{}`",
                    colour_codes::ErrorColour,
                    colour_codes::ResetColour,
                    colour_codes::InfoColour,
                    why,
                    colour_codes::ResetColour
                );
                process::exit(1);
            }
        };

        // Make sure the syntax makes sense
        let config_data = match config_data_buf.parse::<Table>() {
            Ok(config_data) => config_data,
            Err(why) => {
                println!(
                    "{}Error{}: Failed to parse configuration data. Is the config file malformed?: \n{}{}{}",
                    colour_codes::ErrorColour,
                    colour_codes::ResetColour,
                    colour_codes::InfoColour,
                    why,
                    colour_codes::ResetColour
                );
                process::exit(1);
            }
        };
        
        // Verify existance of all required fields
        // We know that the config file is already correct in it's syntax, here we check to see if
        // it makes sense, by making sure each required field exists and is of the same value.
        // The list will grow as we add more fields to it
        let missing_fields = {
            let mut missing_fields = vec![];

            if !matches!( config_data.get("bot_token"), Some(Value::String(_)) ) {
                missing_fields.push("bot_token");
            }

            if!matches!( config_data.get("wakeup_channel_id"), Some(Value::Integer(_)) ) {
                missing_fields.push("wakeup_channel_id");
            }

            missing_fields
        };

        if !missing_fields.is_empty() {
            print!(
                "{}Error{}: Config file is either missing fields, or they are incorrect. Offending fields: `{}",
                colour_codes::ErrorColour,
                colour_codes::ResetColour,
                colour_codes::InfoColour
            );

            for (idx, field) in missing_fields.iter().enumerate() {
                if missing_fields.len()-1 == idx {
                    print!("{field}");
                } else {
                    print!("{field}{}, {}", colour_codes::ResetColour, colour_codes::InfoColour);
                }
            }
            println!(
                "{}`",
                colour_codes::ResetColour
            );
            process::exit(1);
        }
        println!(
            "{}Ok!{}",
            colour_codes::SuccessColour,
            colour_codes::ResetColour
        );

        // Store config data into variables
        let bot_token =
            match &config_data["bot_token"] { Value::String(token) => token, _ => panic!("Code failed to validate field `bot_token`") };
        let wakeup_channel_id =
            match &config_data["wakeup_channel_id"] { Value::Integer(id) => *id as u64, _ => panic!("Code failed to validate field `wakeup_channel_id`") };
    // ==--
    
    // --== OPEN DATABASE ==-- //
    
        print!("Opening connection to database... ");
        let sqlite_connection_options = SqliteConnectOptions::new()
            .filename("realm.db")
            .create_if_missing(true);


        let db_connection = match SqlitePool::connect_with(sqlite_connection_options).await {
            Ok(conn) => conn,
            Err(why) => {
                println!(
                    "{}Error{}: Unable to connect to database: `{}{}{}`",
                    colour_codes::ErrorColour,
                    colour_codes::ResetColour,
                    colour_codes::InfoColour,
                    why,
                    colour_codes::ResetColour
                );
                process::exit(20);
            }
        };
        println!(
            "{}Ok!{}",
            colour_codes::SuccessColour,
            colour_codes::ResetColour
        );
    // ==--
    
    // --== RUN INIT SCRIPT ==-- // 
    
        print!("Running database migration script... ");
        match sqlx::migrate!("./src/migrations").run(&db_connection).await {
            Ok(()) => println!("{}Ok!{}", colour_codes::SuccessColour, colour_codes::ResetColour),
            Err(why) => {
                println!(
                    "{}Error{}: Unable to preform database migration: `{}{}{}`",
                    colour_codes::ErrorColour,
                    colour_codes::ResetColour,
                    colour_codes::InfoColour,
                    why,
                    colour_codes::ResetColour
                );
            }
        }
    // ==--

    // --== SETUP CONECTION TO GATEWAY ==-- //
    
        print!("Setting up client... ");
        let gateway_intents = GatewayIntents::GUILD_MESSAGES
            | GatewayIntents::MESSAGE_CONTENT
            | GatewayIntents::GUILD_MESSAGE_REACTIONS;

        let client = runtime_client::RuntimeClient {
            database_connection: db_connection
        };

        println!(
            "{}Ok!{}",
            colour_codes::SuccessColour,
            colour_codes::ResetColour
        );
    // ==--
    
    // --== BUILD CLIENT ==-- //

        print!("Building client... ");
        let mut bot_client = match Client::builder( bot_token, gateway_intents ).event_handler(client).await {
            Ok(client_builder) => {

                // Filling our client's context typemap with values
                {
                    let mut data_write = client_builder.data.write().await;
                    data_write.insert::<context_keys::WakeupChannelIdKey>( wakeup_channel_id );
                }

                println!(
                    "{}Ok!{}",
                    colour_codes::SuccessColour,
                    colour_codes::ResetColour
                );

                client_builder
            },
            Err(why) => {
                println!(
                    "{}Error{}: Failed to build client: `{}{}{}`",
                    colour_codes::ErrorColour,
                    colour_codes::ResetColour,
                    colour_codes::InfoColour,
                    why,
                    colour_codes::ResetColour
                );
                process::exit(3);
            }
        };
    // ==--
    
    // --== STARTING CLIENT ==-- //
        println!("Starting Client... {}Ok!{}", colour_codes::SuccessColour, colour_codes::ResetColour);
        let client_exit = bot_client.start().await;
        match client_exit {
            Ok(()) => {
                println!("{}Client exited sucessfully{}", colour_codes::SuccessColour, colour_codes::ResetColour);
                process::exit(0);
            },
            Err(why) => {
                println!(
                    "{}Error{}: Failed to start client `{}{}{}`",
                    colour_codes::ErrorColour,
                    colour_codes::ResetColour,
                    colour_codes::InfoColour,
                    why,
                    colour_codes::ResetColour
                );
                process::exit(1);
            }
        }
    //
}

