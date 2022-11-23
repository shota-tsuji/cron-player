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

#[tokio::main]
async fn main() {
    let expression = "0 1/1 * * * * *";
    let mut sched = JobScheduler::new().await.unwrap();

    let job = Job::new(expression, |_uuid, _l| {
        println!("{:?}", Utc::now());
        let filename = env::args().nth(1).unwrap();
        let file = std::fs::File::open(filename).unwrap();
        match play_sound(&file) {
            Err(err) => println!("{}", err),
            _ => {},
        }
        println!("{:?}", Utc::now());
    }).unwrap();

    sched.add(job).await.unwrap();

    #[cfg(feature = "signal")]
    sched.shutdown_on_ctrl_c();

    sched.set_shutdown_handler(Box::new(|| {
        Box::pin(async move {
            println!("Shut down done");
        })
    }));

    let _ = sched.start().await.unwrap();
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