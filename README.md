# CLIsmith

> [!WARNING]
> wip

A cli framework for distributing modular and standardized cli tools generated from OpenAPI specs. CLIsmith allows you to distribute a single cli tool and the end-user can download the modules that fit their workflow.

## Dev

Run the example api compose `./examples/billing_api/docker-compose.yml`. From there you can use the dev harness to test commands against the project in `./examples/billing_api/openapi`:

```bash
cd ./scripts
bash dev-full.sh manifest list -o table
```

## License

CLIsmith is available under the following license:

- [Apache License 2.0](./LICENSE-APACHE)
