use uuid::Uuid;

#[derive(Debug)]
pub struct Sound {
    pub id: Uuid,
    pub title: String,
    pub format: String,
    pub storage_path: String,
}