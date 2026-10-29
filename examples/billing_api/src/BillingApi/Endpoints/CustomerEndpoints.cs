using BillingApi.Data;
using BillingApi.Models;
using Microsoft.AspNetCore.Http.HttpResults;

namespace BillingApi.Endpoints;

public static class CustomerEndpoints
{
    public static RouteGroupBuilder MapCustomerEndpoints(this IEndpointRouteBuilder app)
    {
        var group = app.MapGroup("/customers").WithTags("Customers");

        group.MapGet("/", (BillingStore store, string? search) => TypedResults.Ok(store.GetCustomers(search)))
            .WithName("ListCustomers")
            .WithSummary("List customers")
            .WithDescription("Returns all customers. Optionally filter by a case-insensitive search on name, email or company.");

        group.MapGet("/{id:int}", Results<Ok<Customer>, NotFound<ProblemDetails>> (int id, BillingStore store) =>
                store.GetCustomer(id) is { } c
                    ? TypedResults.Ok(c)
                    : TypedResults.NotFound(Problems.NotFound($"Customer {id} was not found.")))
            .WithName("GetCustomer")
            .WithSummary("Get a customer by id");

        group.MapGet("/{id:int}/invoices", Results<Ok<IReadOnlyList<Invoice>>, NotFound<ProblemDetails>> (
                int id, InvoiceStatus? status, BillingStore store) =>
                store.GetCustomer(id) is null
                    ? TypedResults.NotFound(Problems.NotFound($"Customer {id} was not found."))
                    : TypedResults.Ok(store.GetInvoices(id, status)))
            .WithName("ListCustomerInvoices")
            .WithSummary("List a customer's invoices");

        group.MapPost("/", Results<Created<Customer>, Conflict<ProblemDetails>> (
                CreateCustomerRequest request, BillingStore store) =>
            {
                if (store.EmailExists(request.Email))
                    return TypedResults.Conflict(Problems.Conflict($"A customer with email '{request.Email}' already exists."));

                var customer = store.AddCustomer(request);
                return TypedResults.Created($"/customers/{customer.Id}", customer);
            })
            .WithName("CreateCustomer")
            .WithSummary("Create a customer")
            .ProducesValidationProblem();

        return group;
    }
}
