use std::fmt;

use sensor_input::Reading;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NormalizeError {
    pub sensor: String,
    pub timestamp: u64,
}

impl fmt::Display for NormalizeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "conflicting readings for sensor {:?} at timestamp {}",
            self.sensor, self.timestamp
        )
    }
}

impl std::error::Error for NormalizeError {}

pub fn normalize(mut readings: Vec<Reading>) -> Result<Vec<Reading>, NormalizeError> {
    readings.sort_by(|a, b| {
        a.sensor
            .cmp(&b.sensor)
            .then_with(|| a.timestamp.cmp(&b.timestamp))
    });

    let mut normalized: Vec<Reading> = Vec::with_capacity(readings.len());
    for reading in readings {
        if let Some(previous) = normalized.last() {
            if previous.sensor == reading.sensor && previous.timestamp == reading.timestamp {
                if previous.value != reading.value {
                    return Err(NormalizeError {
                        sensor: reading.sensor,
                        timestamp: reading.timestamp,
                    });
                }
                continue;
            }
        }
        normalized.push(reading);
    }
    Ok(normalized)
}
