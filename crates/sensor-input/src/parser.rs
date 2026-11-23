use crate::{fields::valid_identifier, ParseError, Reading};

pub fn parse_line(line: &str) -> Result<Reading, ParseError> {
    let mut fields = line.split(',');
    let sensor = fields.next().unwrap_or_default().trim();
    let timestamp = fields
        .next()
        .ok_or(ParseError::MissingField { index: 1 })?
        .trim();
    let value = fields
        .next()
        .ok_or(ParseError::MissingField { index: 2 })?
        .trim();
    if fields.next().is_some() {
        return Err(ParseError::ExtraField);
    }
    if sensor.is_empty() {
        return Err(ParseError::MissingField { index: 0 });
    }
    if !valid_identifier(sensor) {
        return Err(ParseError::InvalidIdentifier);
    }
    let timestamp = timestamp
        .parse()
        .map_err(|_| ParseError::InvalidTimestamp)?;
    let value = value.parse().map_err(|_| ParseError::InvalidValue)?;
    Ok(Reading {
        sensor: sensor.to_owned(),
        timestamp,
        value,
    })
}
