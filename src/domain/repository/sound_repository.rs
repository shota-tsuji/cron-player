
use crate::domain::entity::sound::Sound;
use crate::domain::error::ProcessError;

pub trait SoundRepository {
    fn find_all(&self) -> Result<Vec<Sound>, ProcessError>;
}