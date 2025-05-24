#!/bin/bash
# Script to test authentication

# Base URL
BASE_URL="http://localhost:8080"

# Get credentials from environment variables or use defaults
USER_EMAIL=${USER_EMAIL:-"admin"}
USER_PASSWORD=${USER_PASSWORD:-"admin"}

echo "Testing login..."
LOGIN_RESPONSE=$(curl -s -X POST "${BASE_URL}/auth/login" \
  -H "Content-Type: application/json" \
  -d '{
    "username": "'${USER_EMAIL}'",
    "password": "'${USER_PASSWORD}'"
  }')

echo "Login response: $LOGIN_RESPONSE"

# Extract token
TOKEN=$(echo $LOGIN_RESPONSE | grep -o '"token":"[^"]*' | sed 's/"token":"//')

if [ -z "$TOKEN" ]; then
  echo "Error: Failed to authenticate. Response: $LOGIN_RESPONSE"
  exit 1
fi

echo "Successfully authenticated with token: $TOKEN"
