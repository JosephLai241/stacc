#!/bin/bash

# This Bash script just extracts the version number from the API's `Cargo.toml`
# and writes it to the top level `.env` file, assigning it to the `API_VERSION`
# environment variable.

version_line=$(grep -m 1 "version =" api/Cargo.toml)
VERSION=$(echo "$version_line" | awk -F '"' '{print $2}')

echo "Got API version: v$VERSION"

echo "API_VERSION=${VERSION}" > .env

echo "New API version written to .env!"
