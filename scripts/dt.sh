#!/bin/bash

cd "$(dirname "$0")" || exit
pwd=$(pwd)

# 开启自动导出
set -a
# shellcheck disable=SC1091
source "${pwd}/../.env"
# 关闭自动导出
set +a


DIR_PG=${pwd}/../src/db/postgres
SCHEMA_PG=${DIR_PG}/welds.yaml

mkdir -p "${DIR_PG}"
welds --database-url "${DB_PG}" --schema-file "${SCHEMA_PG}" update
welds --schema-file "${SCHEMA_PG}" generate

# DIR_MSSQL=${pwd}/../src/db/mssql
# SCHEMA_MSSQL=${DIR_MSSQL}/welds.yaml

# mkdir -p "${DIR_MSSQL}"
# welds --database-url "${DB_MSSQL}" --schema-file "${SCHEMA_MSSQL}" update
# welds --schema-file "${SCHEMA_MSSQL}" generate
