use std::{env, fmt};
use std::fmt::Formatter;
use std::io::BufReader;
use chrono::Utc;
use tokio_cron_scheduler::{Job, JobScheduler};

fn play_sound(file: &std::fs::File) -> Result<(), ProcessError>{
    let file = file.try_clone().map_err(|_| ProcessError::FileReadError)?;
    let decoder = rodio::Decoder::new(BufReader::new(file)).map_err(|_| ProcessError::FileReadError)?;
    // OutputStream (_stream) is needed to play sound.
    let (_stream, handle) = rodio::OutputStream::try_default().map_err(|_| ProcessError::OutputStreamError)?;
    let sink = rodio::Sink::try_new(&handle).map_err(|_| ProcessError::OutputStreamError)?;

    sink.append(decoder);
    sink.sleep_until_end();
    Ok(())
}

struct JobCreateService {
}

impl JobCreateService {
    fn new() -> Self {
        Self {}
    }

    /// Create a job to add scheduler
    ///
    /// * `expression` - Text to represent the schedule.
    /// * `filename` - Sound file path. (String is used to move the ownership to closure and save file path within job to return)
    fn create_sound_job(expression: &str, file_path: String) -> Job {
        Job::new(expression, move |_uuid, _l| {
            println!("{:?}", Utc::now());
            let file = std::fs::File::open(file_path.clone()).unwrap();
            match play_sound(&file) {
                Err(err) => println!("{}", err),
                _ => {},
            }
            println!("{:?}", Utc::now());
        }).unwrap()
    }
}

struct JobExecuteService {
    scheduler: JobScheduler,
}

impl JobExecuteService {
    fn new(scheduler: JobScheduler) -> Self {
        Self { scheduler }
    }

    async fn start(&mut self) {
        #[cfg(feature = "signal")]
        self.scheduler.s.shutdown_on_ctrl_c();

        self.scheduler.set_shutdown_handler(Box::new(|| {
            Box::pin(async move {
                println!("Shut down done");
            })
        }));
        let _ = self.scheduler.start().await.unwrap();
    }

    async fn add(&self, job: Job) {
        self.scheduler.add(job).await.unwrap();
    }
}

#[tokio::main]
async fn main() {
    let expression = "0 1/1 * * * * *";
    let mut sched = JobScheduler::new().await.unwrap();

    let file_path = env::args().nth(1).unwrap();
    let job = JobCreateService::create_sound_job(expression, file_path.clone());

    let mut job_execute_service = JobExecuteService::new(sched);
    job_execute_service.add(job).await;
    job_execute_service.start().await;

    loop {
        tokio::time::sleep(core::time::Duration::from_millis(500)).await;
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