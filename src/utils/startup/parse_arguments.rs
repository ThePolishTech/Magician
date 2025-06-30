use std::{
    path::PathBuf, process
};
use crate::utils::misc::colour_codes;


use serenity::prelude::{
    TypeMap, TypeMapKey
};



pub struct ConfigPathKey;
impl TypeMapKey for ConfigPathKey {
    type Value = Option<PathBuf>;
}

/// # Exits
/// This function will exit the process with a value of `1` in the case of invalid inputs, or
/// erroneous states
pub fn parse_arguments( arguments_in: &[String] ) -> TypeMap {

    // Because what we expect to be returned in terms of values mutable by user selected args is
    // inexhaustive, aka. might change in the future, we will just return a typemap and publish
    // TypeMapKeys for value reading. Additionally, we will preload the values with their defaults
    let mut return_typemap = TypeMap::new();
    return_typemap.insert::<ConfigPathKey>( None );

    // Here the bulk of the processing will happen. The way we will do this is matching on the
    // first item in our arguments list, then if applicable, the second in a nested `match`, etc
    match arguments_in.get(1) {
        Some(primary_argument) 
            if primary_argument == "help" || primary_argument == "--help"
        => {
            println!(
                "This will be a help menu! For now have some colours!\n{}Error\n{}Warning\n{}Caution\n{}Success\n{}Info\n{}Field\n{}Location{}",
                colour_codes::ErrorColour,
                colour_codes::WarningColour,
                colour_codes::CautionColour,
                colour_codes::SuccessColour,
                colour_codes::InfoColour,
                colour_codes::FieldColour,
                colour_codes::LocationColour,
                colour_codes::ResetColour
            )
        },
        // [1] Help
        
        Some(primary_argument) if primary_argument == "config" => {

            match arguments_in.get(2) {
                Some(secondary_argument) if secondary_argument == "path" => {},
                // [1] Config [2] path

                Some(secondary_argument) if secondary_argument == "generate" => {},
                // [1] Config [2] generate

                Some(unknown_secondary_argument) => {
                    println!(
                        "{}Error{}: Unknown command option: `{}{}{}` Consider invoking with `{}help{}` for a list of commands",
                        colour_codes::ErrorColour,
                        colour_codes::ResetColour,
                        colour_codes::FieldColour,
                        unknown_secondary_argument,
                        colour_codes::ResetColour,
                        colour_codes::InfoColour,
                        colour_codes::ResetColour
                    );
                    process::exit(1);

                },
                // [1] Config [2] Unknown

                None => {
                    println!(
                        "{}Error{}: Missing command option. Consider invoking with `{}help{}` for a list of commands",
                        colour_codes::ErrorColour,
                        colour_codes::ResetColour,
                        colour_codes::InfoColour,
                        colour_codes::ResetColour
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
                colour_codes::ErrorColour,
                colour_codes::ResetColour,
                colour_codes::FieldColour,
                unknown_argument,
                colour_codes::ResetColour,
                colour_codes::InfoColour,
                colour_codes::ResetColour
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
