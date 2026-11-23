use sensor_input::{parse_line, ParseError, Reading};

#[test]
fn parses_trimmed_fields() {
    assert_eq!(
        parse_line(" temp-1 , 42 , -7 ").unwrap(),
        Reading {
            sensor: "temp-1".into(),
            timestamp: 42,
            value: -7,
        }
    );
}

#[test]
fn rejects_missing_and_extra_fields() {
    assert_eq!(
        parse_line("sensor,12"),
        Err(ParseError::MissingField { index: 2 })
    );
    assert_eq!(parse_line("sensor,12,"), Err(ParseError::InvalidValue));
    assert_eq!(parse_line("sensor,12,3,extra"), Err(ParseError::ExtraField));
    assert_eq!(
        parse_line(",12,3"),
        Err(ParseError::MissingField { index: 0 })
    );
}

#[test]
fn rejects_malformed_integers_and_out_of_bounds_values() {
    assert_eq!(parse_line("sensor,-1,0"), Err(ParseError::InvalidTimestamp));
    assert_eq!(
        parse_line("sensor,18446744073709551616,0"),
        Err(ParseError::InvalidTimestamp)
    );
    assert_eq!(
        parse_line("sensor,1,9223372036854775808"),
        Err(ParseError::InvalidValue)
    );
    assert_eq!(
        parse_line("sensor,1,-9223372036854775809"),
        Err(ParseError::InvalidValue)
    );
    assert_eq!(
        parse_line("sensor,nope,2"),
        Err(ParseError::InvalidTimestamp)
    );
    assert_eq!(parse_line("sensor,2,1.5"), Err(ParseError::InvalidValue));
}

#[test]
fn rejects_invalid_identifiers() {
    for line in ["1sensor,2,3", "bad name,2,3", "bad.dot,2,3", "é,2,3"] {
        assert_eq!(
            parse_line(line),
            Err(ParseError::InvalidIdentifier),
            "{line:?}"
        );
    }
}
