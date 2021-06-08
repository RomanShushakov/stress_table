DROP SCHEMA IF EXISTS stress_table_schema CASCADE;
CREATE SCHEMA stress_table_schema AUTHORIZATION stress_table_user;

CREATE TABLE stress_table_schema.users (
	id uuid NOT NULL,
	first_name VARCHAR(50) NOT NULL,
	last_name VARCHAR(50) NOT NULL,
    email VARCHAR(100) NOT NULL,
    password VARCHAR(100) NOT NULL,
    is_active BOOLEAN NOT NULL,
    is_superuser BOOLEAN NOT NULL,
    username VARCHAR(2) NOT NULL,
	PRIMARY KEY (id),
	UNIQUE (email)
);

ALTER TABLE stress_table_schema.users OWNER TO stress_table_user;
