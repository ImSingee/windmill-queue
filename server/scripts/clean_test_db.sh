#!/bin/bash

if [[ -z "${DATABASE_URL}" ]]; then
  echo >&2 "Error: DATABASE_URL is not set."
  exit 1
fi

# 指定数据库前缀
DB_PREFIX="wq_queue_test_"

# 列出所有匹配前缀的数据库名称
DATABASES=$(psql "$DATABASE_URL" -t -c "SELECT datname FROM pg_database WHERE datname LIKE '${DB_PREFIX}%'")

# 读取并删除每个数据库
echo "$DATABASES" | while read -r dbname; do
    if [ ! -z "$dbname" ]; then
        echo "Deleting database: $dbname"
        psql "$DATABASE_URL" -c "DROP DATABASE \"$dbname\""
    fi
done
