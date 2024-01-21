#!/bin/bash

source .env

docker pull jlai241/stacc-api:$API_VERSION
docker compose down
docker compose --env-file .env --env-file .compose-env up -d api
