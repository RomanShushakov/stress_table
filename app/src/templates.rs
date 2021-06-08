use crate::models;
use askama::Template;


#[derive(Template)]
#[template(path = "index.html")]
pub struct AuthorizedUserInfo<'a>
{
    pub username: &'a str,
}
