use std::env;
use std::io::BufReader;
use chrono::Utc;
use tokio_cron_scheduler::{Job, JobScheduler};

fn play_sound(file: &String) {
    let (_stream, handle) = rodio::OutputStream::try_default().unwrap();
    let sink = rodio::Sink::try_new(&handle).unwrap();

    let file = std::fs::File::open(file).unwrap();
    sink.append(rodio::Decoder::new(BufReader::new(file)).unwrap());

    sink.sleep_until_end();
}

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);


    let expression = "0 1/1 * * * * *";
    let mut sched = JobScheduler::new().await.unwrap();

    let job = Job::new(expression, move |_uuid, _l| {
        println!("{:?}", Utc::now());
        play_sound(&args[1]);
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
