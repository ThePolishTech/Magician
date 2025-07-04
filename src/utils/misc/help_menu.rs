use crate::utils::misc::colour_codes;

pub fn help_menu() -> String {
        
    format!(
        "This will be a help menu! For now have some colours!\n{}Error\n{}Warning\n{}Caution\n{}Success\n{}Info\n{}Field\n{}Location{}\n\
        Error codes:\n\
          - 1:  Incorrect Configuration\n\
          - 20: Database Connection Error",
        colour_codes::ErrorColour,
        colour_codes::WarningColour,
        colour_codes::CautionColour,
        colour_codes::SuccessColour,
        colour_codes::InfoColour,
        colour_codes::FieldColour,
        colour_codes::LocationColour,
        colour_codes::ResetColour
    )
}
