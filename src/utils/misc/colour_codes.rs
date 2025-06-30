#![allow(dead_code)]

use std::fmt::Display;

use termion::color;

// Error    = color::LightRed;
// Warning  = color::Yellow;
// Caution  = color::LightYellow;
// Success  = color::LightGreen;
// Info     = color::LightCyan;
// Field    = color::LightBlue;
// Location = color::Magenta;

/// Clears the foreground colours
pub struct ResetColour;
impl Display for ResetColour {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!( f, "{}", color::Reset.fg_str() )
    }
}

/// A nice `LightRed` colour for logging errors
pub struct ErrorColour;
impl Display for ErrorColour {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!( f, "{}", color::LightRed.fg_str() )
    }
}


/// A nice `Yellow` colour for logging warnings 
pub struct WarningColour;
impl Display for WarningColour {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!( f, "{}", color::Yellow.fg_str() )
    }
}


/// A nice `LightYellow` colour for logging cautions
pub struct CautionColour;
impl Display for CautionColour {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!( f, "{}", color::LightYellow.fg_str() )
    }
}


/// A nice `LightGreen` colour for logging successes
pub struct SuccessColour;
impl Display for SuccessColour {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!( f, "{}", color::LightGreen.fg_str() )
    }
}


/// A nice `LightCyan` colour for logging infos
pub struct InfoColour;
impl Display for InfoColour {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!( f, "{}", color::LightCyan.fg_str() )
    }
}


/// A nice `LightBlue` colour for logging fields
pub struct FieldColour;
impl Display for FieldColour {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!( f, "{}", color::LightBlue.fg_str() )
    }
}


/// A nice `Magenta` colour for logging locations
pub struct LocationColour;
impl Display for LocationColour {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!( f, "{}", color::Magenta.fg_str() )
    }
}

