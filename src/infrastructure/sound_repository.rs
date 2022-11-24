
use uuid::Uuid;
use crate::domain::entity::sound::Sound;
use crate::domain::error::kind::ProcessError;
use crate::domain::repository::sound_repository::SoundRepository;

pub struct LocalSoundRepository {
    directory: String,
}

impl LocalSoundRepository {
    pub fn new(directory: String) -> Self {
        Self { directory }
    }
}

impl SoundRepository for LocalSoundRepository {
    fn find_all(&self) -> Result<Vec<Sound>, ProcessError> {
        let sound = Sound {
            id: Uuid::new_v4(),
            title: "title0".to_string(),
            format: "flac".to_string(),
            storage_path: "/media/sound.flac".to_string()
        };
        Ok(vec![sound])
    }
}