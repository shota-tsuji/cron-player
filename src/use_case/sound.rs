
use crate::domain::entity::sound::Sound;
use crate::domain::error::ProcessError;

pub trait SoundUseCase {
    fn find_all_sounds(&self) -> Result<Vec<Sound>, ProcessError>;
}