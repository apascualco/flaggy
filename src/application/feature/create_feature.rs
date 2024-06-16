use crate::domain::user_repository::UserRepository;

pub trait FeatureCreator<R> where R: UserRepository + Send + Sync {
    fn new(repository: R) -> Self;
    fn create_feature(&self);
}

pub struct FeatureCreatorService<R> where R: UserRepository + Send + Sync {
    #[allow(unused)]
    repository: R,
}

impl<R> FeatureCreator<R> for FeatureCreatorService<R> where R: UserRepository + Send + Sync {

    fn new(repository: R) -> Self {
        Self { repository: repository }
    }

    fn create_feature(&self) {
    }

}
