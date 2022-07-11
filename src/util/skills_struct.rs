#[derive(Debug)]
pub struct SkillsStruct {
    pub name: String,
    pub current_level: i64,
    pub max_level: i64,
    pub total_exp: i64,
    pub exp_current: i64,
    pub exp_for_next: i64,
    pub progress_to_next: f64,
}

impl SkillsStruct {
    pub fn is_maxed(&self) -> bool {
        self.current_level == self.max_level
    }

    pub fn get_progress_level(&self) -> f64 {
        (self.exp_current as f64) + self.progress_to_next
    }
}
