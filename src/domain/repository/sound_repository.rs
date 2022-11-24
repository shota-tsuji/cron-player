
use crate::domain::entity::sound::Sound;
use crate::domain::error::kind::ProcessError;

pub trait SoundRepository {
    fn find_all(&self) -> Result<Vec<Sound>, ProcessError>;
}