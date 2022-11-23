use tokio_cron_scheduler::{Job, JobScheduler};

pub struct JobExecuteService {
    scheduler: JobScheduler,
}

impl JobExecuteService {
    pub fn new(scheduler: JobScheduler) -> Self {
        Self { scheduler }
    }

    pub async fn start(&mut self) {
        #[cfg(feature = "signal")]
        self.scheduler.s.shutdown_on_ctrl_c();

        self.scheduler.set_shutdown_handler(Box::new(|| {
            Box::pin(async move {
                println!("Shut down done");
            })
        }));
        let _ = self.scheduler.start().await.unwrap();
    }

    pub async fn add(&self, job: Job) {
        self.scheduler.add(job).await.unwrap();
    }
}

