
use crate::domain::entity::sound::Sound;
use crate::domain::error::DomainError;

pub trait SoundRepository {
    fn find_all(&self) -> Result<Vec<Sound>, DomainError>;
}