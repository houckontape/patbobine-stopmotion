use crate::models::project::Project;

/// Port métier : stockage des projets.
pub trait ProjectRepository {
    fn save(&self, project: &Project) -> Result<(), String>;
    fn load(&self, id: uuid::Uuid) -> Result<Project, String>;
}
