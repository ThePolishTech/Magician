use std::fmt::Display;
use serenity::model::Colour;
use termion::color;


pub enum ColourCode {
    /// Resets the colour choice
    Reset,
    Info,
    Field,
    Error,
    Warning,
    Caution,
    Success,
    Location
}
impl ColourCode {
    pub fn to_embed_colour(&self) -> Colour {
        match self {
            ColourCode::Reset  => Colour::from_rgb(0, 0, 0),
            ColourCode::Info   => Colour::from_rgb(0, 255, 255),
            ColourCode::Field  => Colour::from_rgb(0, 0, 255),
            ColourCode::Error  => Colour::from_rgb(255, 0, 0),

            ColourCode::Warning   => Colour::from_rgb(128, 128, 0),
            ColourCode::Caution   => Colour::from_rgb(255, 255, 0),
            ColourCode::Success   => Colour::from_rgb(0, 255, 0),
            ColourCode::Location  => Colour::from_rgb(255, 0, 255)
        }
    }
}

impl Display for ColourCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {

        match self {
            ColourCode::Reset  =>  write!(  f, "{}", color::Reset.fg_str()      ),
            ColourCode::Info   =>  write!(  f, "{}", color::LightCyan.fg_str()  ),
            ColourCode::Field  =>  write!(  f, "{}", color::LightBlue.fg_str()  ),
            ColourCode::Error  =>  write!(  f, "{}", color::LightRed.fg_str()   ),
            
            ColourCode::Warning   =>  write!(  f, "{}", color::Yellow.fg_str()       ),
            ColourCode::Caution   =>  write!(  f, "{}", color::LightYellow.fg_str()  ),
            ColourCode::Success   =>  write!(  f, "{}", color::LightGreen.fg_str()   ),
            ColourCode::Location  =>  write!(  f, "{}", color::Magenta.fg_str()      )
        }

    }
}
