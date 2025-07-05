use crate::utils::misc::colour_codes::ColourCode;

pub fn create_log_message( message: impl ToString, severity: ColourCode ) -> String {

    let timestamp = chrono::offset::Local::now();
    let timestamp = timestamp.format("%Y-%m-%d | %H:%M:%S").to_string();


    let severity_identifier = match severity {
        ColourCode::Reset     => "   Reset",
        ColourCode::Info      => "    Info",
        ColourCode::Field     => "   Field",
        ColourCode::Error     => "   Error",
        ColourCode::Warning   => " Warning",
        ColourCode::Caution   => " Caution",
        ColourCode::Success   => " Success",
        ColourCode::Location  => "Location",
    };
    let severity_identifier = format!(
        "{}{}{}",
        severity,
        severity_identifier,
        ColourCode::Reset
    );

    format!(
        "[ {timestamp} ]  => {severity_identifier}: {}", message.to_string()
    )
}

