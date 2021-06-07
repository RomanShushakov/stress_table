DROP SCHEMA IF EXISTS actix_pg_db_test CASCADE;
CREATE SCHEMA actix_pg_db_test;

CREATE TABLE actix_pg_db_test.users (
	id  BIGSERIAL PRIMARY KEY,
	email       VARCHAR(200) NOT NULL,
	first_name  VARCHAR(200) NOT NULL,
	last_name   VARCHAR(200) NOT NULL,
	username    VARCHAR(50) UNIQUE NOT NULL,
	UNIQUE (username)
);
