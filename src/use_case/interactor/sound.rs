
use crate::domain::entity::sound::Sound;
use crate::domain::error::ProcessError;
use crate::domain::repository::sound_repository::SoundRepository;
use crate::use_case::sound::SoundUseCase;

pub struct SoundInteractor<SR> {
    pub sound_repository: SR,
}

impl<SR> SoundUseCase for SoundInteractor<SR>
where SR: SoundRepository,
{
    fn find_all_sounds(&self) -> Result<Vec<Sound>, ProcessError> {
        let sounds = self.sound_repository.find_all()?;
        Ok(sounds)
    }
}