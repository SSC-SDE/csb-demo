use std::io::BufRead;

/// Convert sensor,timestamp,value lines into one ordered summary per sensor.
/// Blank lines are ignored. Invalid input fails the batch before any CSV is returned.
pub fn process_batch(input: impl BufRead) -> Result<String, String> {
    let mut readings = Vec::new();
    for (index, line) in input.lines().enumerate() {
        let line = line.map_err(|error| format!("line {}: {error}", index + 1))?;
        if line.trim().is_empty() {
            continue;
        }
        let reading = sensor_input::parse_line(&line)
            .map_err(|error| format!("line {}: {error:?}", index + 1))?;
        readings.push(reading);
    }
    let normalized = sensor_normalize::normalize(readings)
        .map_err(|error| format!("conflicting sensor readings: {error:?}"))?;
    let summaries = sensor_stats::summarize(&normalized);
    Ok(sensor_render::render(&summaries))
}
