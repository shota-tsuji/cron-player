use std::error::Error;
use crate::domain::entity::sound::Sound;

pub trait SoundRepository {
    fn find_all(&self) -> Result<Vec<Sound>, Error>;
}