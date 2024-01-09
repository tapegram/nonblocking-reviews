/**
 * This module is a dumping ground of of routes plus functions for hydrating those routes
*
* so a const like "/something/:something_id"
* and a function like fn something(id: String) -> String { format!("/something/{}", id) }
 *
* The idea is that this will make is easier to refactor routes in the future and to avoid passing
* around "magic strings." Additionally, this helps the dependency graph by not having weird
* circular dependencies between difference resources and components.
*
* The downside is that these routes are not colocated in the appropriate
* resource module.
 */

pub const HOME: &str = "/";
pub fn home() -> String {
    HOME.into()
}

#[cfg(debug_assertions)]
pub const HOME_REDIRECT: &str = PLAYGROUND;

#[cfg(not(debug_assertions))]
pub const HOME_REDIRECT: &str = FEED;

pub const PLAYGROUND: &str = "/playground";
pub fn _playground() -> String {
    PLAYGROUND.into()
}

pub const CLIENT: &str = "/client";
pub fn _client() -> String {
    CLIENT.into()
}

pub const LOGIN: &str = "/login";
pub fn login() -> String {
    LOGIN.into()
}

pub const GITHUB_AUTH_CALLBACK: &str = "/github-auth-callback";
pub fn github_auth_callback() -> String {
    GITHUB_AUTH_CALLBACK.into()
}

pub const FEED: &str = "/feed";
pub fn feed() -> String {
    FEED.into()
}

pub const REPOSITORIES: &str = "/repositories";
pub fn repositories() -> String {
    REPOSITORIES.into()
}

pub const REPOSITORIES_CREATE_FORM: &str = "/repositories/create-form";
pub fn repositories_create_form() -> String {
    REPOSITORIES_CREATE_FORM.into()
}

pub const REPOSITORY: &str = "/repositories/:repository_id";
pub fn repository(repository_id: String) -> String {
    format!("/repositories/{}", repository_id)
}

pub const REPOSITORY_EDIT_FORM: &str = "/repositories/:repository_id/edit-form";
pub fn repository_edit_form(repository_id: String) -> String {
    format!("/repositories/{}/edit-form", repository_id)
}

