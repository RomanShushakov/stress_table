SELECT id, username FROM stress_table_schema.users
WHERE email = $1 AND password = $2;
