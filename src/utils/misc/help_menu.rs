use crate::utils::misc::colour_codes::ColourCode;

pub fn help_menu() -> String {
        
    format!(
        "This will be a help menu! For now have some colours!\n{}Error\n{}Warning\n{}Caution\n{}Success\n{}Info\n{}Field\n{}Location{}\n\
        Error codes:\n\
          - 1:  Incorrect Configuration\n\
          - 20: Database Connection Error",
        ColourCode::Error,
        ColourCode::Warning,
        ColourCode::Caution,
        ColourCode::Success,
        ColourCode::Info,
        ColourCode::Field,
        ColourCode::Location,
        ColourCode::Reset
    )
}
