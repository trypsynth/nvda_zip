#!/bin/bash

git pull
docker compose down
docker rmi nvda_zip-nvda_zip
docker compose up -d
