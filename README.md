# dawarkitty

This is a synchronization tool to store historical data from the Tractive API in a Dawarich instance.

# Running

Env vars:
- `DAWARICH_HOST`: URL of the Dawarich instance
- `DAWARICH_API_KEY`: API key for the Dawarich instance
- `TRACTIVE_EMAIL`: Email for the Tractive account
- `TRACTIVE_PASSWORD`: Password for the Tractive account
- `TRACTIVE_TRACKER_IDS`: Comma-separated list of tracker IDs to synchronize

# Development

## Generate API client

`docker run --user="$(id -u):$(id -g)" --rm -v "${PWD}:/local" openapitools/openapi-generator-cli generate -i https://github.com/Freika/dawarich/raw/refs/tags/0.23.5/swagger/v1/swagger.yaml -g rust -o /local/openapi/dawarich/client --skip-validate-spec --package-name dawarich`
