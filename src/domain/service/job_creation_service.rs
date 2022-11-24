use chrono::Utc;

//use crate::domain::error::kind::ProcessError;
use crate::domain::error::ProcessError;
use std::io::BufReader;
use tokio_cron_scheduler::Job;
use crate::PlaySound;

pub struct JobCreateService {}

fn play_sound(file: &std::fs::File) -> Result<(), ProcessError> {
    let file = file.try_clone().map_err(|e| ProcessError::FileReadError(e))?;
    let decoder =
        rodio::Decoder::new(BufReader::new(file)).map_err(|e| ProcessError::DecodeError(e))?;
    // OutputStream (_stream) is needed to play sound.
    let (_stream, handle) =
        rodio::OutputStream::try_default().map_err(|e| ProcessError::OutputStreamError(e))?;
    let sink = rodio::Sink::try_new(&handle).map_err(|e| ProcessError::PlayingError(e))?;

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
    /// * `command` - job command. (it's ownership is used to move the ownership to closure and save file path within job to return)
    pub fn create_sound_job(command: PlaySound) -> Job {
        let PlaySound { job_schedule, file_path } = command;
        Job::new(job_schedule.as_str(), move |_uuid, _l| {
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
