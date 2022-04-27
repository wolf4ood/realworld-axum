use domain::repositories::Repository;

use crate::repo::ArcRepo;

#[derive(Clone)]
pub struct ApplicationContext {
    repo: ArcRepo,
}

impl ApplicationContext {
    pub fn new(repo: impl Repository + 'static) -> Self {
        Self {
            repo: ArcRepo::new(repo),
        }
    }

    pub fn repo(&self) -> &ArcRepo {
        &self.repo
    }
}
