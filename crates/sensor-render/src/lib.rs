use sensor_stats::Summary;
use std::fmt::Write;

/// Render sensor summaries as CSV with a header and one record per summary.
pub fn render(summaries: &[Summary]) -> String {
    let mut output = String::from("sensor,count,min,max,sum\n");
    for summary in summaries {
        write_field(&mut output, &summary.sensor);
        writeln!(
            output,
            ",{},{},{},{}",
            summary.count, summary.min, summary.max, summary.sum
        )
        .expect("writing to a String is infallible");
    }
    output
}

fn write_field(output: &mut String, value: &str) {
    if value
        .bytes()
        .any(|byte| matches!(byte, b',' | b'"' | b'\r' | b'\n'))
    {
        output.push('"');
        for character in value.chars() {
            if character == '"' {
                output.push('"');
            }
            output.push(character);
        }
        output.push('"');
    } else {
        output.push_str(value);
    }
}

#[cfg(test)]
mod tests {
    use super::render;
    use sensor_stats::Summary;

    #[test]
    fn renders_header_and_csv_escaped_sensor_names() {
        let summaries = [
            Summary {
                sensor: "plain".into(),
                count: 2,
                min: -3,
                max: 5,
                sum: 2,
            },
            Summary {
                sensor: "room, \"north\"\nline".into(),
                count: 1,
                min: 8,
                max: 8,
                sum: 8,
            },
        ];
        assert_eq!(
            render(&summaries),
            concat!(
                "sensor,count,min,max,sum\nplain,2,-3,5,2\n",
                "\"room, \"\"north\"\"\nline\",1,8,8,8\n"
            )
        );
    }

    #[test]
    fn renders_empty_summary_with_header_only() {
        assert_eq!(render(&[]), "sensor,count,min,max,sum\n");
    }
}
