use chrono::Utc;
use std::fmt;
use std::fmt::Formatter;
use std::io::BufReader;
use tokio_cron_scheduler::{Job};

pub struct JobCreateService {}

fn play_sound(file: &std::fs::File) -> Result<(), ProcessError> {
    let file = file.try_clone().map_err(|_| ProcessError::FileReadError)?;
    let decoder =
        rodio::Decoder::new(BufReader::new(file)).map_err(|_| ProcessError::FileReadError)?;
    // OutputStream (_stream) is needed to play sound.
    let (_stream, handle) =
        rodio::OutputStream::try_default().map_err(|_| ProcessError::OutputStreamError)?;
    let sink = rodio::Sink::try_new(&handle).map_err(|_| ProcessError::OutputStreamError)?;

    sink.append(decoder);
    sink.sleep_until_end();
    Ok(())
}

impl JobCreateService {
    /*pub fn new() -> Self {
        Self {}
    }*/

    /// Create a job to add scheduler
    ///
    /// * `expression` - Text to represent the schedule.
    /// * `filename` - Sound file path. (String is used to move the ownership to closure and save file path within job to return)
    pub fn create_sound_job(expression: &str, file_path: String) -> Job {
        Job::new(expression, move |_uuid, _l| {
            println!("{:?}", Utc::now());
            let file = std::fs::File::open(file_path.clone()).unwrap();
            if let Err(err) = play_sound(&file) {
                println!("{}", err)
            }
            println!("{:?}", Utc::now());
        })
        .unwrap()
    }
}

enum ProcessError {
    FileReadError,
    OutputStreamError,
}

impl fmt::Display for ProcessError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match *self {
            ProcessError::FileReadError => write!(f, "file state is invalid"),
            ProcessError::OutputStreamError => write!(f, "output stream setting is failed"),
        }
    }
}
