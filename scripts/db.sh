#!/bin/bash

cd "$(dirname "$0")" || exit
pwd=$(pwd)

rm -f "${pwd}"/../db/mm.sql && cat "${pwd}"/../db/*.sql > "${pwd}"/../init.d/mm.sql && sed -i '1iDROP SCHEMA public CASCADE;\nCREATE SCHEMA public;\nGRANT ALL ON SCHEMA public TO public;\n' "${pwd}"/../init.d/mm.sql && cat "${pwd}"/../init.d/mm.sql "${pwd}"/../db/data > "${pwd}"/../db/mm.sql && mv "${pwd}"/../db/mm.sql "${pwd}"/../init.d/
