INSERT INTO stress_table_schema.users(id, first_name, last_name, email, password, is_active, is_superuser, username)
VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
RETURNING $table_fields;
