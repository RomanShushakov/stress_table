import os

psql_user_secret_file = os.path.normpath("/run/secrets/psql_user")

with open(psql_user_secret_file, mode="r") as input_file:
    line = input_file.readline()
    psql_user = line[: -1] if line.endswith("\n") else line
    os.environ["PG.USER"] = psql_user

psql_pass_secret_file = os.path.normpath("/run/secrets/psql_pass")

with open(psql_pass_secret_file, mode="r") as input_file:
    line = input_file.readline()
    psql_pass = line[: -1] if line.endswith("\n") else line
    os.environ["PG.PASSWORD"] = psql_pass

psql_db_secret_file = os.path.normpath("/run/secrets/psql_db")

with open(psql_db_secret_file, mode="r") as input_file:
    line = input_file.readline()
    psql_db = line[: -1] if line.endswith("\n") else line
    os.environ["PG.DBNAME"] = psql_db
