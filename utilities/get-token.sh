#!/bin/bash

set -e

USER_ID=$1
PASSWORD=$2

USER_POOL_ID=$(aws cognito-idp list-user-pools --max-results 5 --output json | jq -r '.UserPools[] | select(.Name == "rust-demo") | .Id')
CLIENT_ID=$(aws cognito-idp list-user-pool-clients --user-pool-id "$USER_POOL_ID" --max-results 1 | jq -r '.UserPoolClients[0].ClientId')

aws cognito-idp admin-initiate-auth --user-pool-id "$USER_POOL_ID" --client-id "$CLIENT_ID" --auth-flow ADMIN_USER_PASSWORD_AUTH --auth-parameters USERNAME="$USER_ID",PASSWORD="$PASSWORD" | jq -r '.AuthenticationResult.IdToken'
