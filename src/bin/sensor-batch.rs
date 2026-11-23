use std::io::{self, Write};
use std::process::ExitCode;

fn main() -> ExitCode {
    match csb_demo::process_batch(io::stdin().lock()) {
        Ok(csv) => match io::stdout().lock().write_all(csv.as_bytes()) {
            Ok(()) => ExitCode::SUCCESS,
            Err(error) if error.kind() == io::ErrorKind::BrokenPipe => ExitCode::SUCCESS,
            Err(error) => {
                eprintln!("sensor-batch: {error}");
                ExitCode::FAILURE
            }
        },
        Err(message) => {
            eprintln!("sensor-batch: {message}");
            ExitCode::FAILURE
        }
    }
}
