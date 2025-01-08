#!/bin/bash

cd "$(dirname "$0")" || exit
pwd=$(pwd)

DB_PG=postgres://01factory:01factory@db:5432/01factory
# DB_MSSQL='server=0.0.0.0;port=1433;database=d01factory;schema=dbo;user id=sa;password=welds!123;TrustServerCertificate=true;'

DIR_PG=${pwd}/../src/db/postgres
DIR_MSSQL=${pwd}/../src/db/mssql

SCHEMA_PG=${DIR_PG}/welds.yaml
SCHEMA_MSSQL=${DIR_MSSQL}/welds.yaml

mkdir -p ${DIR_PG}
welds --database-url "${DB_PG}" --schema-file ${SCHEMA_PG} update
welds --schema-file ${SCHEMA_PG} generate

# mkdir -p ${DIR_MSSQL}
# welds --database-url "${DB_MSSQL}" --schema-file ${SCHEMA_MSSQL} update
# welds --schema-file ${SCHEMA_MSSQL} generate
