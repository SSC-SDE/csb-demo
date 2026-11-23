use std::collections::BTreeMap;

use sensor_input::Reading;

/// Aggregate statistics for all readings from one sensor.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Summary {
    pub sensor: String,
    pub count: usize,
    pub min: i64,
    pub max: i64,
    pub sum: i128,
}

/// Summarize readings by sensor, returning sensors in lexicographic order.
pub fn summarize(readings: &[Reading]) -> Vec<Summary> {
    let mut grouped: BTreeMap<&str, (usize, i64, i64, i128)> = BTreeMap::new();

    for reading in readings {
        let entry =
            grouped
                .entry(reading.sensor.as_str())
                .or_insert((0, reading.value, reading.value, 0));
        entry.0 += 1;
        entry.1 = entry.1.min(reading.value);
        entry.2 = entry.2.max(reading.value);
        entry.3 += i128::from(reading.value);
    }

    grouped
        .into_iter()
        .map(|(sensor, (count, min, max, sum))| Summary {
            sensor: sensor.to_owned(),
            count,
            min,
            max,
            sum,
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::{summarize, Summary};
    use sensor_input::Reading;

    fn reading(sensor: &str, value: i64) -> Reading {
        Reading {
            sensor: sensor.to_owned(),
            timestamp: 0,
            value,
        }
    }

    #[test]
    fn summarizes_and_sorts_sensors() {
        let readings = [reading("zeta", 7), reading("alpha", -4), reading("zeta", 3)];
        assert_eq!(
            summarize(&readings),
            vec![
                Summary {
                    sensor: "alpha".into(),
                    count: 1,
                    min: -4,
                    max: -4,
                    sum: -4
                },
                Summary {
                    sensor: "zeta".into(),
                    count: 2,
                    min: 3,
                    max: 7,
                    sum: 10
                },
            ]
        );
    }

    #[test]
    fn accumulates_without_i64_overflow_and_handles_empty_input() {
        let readings = [reading("wide", i64::MAX), reading("wide", i64::MAX)];
        assert_eq!(summarize(&[]), Vec::new());
        assert_eq!(summarize(&readings)[0].sum, i128::from(i64::MAX) * 2);
    }
}
