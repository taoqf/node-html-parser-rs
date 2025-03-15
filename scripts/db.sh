#!/bin/bash

cd "$(dirname "$0")" || exit
pwd=$(pwd)

rm -f "${pwd}"/../db/01factory.sql && cat "${pwd}"/../db/*.sql > "${pwd}"/../init.d/01factory.sql && sed -i '1iDROP SCHEMA public CASCADE;\nCREATE SCHEMA public;\nGRANT ALL ON SCHEMA public TO public;\n' "${pwd}"/../init.d/01factory.sql && cat "${pwd}"/../init.d/01factory.sql "${pwd}"/../db/data > "${pwd}"/../db/01factory.sql && mv "${pwd}"/../db/01factory.sql "${pwd}"/../init.d/
