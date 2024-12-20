// --== ATTRIBUTES ==-- //
#![allow(unused_braces)]

// xxxxxxxxxxxxxxxxx //
// --== CRATES == -- //
// xxxxxxxxxxxxxxxxx //
use std::{
    env,
    sync::{Arc,Mutex}
};

use serenity::{
    model::gateway::GatewayIntents, Client
};

mod sql_scripts;
mod event_handler;
mod commands;
mod utils;
use utils::DatabaseConnectionContainer;
//use utils::{
//    LogLevel,
//    create_log_message
//};


// xxxxxxxxxxxxxx //
// --== MAIN ==-- //
// xxxxxxxxxxxxxx //
#[tokio::main]
async fn main() {

    println!( "{}", utils::TITLE );

    let bot_client: Result< (serenity::Client, rusqlite::Connection), i32 > = 'main: {

        // --== LOAD/CREATE DATABASE ==-- //

            print!("Opening Connection to Database...");
            let rusqlite_connection = match rusqlite::Connection::open("kermmaw_db") {
                Ok(conn) => {
                    println!("Ok");
                    conn
                },
                Err(why) => {
                    println!("Error: {why}");
                    break 'main Err( 1 );
                }
            };

            print!("Running Table Creation Script...");
            match rusqlite_connection.execute_batch( sql_scripts::create_tables::SCRIPT ) {
                Ok(_) => {
                    println!("Ok")
                },
                Err(why) => {
                    println!("Error: {why}");
                    break 'main Err( 1 );
                }
            }
        // ==--

        // Verify that there is a bot token in enviroment
        print!("Reading Bot Token From Enviromental Variable..."); 
        let bot_token = match env::var("BOT_TOKEN") {
            Ok(token) => {
                println!("Ok");
                token
            },
            Err(_) => {
                println!("Error: Missing Token in Enviroment");
                break 'main Err( 1 );
            }
        };

        // --== SETUP CONNECTION TO GATEWAY ==-- //   

            let gateway_intents = GatewayIntents::GUILD_MESSAGES
                | GatewayIntents::MESSAGE_CONTENT
                | GatewayIntents::GUILD_MESSAGE_REACTIONS;

            print!("Building Client...");
            match Client::builder( bot_token, gateway_intents ).event_handler(event_handler::Handler).await {
                Ok(client_builder) => {
                    println!("Ok");
                    break 'main Ok( (client_builder, rusqlite_connection) )
                },
                Err(why) => {
                    println!("Error: {}", why);
                    break 'main Err( 1 );
                }
            }
        // ==--
    };

    // Start the client
    print!("Starting Client...");
    match bot_client {
        Ok( (mut client, rusqlite_connection) ) => {
            println!("Ok\n");
            let data = client.data.write();
            data.await.insert::<DatabaseConnectionContainer>(Arc::new( Mutex::new( rusqlite_connection ) ));
            let _ = client.start().await;
        },
        Err(code) => {
            println!("Error: Code {}\n", code);
        }
    }
}

