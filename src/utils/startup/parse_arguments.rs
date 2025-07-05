#![allow(clippy::print_literal)]

// --== MODULE IMPORTS ==-- //
    use crate::utils::misc::{
        colour_codes::ColourCode, help_menu::help_menu, template_config
    };
// ==--

// --== CRATE IMPORTS ==-- //

    // STD & CORE
    use std::{
        fs,
        process,
        path::PathBuf,
        io::{self, Write}
    };
// ==--



use serenity::prelude::{
    TypeMap, TypeMapKey
};


// --== OUTPUT KEYS ==-- //
    
    // As we are returning with a TypeMap, we need to publish keys for accessing the values
    // returned from our argument parser function 

    // Bot config path
    pub struct ConfigPathKey;
    impl TypeMapKey for ConfigPathKey {
        type Value = PathBuf;
    }
// ==--

/// # Exits
/// This function will exit the process with a value of `1` in the case of invalid inputs, or
/// erroneous states
pub fn parse_arguments( arguments_in: Vec<String> ) -> TypeMap {

    // Because what we expect to be returned in terms of values mutable by user selected args is
    // inexhaustive, aka. might change in the future, we will just return a typemap and publish
    // TypeMapKeys for value reading. Additionally, we will preload the values with their defaults
    let mut return_typemap = TypeMap::new();

    // Here the bulk of the processing will happen. The way we will do this is matching on the
    // first item in our arguments list, then if applicable, the second in a nested `match`, etc
    match arguments_in.get(1) {
        Some(primary_argument) 
            if primary_argument == "help" || primary_argument == "--help"
        => {
            println!( "{}", help_menu() );
            process::exit(1);
        },
        // [1] Help
        
        Some(primary_argument) if primary_argument == "config" => {

            match arguments_in.get(2) {
                Some(secondary_argument) if secondary_argument == "path" => {
                    let queried_path = match arguments_in.get(3) {
                        Some(query) => query,
                        None => {
                            println!(
                                "{}Error{}: {}Config{}: Path not specified",
                                ColourCode::Error,
                                ColourCode::Reset,
                                ColourCode::Location,
                                ColourCode::Reset
                            );
                            process::exit(1);
                        }
                    };
                    if let Err(why) = fs::File::open(queried_path) {
                        match why.kind() {
                            io::ErrorKind::NotFound => {
                                println!(
                                    "{}Error{}: {}Config{}: Failed to open config file `{}{}{}`; config file missing",
                                    ColourCode::Error,
                                    ColourCode::Reset,
                                    ColourCode::Location,
                                    ColourCode::Reset,
                                    ColourCode::Field,
                                    queried_path,
                                    ColourCode::Reset
                                );
                                process::exit(1);
                            },
                            _ => {
                                println!(
                                    "{}Error{}: {}Config{}: Failed to open config file: {}{}{}",
                                    ColourCode::Error,
                                    ColourCode::Reset,
                                    ColourCode::Location,
                                    ColourCode::Reset,
                                    ColourCode::Info,
                                    why,
                                    ColourCode::Reset
                                );
                                process::exit(1);
                            }
                        }
                    } else {
                        return_typemap.insert::<ConfigPathKey>(
                            PathBuf::from(queried_path)
                        );
                    }
                },
                // [1] Config [2] path

                Some(secondary_argument) if secondary_argument == "generate" => {

                    println!(
                        "{}Info{}: Generating config file...",
                        ColourCode::Info,
                        ColourCode::Reset
                    );

                    let mut generated_file = match fs::File::create_new("bot_config.toml") {
                        Ok(file) => file,
                        Err(why) => {
                            if why.kind() == io::ErrorKind::AlreadyExists {
                                println!(
                                    "{}Error{}: {}Config{}: Attempting to generate already existing config file",
                                    ColourCode::Error,
                                    ColourCode::Reset,
                                    ColourCode::Location,
                                    ColourCode::Reset
                                );
                                process::exit(1);
                            }

                            println!(
                                "{}Error{}: {}Config{}: Cannot generate conifg file: `{}{}{}`",
                                ColourCode::Error,
                                ColourCode::Reset,
                                ColourCode::Location,
                                ColourCode::Reset,
                                ColourCode::Info,
                                why,
                                ColourCode::Reset
                            );
                            process::exit(1);
                        }
                    };

                    if let Err(why) = generated_file.write_all( template_config::TEMPLATE.as_bytes() ) {
                        println!(
                            "{}Warning{}: Config file generated, but failed to insert values. Consider removing `{}bot_config.toml{}` and retrying. Error reason: `{}{}{}`",
                            ColourCode::Warning,
                            ColourCode::Reset,
                            ColourCode::Field,
                            ColourCode::Reset,
                            ColourCode::Info,
                            why,
                            ColourCode::Reset
                        );
                        process::exit(1);
                    };

                    println!(
                        "{}Caution{}: Generated config file contains placeholder values, please remember to update them",
                        ColourCode::Caution,
                        ColourCode::Reset
                    );
                    process::exit(0);
                },
                // [1] Config [2] generate

                Some(unknown_secondary_argument) => {
                    println!(
                        "{}Error{}: Unknown command option: `{}{}{}` Consider invoking with `{}help{}` for a list of commands",
                        ColourCode::Error,
                        ColourCode::Reset,
                        ColourCode::Field,
                        unknown_secondary_argument,
                        ColourCode::Reset,
                        ColourCode::Info,
                        ColourCode::Reset
                    );
                    process::exit(1);

                },
                // [1] Config [2] Unknown

                None => {
                    println!(
                        "{}Error{}: Missing command option. Consider invoking with `{}help{}` for a list of commands",
                        ColourCode::Error,
                        ColourCode::Reset,
                        ColourCode::Info,
                        ColourCode::Reset
                    );
                    process::exit(1);
                }
            }
        }
        // [1] Config

        Some(unknown_argument) => {
            // An argument has been passed in, but because none of the match guards caught it, it
            // must be an unknown option. We shall report this to the user and exit the program
            println!(
                "{}Error{}: Unknown command: `{}{}{}`. Consider invoking with `{}help{}` for list of commands",
                ColourCode::Error,
                ColourCode::Reset,
                ColourCode::Field,
                unknown_argument,
                ColourCode::Reset,
                ColourCode::Info,
                ColourCode::Reset
            );
            process::exit(1);
        },
        // [1] Unknown

        None => {
            // In this case we don't have to do anything, so we will just let the function exit
        }
        // [2] None
    }


    return_typemap
}
