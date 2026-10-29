using System.ComponentModel.DataAnnotations;

namespace BillingApi.Models;

/// <summary>A billable customer account.</summary>
public sealed record Customer
{
    public required int Id { get; init; }
    public required string Name { get; init; }
    public required string Email { get; init; }
    public string? Company { get; init; }
    public string? Phone { get; init; }
    public Address? BillingAddress { get; init; }
    public required DateTimeOffset CreatedAt { get; init; }
}

public sealed record Address(
    [property: Required] string Line1,
    string? Line2,
    [property: Required] string City,
    string? Region,
    [property: Required] string PostalCode,
    [property: Required, StringLength(2, MinimumLength = 2)] string Country);

public enum InvoiceStatus
{
    Draft,
    Sent,
    Paid,
    Overdue,
    Void
}

/// <summary>An invoice issued to a customer.</summary>
public sealed record Invoice
{
    public required int Id { get; init; }
    public required string Number { get; init; }
    public required int CustomerId { get; init; }
    public required InvoiceStatus Status { get; init; }
    public required string Currency { get; init; }
    public required DateOnly IssueDate { get; init; }
    public required DateOnly DueDate { get; init; }
    public DateTimeOffset? PaidAt { get; init; }
    public string? Notes { get; init; }
    public required IReadOnlyList<LineItem> LineItems { get; init; }

    /// <summary>Tax rate as a fraction, e.g. 0.08 for 8%.</summary>
    public required decimal TaxRate { get; init; }

    public decimal Subtotal => Math.Round(LineItems.Sum(li => li.Amount), 2);
    public decimal Tax => Math.Round(Subtotal * TaxRate, 2);
    public decimal Total => Subtotal + Tax;
}

public sealed record LineItem(string Description, decimal Quantity, decimal UnitPrice)
{
    public decimal Amount => Math.Round(Quantity * UnitPrice, 2);
}

// ---------- Requests ----------

public sealed record CreateCustomerRequest
{
    [Required, StringLength(200, MinimumLength = 1)]
    public string Name { get; init; } = "";

    [Required, EmailAddress]
    public string Email { get; init; } = "";

    [StringLength(200)]
    public string? Company { get; init; }

    [Phone]
    public string? Phone { get; init; }

    public Address? BillingAddress { get; init; }
}

public sealed record CreateInvoiceRequest
{
    [Range(1, int.MaxValue)]
    public int CustomerId { get; init; }

    /// <summary>ISO 4217 currency code. Defaults to USD.</summary>
    [StringLength(3, MinimumLength = 3)]
    public string Currency { get; init; } = "USD";

    /// <summary>Defaults to today (UTC).</summary>
    public DateOnly? IssueDate { get; init; }

    /// <summary>Defaults to 30 days after the issue date.</summary>
    public DateOnly? DueDate { get; init; }

    /// <summary>Tax rate as a fraction, e.g. 0.08 for 8%.</summary>
    [Range(0, 1)]
    public decimal TaxRate { get; init; }

    [StringLength(1000)]
    public string? Notes { get; init; }

    [Required, MinLength(1)]
    public List<CreateLineItemRequest> LineItems { get; init; } = [];
}

public sealed record CreateLineItemRequest
{
    [Required, StringLength(500, MinimumLength = 1)]
    public string Description { get; init; } = "";

    [Range(0.01, 1_000_000)]
    public decimal Quantity { get; init; } = 1;

    [Range(0, 10_000_000)]
    public decimal UnitPrice { get; init; }
}
