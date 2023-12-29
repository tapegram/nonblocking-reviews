use std::sync::Arc;

use crate::ports::push_repository::PushRepository;

#[derive(Clone)]
pub struct ReviewStreamService {
    //##PLOP INSERT COMMAND HOOK##
    pub push_repository: Arc<dyn PushRepository>,
}

impl ReviewStreamService {
    pub fn new(push_repository: Arc<dyn PushRepository>) -> Self {
        Self {
            //##PLOP INSERT COMMAND INSTANTIATION HOOK##
            push_repository,
        }
    }
    //##PLOP INSERT DELEGATE HOOK##
}
