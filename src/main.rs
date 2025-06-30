// --== MODULE IMPORTS ==-- //
    mod utils;
// ==--

// --== CRATE IMPORTS ==-- //
    
    // STD & CORE
        use std::{
            env, fs, io::{self, Write}, path::PathBuf, process
        };

    // SERENITY

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
                            "{}Info{}: {}Config{}: Generating config file...",
                            color::LightCyan.fg_str(),
                            color::Reset.fg_str(),
                            color::Cyan.fg_str(),
                            color::Reset.fg_str()
                        );
                        let mut generated_config = match fs::File::create_new("bot_config.toml") {
                            Ok(file) => file,
                            Err(why) => {
                                if let io::ErrorKind::AlreadyExists = why.kind() {
                                    println!(
                                        "{}Error{}: {}Config{}: Attempting to generate already existing config file",
                                        color::Red.fg_str(),
                                        color::Reset.fg_str(),
                                        color::Cyan.fg_str(),
                                        color::Reset.fg_str()
                                    );
                                } else {
                                    println!(
                                        "{}Error{}: {}Config{}: An unexpected error occured! `{}{}{}`",
                                        color::Red.fg_str(),
                                        color::Reset.fg_str(),
                                        color::Cyan.fg_str(),
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
                            "{}Caution{}: {}Config{}: Generated config file contains placeholder values, please remember to replace them",
                            color::LightYellow.fg_str(),
                            color::Reset.fg_str(),
                            color::Cyan.fg_str(),
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
                                    "{}Error{}: {}Config{}: Specified config file path `{}{}{}` does not exist or is not readable",
                                    color::Red.fg_str(),
                                    color::Reset.fg_str(),
                                    color::Cyan.fg_str(),
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
                                "{}Error{}: {}Config{}: No config path specified",
                                color::Red.fg_str(),
                                color::Reset.fg_str(),
                                color::Cyan.fg_str(),
                                color::Reset.fg_str()
                            );
                            process::exit(1);
                        }
                    },
                    // option == "path"

                    Some(option) => {
                        println!(
                            "{}Error{}: {}Config{}: Unknown option `{}{}{}`",
                            color::Red.fg_str(),
                            color::Reset.fg_str(),
                            color::Cyan.fg_str(),
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
                            "{}Error{}: {}Config{}: No options detected, invoke with `{}help{}` for list of options",
                            color::Red.fg_str(),
                            color::Reset.fg_str(),
                            color::Cyan.fg_str(),
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

            Some(value) => {
                println!(
                    "{}Error{}: Unknown argument: `{}{}{}`",
                    color::Red.fg_str(),
                    color::Reset.fg_str(),
                    color::LightBlue.fg_str(),
                    value,
                    color::Reset.fg_str()
                );
                process::exit(1)
            }
            _ => {}
        }
    // ==--
    

    // If user didn't specify a custom path to the bot config toml, attempt to find the file, make
    // sure to see if it can be openable
    let bot_config_path: PathBuf = match bot_config_path {
        Some(path) => path,
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

    println!(
        "Current Dir: {:?}\nArguments: {:?}\nPath: {:?}", env::current_dir(), inbound_arguments, bot_config_path
    );
}

