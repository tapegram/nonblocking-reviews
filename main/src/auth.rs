use std::collections::HashMap;

use axum::async_trait;
use axum_login::{AuthUser, AuthnBackend, UserId};

/**
* This is the in memory implementation of the AuthnBackend trait from auth-login
* https://docs.rs/axum-login/latest/axum_login/
*
* Swap this out with a real implementation at your leisure!
*/

#[derive(Debug, Clone)]
pub struct User {
    id: i64,
    access_token: String,
}

impl AuthUser for User {
    type Id = i64;

    fn id(&self) -> Self::Id {
        self.id
    }

    fn session_auth_hash(&self) -> &[u8] {
        self.access_token.as_bytes()
    }
}

#[derive(Clone, Default)]
pub struct Backend {
    users: HashMap<i64, User>,
}

#[derive(Clone)]
pub struct Credentials {
    user_id: i64,
}

#[async_trait]
impl AuthnBackend for Backend {
    type User = User;
    type Credentials = Credentials;
    type Error = std::convert::Infallible;

    async fn authenticate(
        &self,
        Credentials { user_id }: Self::Credentials,
    ) -> Result<Option<Self::User>, Self::Error> {
        Ok(self.users.get(&user_id).cloned())
    }

    async fn get_user(&self, user_id: &UserId<Self>) -> Result<Option<Self::User>, Self::Error> {
        Ok(self.users.get(user_id).cloned())
    }
}
