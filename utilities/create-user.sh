#!/bin/bash

set -e

USER_ID=$1
PASSWORD=$2
EMAIL=$3
FIRST_NAME=$4
LAST_NAME=$5

USER_POOL_ID=$(aws cognito-idp list-user-pools --max-results 5 --output json | jq -r '.UserPools[] | select(.Name == "rust-demo") | .Id')

aws cognito-idp admin-create-user --user-pool-id "$USER_POOL_ID" --username "$USER_ID" --user-attributes Name=email,Value="$EMAIL" Name=given_name,Value="$FIRST_NAME" Name=family_name,Value="$LAST_NAME"
aws cognito-idp admin-set-user-password --user-pool-id "$USER_POOL_ID" --username "$USER_ID" --password "$PASSWORD" --permanent
aws cognito-idp admin-update-user-attributes --user-pool-id "$USER_POOL_ID" --username "$USER_ID" --user-attributes Name=email_verified,Value=true