pub struct PlaySound {
    /// * `expression` - Text to represent the schedule.
    pub job_schedule: String,

    /// * `file_path` - Sound file path.
    pub file_path: String,
}

impl PlaySound {
    pub fn new(job_schedule: String, file_path: String) -> Self {
        Self { job_schedule, file_path }
    }

    pub fn job_schedule(&self) -> &str {
        self.job_schedule.as_str()
    }

    pub fn file_path(&self) -> String {
        self.file_path.clone()
    }
}