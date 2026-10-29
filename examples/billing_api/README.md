# Billing API

An example ASP.NET Core (.NET 10) minimal API for a billing domain: customers and invoices,
seeded with sample data on startup.

## Run it

**Docker (stays running, port 5080):**

```bash
docker compose up -d --build
```

The container restarts automatically (`restart: unless-stopped`) and is always reachable at
**http://localhost:5080**. Change the left side of `5080:8080` in `docker-compose.yml` to use a different host port.

**Locally:**

```bash
cd src/BillingApi
dotnet run          # http://localhost:5080
```

(Docker and `dotnet run` both use 5080, so run one at a time.)

## Explore

| URL | What |
| --- | --- |
| `/docs` | Interactive API reference (Scalar) — try requests in the browser |
| `/openapi/v1.yaml` | Live OpenAPI document (YAML) |
| `/openapi/v1.json` | Live OpenAPI document (JSON) |
| `/health` | Health check |

`src/BillingApi/BillingApi.http` has ready-made requests for VS Code (REST Client), Visual Studio and Rider.

## Endpoints

| Method | Path | Description |
| --- | --- | --- |
| GET | `/customers?search=` | List customers (search name/email/company) |
| GET | `/customers/{id}` | Get a customer |
| GET | `/customers/{id}/invoices?status=` | A customer's invoices |
| POST | `/customers` | Create a customer (409 if the email exists) |
| GET | `/invoices?customerId=&status=` | List invoices |
| GET | `/invoices/{id}` | Get an invoice |
| POST | `/invoices` | Create an invoice (Draft; subtotal/tax/total are calculated) |
| POST | `/invoices/{id}/send` | Draft → Sent |
| POST | `/invoices/{id}/pay` | Sent/Overdue → Paid |
| POST | `/invoices/{id}/void` | Draft/Sent/Overdue → Void |

Invoice statuses: `Draft`, `Sent`, `Paid`, `Overdue`, `Void`. A `Sent` invoice past its due date
is reported as `Overdue` automatically. Errors use RFC 9457 problem details; validation errors return 400.

### Example

```bash
curl -X POST http://localhost:5080/invoices -H "Content-Type: application/json" -d '{
  "customerId": 1,
  "taxRate": 0.08,
  "lineItems": [{ "description": "Consulting", "quantity": 10, "unitPrice": 150 }]
}'
```

## Seed data

8 customers and 16 invoices covering every status, in USD, GBP and EUR (`src/BillingApi/Data/SeedData.cs`).
Dates are relative to today, so there's always a realistic mix of paid, open and overdue invoices.
Storage is in-memory: anything you create is lost when the app or container restarts.

## OpenAPI YAML at build time

Every `dotnet build` writes **`openapi/billing-api.yaml`** (OpenAPI 3.1). The `ExportOpenApiYaml`
target in `BillingApi.csproj` runs the compiled app with `--export-openapi <path>`, which generates
the document without opening a port and exits. It only re-runs when the compiled output changes.

- Change the output path: `dotnet build -p:OpenApiExportPath=some/other.yaml`
- Skip it: `dotnet build -p:OpenApiExportOnBuild=false` (the Docker build does this)

## Layout

```
├── Dockerfile
├── docker-compose.yml
├── openapi/billing-api.yaml        # generated on build
└── src/BillingApi/
    ├── Program.cs                  # wiring, OpenAPI, docs UI
    ├── OpenApiExport.cs            # build-time YAML export
    ├── Endpoints/                  # /customers and /invoices routes
    ├── Data/                       # in-memory store + seed data
    └── Models/                     # entities and request DTOs
```
