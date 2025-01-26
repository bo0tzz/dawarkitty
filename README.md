# dawarkitty

This is a synchronization tool to store historical data from the Tractive API in a Dawarich instance.

# Development

## Generate API clients

`docker run --user="$(id -u):$(id -g)" --rm -v "${PWD}:/local" openapitools/openapi-generator-cli generate -i https://github.com/Freika/dawarich/raw/refs/tags/0.23.5/swagger/v1/swagger.yaml -g rust -o /local/openapi/dawarich/client --skip-validate-spec --package-name dawarich`
