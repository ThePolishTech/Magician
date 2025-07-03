// --== MODULE IMPORTS ==-- //
    mod utils;
    use utils::startup::parse_arguments;
// ==--

// --== CRATE IMPORTS ==-- //
    
    // STD & CORE
        use std::{
            env, fs, io, path::PathBuf, process
        };

    // SERENITY

    // TERMION
        use termion::color;

    use crate::utils::startup::parse_arguments::ConfigPathKey;

    // CHRONO

    // TOKIO
    
    // SQLX
    
    // TOML
// ==--

#[tokio::main]
async fn main() {

    // --== PROCESS ARGS ==-- //
    
        // Our first order of buisness is collecting arguments passed to our program. Thanks to this we
        // will be able to bake in some launch paramaters
        let inbound_arguments: Vec<String> = env::args().collect();
        let post_parse_data = parse_arguments::parse_arguments(inbound_arguments.clone());


    // ==--

    // If user didn't specify a custom path to the bot config toml, attempt to find the file, make
    // sure to see if it can be openable
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

    println!(
        "Current Dir: {:?}\nArguments: {:?}\nPath: {:?}", env::current_dir(), inbound_arguments, bot_config_path
    );
}

