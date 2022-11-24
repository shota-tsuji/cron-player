use std::env;
use tokio_cron_scheduler::JobScheduler;

mod domain;
mod use_case;
mod infrastructure;

use crate::domain::service::job_creation_service::JobCreateService;
use crate::domain::service::job_execution_service::JobExecuteService;
use crate::domain::entity::job::PlaySound;
use crate::use_case::sound::SoundUseCase;

#[tokio::main]
async fn main() {
    let expression = "0 1/1 * * * * *";
    let sched = JobScheduler::new().await.unwrap();

    let file_path = env::args().nth(1).unwrap();
    let cmd = PlaySound::new(expression.to_string(), file_path.clone());
    let job = JobCreateService::create_sound_job(cmd);

    let mut job_execute_service = JobExecuteService::new(sched);
    job_execute_service.add(job).await;
    job_execute_service.start().await;

    let repository = infrastructure::sound_repository::LocalSoundRepository::new("/media/".to_string());
    let usecase = use_case::interactor::sound::SoundInteractor { sound_repository: repository };
    println!("{:?}", usecase.find_all_sounds());

    loop {
        tokio::time::sleep(core::time::Duration::from_millis(500)).await;
    }
}
