#!/bin/bash

if [ -z "$1" ]; then
  echo "Usage: $0 <environment>"
  echo "Environments: dev, prod, local"
  exit 1
fi

ENVIRONMENT=$1
ENV_FILE=".env.$ENVIRONMENT"

# Ignorer .env.example
if [ "$ENV_FILE" == ".env.example" ]; then
  echo "Error: .env.example is not a valid environment file!"
  exit 1
fi

if [ ! -f "$ENV_FILE" ]; then
  echo "Error: File $ENV_FILE not found!"
  exit 1
fi

source "$ENV_FILE"

required_vars=("DB_HOST" "DB_PORT" "DB_USER" "DB_PASSWORD" "DB_NAME" "SRV_HOST" "SRV_PORT")
for var in "${required_vars[@]}"; do
  if [ -z "${!var}" ]; then
    echo "Error: Variable $var is not set in $ENV_FILE!"
    exit 1
  fi
done

DATABASE_URL="postgres://${DB_USER}:${DB_PASSWORD}@${DB_HOST}:${DB_PORT}/${DB_NAME}"

cat <<EOF > .env
# PostgreSQL
DB_HOST=${DB_HOST}
DB_PORT=${DB_PORT}
DB_USER=${DB_USER}
DB_PASSWORD=${DB_PASSWORD}
DB_NAME=${DB_NAME}
CONTAINER_DB_NAME=${CONTAINER_DB_NAME}

# Sqlx
DATABASE_URL=${DATABASE_URL}

# Api
SRV_HOST=${SRV_HOST}
SRV_PORT=${SRV_PORT}
EOF

echo ".env file generated successfully for environment: $ENVIRONMENT"