use deadpool_postgres::Client;
use tokio_pg_mapper::FromTokioPostgresRow;
use uuid::Uuid;

use crate::errors::MyError;
use crate::models::{UserDataFromClientForRegistration, User, UserDataFromClientForLogin, UserDataForLogin};


pub async fn add_user(client: &Client,
    user_data_from_client_for_registration: UserDataFromClientForRegistration)
    -> Result<(), MyError>
{
    let user = User::create(user_data_from_client_for_registration);

    let _stmt = include_str!("../sql/add_user.sql");
    let _stmt = _stmt.replace("$table_fields", &User::sql_table_fields());
    let stmt = client.prepare(&_stmt).await.unwrap();

    let _row = client
        .query_one(
            &stmt,
            &[
                &user.id,
                &user.first_name,
                &user.last_name,
                &user.email,
                &user.password,
                &user.is_active,
                &user.is_superuser,
                &user.username,
            ],
        )
        .await?;
    Ok(())
}


pub async fn get_user(client: &Client,
    user_data_from_client_for_login: UserDataFromClientForLogin)
    -> Result<(Uuid, String), MyError>
{
    let user_data_for_login =
        UserDataForLogin::create(user_data_from_client_for_login);

    let _stmt = include_str!("../sql/get_user.sql");
    let stmt = client.prepare(&_stmt).await.unwrap();

    let row = client
        .query_one(
            &stmt,
            &[
                &user_data_for_login.email,
                &user_data_for_login.password,
            ],
        )
        .await?;
    Ok((row.get("id"), row.get("username")))
}
