use askama::Template;

use crate::models;


#[derive(Template)]
#[template(path = "index.html")]
pub struct AuthorizedUserInfo<'a>
{
    pub username: &'a str,
}
