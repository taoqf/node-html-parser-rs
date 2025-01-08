#!/bin/bash

cd "$(dirname "$0")" || exit
pwd=$(pwd)

# DB_PG=postgres://01factory:01factory@db:5432/01factory
# DIR_PG=${pwd}/../src/db/postgres
# SCHEMA_PG=${DIR_PG}/welds.yaml

# mkdir -p "${DIR_PG}"
# welds --database-url "${DB_PG}" --schema-file "${SCHEMA_PG}" update
# welds --schema-file "${SCHEMA_PG}" generate

DB_MSSQL='server=0.0.0.0,1433;database=dbname;user id=sa;password=psw;TrustServerCertificate=true;'
DIR_MSSQL=${pwd}/../src/db/mssql
SCHEMA_MSSQL=${DIR_MSSQL}/welds.yaml

mkdir -p "${DIR_MSSQL}"
welds --database-url "${DB_MSSQL}" --schema-file "${SCHEMA_MSSQL}" update
welds --schema-file "${SCHEMA_MSSQL}" generate
