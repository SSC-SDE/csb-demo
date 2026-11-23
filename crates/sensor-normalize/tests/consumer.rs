use sensor_input::{parse_line, Reading};
use sensor_normalize::{normalize, NormalizeError};

#[test]
fn consumer_can_normalize_sensor_input_readings() {
    let input = ["b,4,8", "a,9,-1", "a,2,3", "a,2,3"]
        .into_iter()
        .map(|line| parse_line(line).unwrap())
        .collect();
    assert_eq!(
        normalize(input).unwrap(),
        vec![
            Reading {
                sensor: "a".into(),
                timestamp: 2,
                value: 3
            },
            Reading {
                sensor: "a".into(),
                timestamp: 9,
                value: -1
            },
            Reading {
                sensor: "b".into(),
                timestamp: 4,
                value: 8
            },
        ]
    );
}

#[test]
fn rejects_conflicting_duplicate_keys() {
    let readings = vec![
        Reading {
            sensor: "x".into(),
            timestamp: 5,
            value: 1,
        },
        Reading {
            sensor: "x".into(),
            timestamp: 5,
            value: 2,
        },
    ];
    assert_eq!(
        normalize(readings),
        Err(NormalizeError {
            sensor: "x".into(),
            timestamp: 5
        })
    );
}
