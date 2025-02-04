#!/usr/bin/env zsh
#
set -x
set -eo pipefail

#if ! [ -x "$(command -v psql)" ]; then
#  echo >&2 "Error: psql is not installed."
#  exit 1
#fi

#if ! [ -x "$(command -v sqlx)" ]; then 
#  echo >&2 "Error: sqlx is not installed."
#  exit 1
#fi
alias psql="nix shell nixpkgs#postgresql --command psql"

DB_USER=${POSTGRES_USER:=postgres}
DB_PASSWORD="${POSTGRES_PASSWORD:=postgres}"
DB_NAME="${POSTGRES_DB:=zerodb}"
DB_PORT="${POSTGRES_PORT:=5431}"

if [[ -z "${SKIP_DOCKER}" ]]
then
docker run \
  -e POSTGRES_USER=${DB_NAME} \
  -e POSTGRES_PASSWORD=${DB_PASSWORD} \
  -e POSTGRES_DB=${DB_NAME} \
  -p "$DB_PORT":5432 \
  --name zerodb \
  -d zerodb \
  postgres -N 1000
fi

export PGPASSWORD="${DB_PASSWORD}"
until psql -h "localhost" -U "${DB_USER}" -p "${DB_PORT}" -d "zerodb" -c '\q'; do 
  >&2 echo "Postgres is unavailable - sleeping"
  sleep 1
done

echo "Postgres is up and running on port ${DB_PORT} - running migrations now!"

export DATABASE_URL=postgres://${DB_USER}:${DB_PASSWORD}@localhost:${DB_PORT}/${DB_NAME}
sqlx database create
sqlx migrate run
