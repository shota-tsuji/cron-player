use std::error::Error;
use crate::domain::entity::sound::Sound;

pub trait SoundUseCase {
    fn find_all_sounds(&self) -> Result<Vec<Sound>, Error>;
}