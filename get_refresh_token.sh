#!/usr/bin/env bash
set -euo pipefail

# Helper script to generate a long-lived Dropbox refresh token.

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
if [ -f "${SCRIPT_DIR}/.env" ]; then
  set -a
  # shellcheck source=/dev/null
  . "${SCRIPT_DIR}/.env"
  set +a
elif [ -f ".env" ]; then
  set -a
  # shellcheck source=/dev/null
  . "./.env"
  set +a
fi

APP_KEY="${DROPBOX_CLIENT_ID:-}"

echo "=== Dropbox Refresh Token Generator ==="
echo

if [ -z "${APP_KEY}" ]; then
  read -r -p "Enter App Key: " APP_KEY
fi

if [ -z "${APP_KEY}" ]; then
  echo "Error: App Key (DROPBOX_CLIENT_ID) cannot be empty."
  exit 1
fi

if [ -z "${DROPBOX_CLIENT_SECRET:-}" ]; then
  read -r -s -p "Enter App Secret: " APP_SECRET
  echo
else
  APP_SECRET="${DROPBOX_CLIENT_SECRET}"
fi

if [ -z "${APP_SECRET}" ]; then
  echo "Error: App Secret cannot be empty."
  exit 1
fi

AUTH_URL="https://www.dropbox.com/oauth2/authorize?client_id=${APP_KEY}&token_access_type=offline&response_type=code"

echo
echo "--- Step A ---"
echo "Open this link in your browser to authorize offline access:"
echo "${AUTH_URL}"
echo

echo "--- Step B ---"
read -r -p "Paste the authorization code displayed by Dropbox: " AUTH_CODE
echo

if [ -z "${AUTH_CODE}" ]; then
  echo "Error: Authorization code cannot be empty."
  exit 1
fi

echo "--- Step C ---"
echo "Requesting tokens from Dropbox..."
echo

RESPONSE=$(curl -s -X POST https://api.dropboxapi.com/oauth2/token \
  -d "code=${AUTH_CODE}" \
  -d "grant_type=authorization_code" \
  -u "${APP_KEY}:${APP_SECRET}")

echo "Response:"
if command -v jq >/dev/null 2>&1; then
  echo "${RESPONSE}" | jq .
else
  echo "${RESPONSE}"
fi
echo
