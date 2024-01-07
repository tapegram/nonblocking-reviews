use axum_login::AuthUser;
use std::fmt::Display;

#[derive(Clone, PartialEq)]
pub struct User {
    pub id: String,
    pub email: String,
    pub access_token: String,
    pub role: UserRole,
}
//
// Here we've implemented `Debug` manually to avoid accidentally logging the
// access token.
impl std::fmt::Debug for User {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("User")
            .field("id", &self.id)
            .field("email", &self.email)
            .field("access_token", &"[redacted]")
            .field("role", &self.role)
            .finish()
    }
}

impl User {
    pub fn new(email: String, access_token: String) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            email,
            access_token,
            role: UserRole::Organizer,
        }
    }
    pub fn update(&self, email: String, role: UserRole) -> Self {
        Self {
            id: self.id.clone(),
            email,
            access_token: self.access_token.clone(),
            role,
        }
    }
    pub fn has_perm(&self, permission: UserPermission) -> bool {
        self.role.has_perm(permission)
    }
}

/**
* Need to implement this for axum-login
*/
impl AuthUser for User {
    type Id = String;

    fn id(&self) -> Self::Id {
        self.id.clone()
    }

    fn session_auth_hash(&self) -> &[u8] {
        self.access_token.as_bytes()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum UserRole {
    Admin,
    Organizer,
    SuperAdmin,
}

impl UserRole {
    pub fn new<T: AsRef<str>>(role: T) -> Option<Self> {
        match role.as_ref() {
            "Organizer" => Some(Self::Organizer),
            "SuperAdmin" => Some(Self::SuperAdmin),
            "Admin" => Some(Self::Admin),
            _ => None,
        }
    }
    pub fn has_perm(&self, permission: UserPermission) -> bool {
        match self {
            Self::Organizer => match permission {},
            Self::Admin => match permission {},
            Self::SuperAdmin => true,
        }
    }
}

impl Display for UserRole {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let role = match self {
            Self::Organizer => "Organizer",
            Self::Admin => "Admin",
            Self::SuperAdmin => "SuperAdmin",
        };
        write!(f, "{}", role)
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum UserPermission {}

impl From<&str> for UserPermission {
    fn from(_permission: &str) -> Self {
        panic!("Permission does not exist")
    }
}
