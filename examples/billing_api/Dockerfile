# syntax=docker/dockerfile:1

# ---------- build ----------
FROM mcr.microsoft.com/dotnet/sdk:10.0 AS build
WORKDIR /src

# Restore first so package downloads are cached between code changes.
COPY src/BillingApi/BillingApi.csproj src/BillingApi/
RUN dotnet restore src/BillingApi/BillingApi.csproj

COPY src/ src/
# The OpenAPI YAML is generated on your machine's build and committed in /openapi;
# the image serves it live at /openapi/v1.yaml, so skip regenerating it here.
RUN dotnet publish src/BillingApi/BillingApi.csproj \
      -c Release -o /app/publish --no-restore \
      -p:OpenApiExportOnBuild=false \
      -p:UseAppHost=false

# ---------- runtime ----------
FROM mcr.microsoft.com/dotnet/aspnet:10.0 AS runtime
WORKDIR /app

ENV ASPNETCORE_HTTP_PORTS=8080 \
    ASPNETCORE_ENVIRONMENT=Production \
    DOTNET_RUNNING_IN_CONTAINER=true

COPY --from=build /app/publish .

# Run as the non-root user that ships with the .NET images.
USER $APP_UID
EXPOSE 8080

ENTRYPOINT ["dotnet", "BillingApi.dll"]
