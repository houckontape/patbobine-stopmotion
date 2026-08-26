use crate::domain::models::project::Project;

pub struct IntegrityService;

impl IntegrityService {
    pub fn check_project_integrity(&self, project: &Project) -> bool {
        // Règle métier : un projet doit avoir au moins une scène
        if project.scenes.is_empty() {
            return false;
        }

        // Règle métier : chaque scène doit avoir un nom
        if project.scenes.iter().any(|s| s.name.trim().is_empty()) {
            return false;
        }

        true
    }
}
