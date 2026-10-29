using System.Text.Json.Serialization;
using BillingApi;
using BillingApi.Data;
using BillingApi.Endpoints;
using Scalar.AspNetCore;

var builder = WebApplication.CreateBuilder(args);

builder.Services.AddSingleton(TimeProvider.System);
builder.Services.AddSingleton<BillingStore>();

builder.Services.AddProblemDetails();
builder.Services.AddValidation();
builder.Services.ConfigureHttpJsonOptions(o =>
{
    o.SerializerOptions.Converters.Add(new JsonStringEnumConverter());
    // Numbers are numbers (not "number or numeric string") — keeps the OpenAPI schema clean.
    o.SerializerOptions.NumberHandling = JsonNumberHandling.Strict;
});

builder.Services.AddOpenApi("v1", o =>
{
    o.AddDocumentTransformer((doc, _, _) =>
    {
        doc.Info.Title = "Billing API";
        doc.Info.Version = "v1";
        doc.Info.Description = "Example billing service with customers and invoices. Data is in-memory and re-seeded on startup.";
        return Task.CompletedTask;
    });
});

if (OpenApiExport.IsRequested(args))
{
    OpenApiExport.UseNoOpServer(builder.Services);
}

var app = builder.Build();

app.UseExceptionHandler();
app.UseStatusCodePages();

// OpenAPI docs are served in every environment so the container is easy to explore.
app.MapOpenApi("/openapi/{documentName}.json");
app.MapOpenApi("/openapi/{documentName}.yaml");
app.MapScalarApiReference("/docs", o => o.WithTitle("Billing API"));

app.MapGet("/", () => Results.Redirect("/docs")).ExcludeFromDescription();
app.MapGet("/health", () => TypedResults.Ok(new { status = "ok" })).WithTags("Health").WithName("Health");

app.MapCustomerEndpoints();
app.MapInvoiceEndpoints();

// Build-time OpenAPI export: writes YAML and exits instead of starting the server.
if (OpenApiExport.IsRequested(args))
{
    return await OpenApiExport.RunAsync(app, args);
}

await app.RunAsync();
return 0;
