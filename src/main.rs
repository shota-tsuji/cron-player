use std::fmt::Formatter;
use std::io::BufReader;
use std::{env, fmt};
use tokio_cron_scheduler::{Job, JobScheduler};

mod service;

use service::job_creation_service::JobCreateService;

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
