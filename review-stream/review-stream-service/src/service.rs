



#[derive(Clone)]
pub struct ReviewStreamService {
    //##PLOP INSERT COMMAND HOOK##
    // Add service infra dependencies here
}

impl Default for ReviewStreamService {
    fn default() -> Self {
        Self::new()
    }
}

impl ReviewStreamService {
    pub fn new() -> Self {
        Self {
            //##PLOP INSERT COMMAND INSTANTIATION HOOK##
        }
    }
    //##PLOP INSERT DELEGATE HOOK##
}
