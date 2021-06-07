INSERT INTO actix_pg_db_test.users(email, first_name, last_name, username)
VALUES ($1, $2, $3, $4)
RETURNING $table_fields;
