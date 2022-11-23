use std::env;
use tokio_cron_scheduler::JobScheduler;

mod domain;

use crate::domain::service::job_creation_service::JobCreateService;
use crate::domain::service::job_execution_service::JobExecuteService;

#[tokio::main]
async fn main() {
    let expression = "0 1/1 * * * * *";
    let sched = JobScheduler::new().await.unwrap();

    let file_path = env::args().nth(1).unwrap();
    let job = JobCreateService::create_sound_job(expression, file_path.clone());

    let mut job_execute_service = JobExecuteService::new(sched);
    job_execute_service.add(job).await;
    job_execute_service.start().await;

    loop {
        tokio::time::sleep(core::time::Duration::from_millis(500)).await;
    }
}
