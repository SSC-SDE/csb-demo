use std::fmt;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ParseError {
    MissingField { index: usize },
    ExtraField,
    InvalidIdentifier,
    InvalidTimestamp,
    InvalidValue,
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MissingField { index } => write!(f, "missing field {}", index + 1),
            Self::ExtraField => f.write_str("too many fields"),
            Self::InvalidIdentifier => f.write_str("invalid sensor identifier"),
            Self::InvalidTimestamp => f.write_str("invalid unsigned timestamp"),
            Self::InvalidValue => f.write_str("invalid signed reading value"),
        }
    }
}

impl std::error::Error for ParseError {}
