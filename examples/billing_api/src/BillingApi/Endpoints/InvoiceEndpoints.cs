using BillingApi.Data;
using BillingApi.Models;
using Microsoft.AspNetCore.Http.HttpResults;

namespace BillingApi.Endpoints;

public static class InvoiceEndpoints
{
    public static RouteGroupBuilder MapInvoiceEndpoints(this IEndpointRouteBuilder app)
    {
        var group = app.MapGroup("/invoices").WithTags("Invoices");

        group.MapGet("/", (BillingStore store, int? customerId, InvoiceStatus? status) =>
                TypedResults.Ok(store.GetInvoices(customerId, status)))
            .WithName("ListInvoices")
            .WithSummary("List invoices")
            .WithDescription("Returns all invoices. Optionally filter by customerId and/or status.");

        group.MapGet("/{id:int}", Results<Ok<Invoice>, NotFound<ProblemDetails>> (int id, BillingStore store) =>
                store.GetInvoice(id) is { } inv
                    ? TypedResults.Ok(inv)
                    : TypedResults.NotFound(Problems.NotFound($"Invoice {id} was not found.")))
            .WithName("GetInvoice")
            .WithSummary("Get an invoice by id");

        group.MapPost("/", Results<Created<Invoice>, ValidationProblem> (CreateInvoiceRequest request, BillingStore store) =>
            {
                var errors = new Dictionary<string, string[]>();
                if (store.GetCustomer(request.CustomerId) is null)
                    errors["CustomerId"] = [$"Customer {request.CustomerId} does not exist."];
                var issue = request.IssueDate ?? DateOnly.FromDateTime(DateTime.UtcNow);
                if (request.DueDate is { } due && due < issue)
                    errors["DueDate"] = ["Due date cannot be before the issue date."];
                if (errors.Count > 0)
                    return TypedResults.ValidationProblem(errors);

                var invoice = store.AddInvoice(request);
                return TypedResults.Created($"/invoices/{invoice.Id}", invoice);
            })
            .WithName("CreateInvoice")
            .WithSummary("Create an invoice")
            .WithDescription("Creates a new invoice in Draft status. Subtotal, tax and total are calculated from the line items.");

        group.MapPost("/{id:int}/send", (int id, BillingStore store) => Transition(id, InvoiceStatus.Sent, store))
            .WithName("SendInvoice")
            .WithSummary("Mark a draft invoice as sent");

        group.MapPost("/{id:int}/pay", (int id, BillingStore store) => Transition(id, InvoiceStatus.Paid, store))
            .WithName("PayInvoice")
            .WithSummary("Record payment for a sent or overdue invoice");

        group.MapPost("/{id:int}/void", (int id, BillingStore store) => Transition(id, InvoiceStatus.Void, store))
            .WithName("VoidInvoice")
            .WithSummary("Void an unpaid invoice");

        return group;
    }

    private static Results<Ok<Invoice>, NotFound<ProblemDetails>, Conflict<ProblemDetails>> Transition(
        int id, InvoiceStatus target, BillingStore store)
    {
        var (invoice, error) = store.Transition(id, target);
        if (error is not null) return TypedResults.Conflict(Problems.Conflict(error));
        if (invoice is null) return TypedResults.NotFound(Problems.NotFound($"Invoice {id} was not found."));
        return TypedResults.Ok(invoice);
    }
}
