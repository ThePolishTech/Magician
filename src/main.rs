// --== CRATE IMPORTS ==-- //
    
    // STD & CORE
        use std::{
            env,
            fs,
            io::{self, Write},
            path::{Path, PathBuf},
            process
        };

    // SERENITY
        use serenity::{Client};

    // TERMION
        use termion::color;

    // CHRONO

    // TOKIO
    
    // SQLX
    
    // TOML
// ==--

#[tokio::main]
async fn main() {

    // --== PROCESS ARGS ==-- //
    
        // We will predeclare some variables with default values that can get mutated by user
        // passed arguments
        let mut bot_config_path: Option<PathBuf> = None;

        // Our first order of buisness is collecting arguments passed to our program. Thanks to this we
        // will be able to bake in some launch paramaters
        let inbound_arguments: Vec<String> = env::args().collect();
        match inbound_arguments.get(1) {
            Some(value)
                if value == "help" || value == "--help"
            => {
                println!("This is a help menu :verger:");
                process::exit(0);
            },

            Some(value) if value == "config" => {
                // config
                // | generate
                // | path

                match inbound_arguments.get(2) {
                    Some(option) if option == "generate" => {

                        // Inform and create file
                        println!(
                            "{}Info{}: Generating config file...",
                            color::LightCyan.fg_str(),
                            color::Reset.fg_str()
                        );
                        let mut generated_config = match fs::File::create_new("bot_config.toml") {
                            Ok(file) => file,
                            Err(why) => {
                                if let io::ErrorKind::AlreadyExists = why.kind() {
                                    println!(
                                        "{}Error{}: Attempting to generate already existing config file",
                                        color::Red.fg_str(),
                                        color::Reset.fg_str()
                                    );
                                } else {
                                    println!(
                                        "{}Error{}: An unexpected error occured! `{}{}{}`",
                                        color::Red.fg_str(),
                                        color::Reset.fg_str(),
                                        color::Red.fg_str(),
                                        why,
                                        color::Reset.fg_str()
                                    );
                                }
                                process::exit(1);
                            }
                        };

                        // Write to file and inform user of placeholder values
                        let _ = generated_config.write_all(
                            b"token = <INSERT DISCORD APPLICATION TOKEN>"
                        );
                        println!(
                            "{}Caution{}: Generated config file contains placeholder values, please remember to replace them",
                            color::LightYellow.fg_str(),
                            color::Reset.fg_str()
                        );
                        process::exit(0);
                    },
                    // option == "generate"

                    Some(option) if option == "path" => {
                        if let Some(path_query) = inbound_arguments.get(3) {
                            
                            // Check to see if provided path leads to readable path.  TODO: Specify
                            // to user whether the file actually exists or just isn't openable for
                            // some reason or another
                            if fs::File::open(path_query).is_err() {
                                println!(
                                    "{}Error{}: Specified config file path `{}{}{}` does not exist or is not readable",
                                    color::Red.fg_str(),
                                    color::Reset.fg_str(),
                                    color::LightBlue.fg_str(),
                                    path_query,
                                    color::Reset.fg_str()
                                );
                                process::exit(1);
                            }

                            bot_config_path = Some(PathBuf::from(path_query));
                        } else {
                            println!(
                                "{}Error{}: No config path specified",
                                color::Red.fg_str(),
                                color::Reset.fg_str()
                            );
                            process::exit(1);
                        }
                    },
                    // option == "path"

                    Some(option) => {
                        println!(
                            "{}Error{}: Unknown option `{}{}{}`",
                            color::Red.fg_str(),
                            color::Reset.fg_str(),
                            color::LightBlue.fg_str(),
                            option,
                            color::Reset.fg_str()
                        );
                        process::exit(1);
                    },
                    // Unknown option

                    None => {
                        println!(
                            "{}Error{}: No options detected, invoke with `{}help{}` for list of options",
                            color::Red.fg_str(),
                            color::Reset.fg_str(),
                            color::LightBlue.fg_str(),
                            color::Reset.fg_str()
                        );
                        process::exit(1);
                    }
                    // Missing option
                }
            },
            // value == "config"

            _ => {}
        }
    // ==--

    let client_or_error: Result<Client, (&'static str, u16)> =  'client_builder: {
        Err( ("WIP", 0) )
    };
    println!(
        "Current Dir: {:?}\nArguments: {:?}\nPath: {:?}", env::current_dir(), inbound_arguments, bot_config_path
    );
}
