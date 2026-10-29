using BillingApi.Models;

namespace BillingApi.Data;

/// <summary>
/// Thread-safe in-memory store. Data is re-seeded every time the app starts.
/// </summary>
public sealed class BillingStore
{
    private readonly Lock _gate = new();
    private readonly Dictionary<int, Customer> _customers = [];
    private readonly Dictionary<int, Invoice> _invoices = [];
    private readonly TimeProvider _time;
    private int _nextCustomerId = 1;
    private int _nextInvoiceId = 1;

    public BillingStore(TimeProvider time)
    {
        _time = time;
        SeedData.Load(this, Today);
    }

    private DateOnly Today => DateOnly.FromDateTime(_time.GetUtcNow().UtcDateTime);

    // ---------- Customers ----------

    public IReadOnlyList<Customer> GetCustomers(string? search)
    {
        lock (_gate)
        {
            IEnumerable<Customer> q = _customers.Values;
            if (!string.IsNullOrWhiteSpace(search))
            {
                q = q.Where(c =>
                    c.Name.Contains(search, StringComparison.OrdinalIgnoreCase) ||
                    c.Email.Contains(search, StringComparison.OrdinalIgnoreCase) ||
                    (c.Company?.Contains(search, StringComparison.OrdinalIgnoreCase) ?? false));
            }
            return q.OrderBy(c => c.Id).ToList();
        }
    }

    public Customer? GetCustomer(int id)
    {
        lock (_gate) return _customers.GetValueOrDefault(id);
    }

    public bool EmailExists(string email)
    {
        lock (_gate) return _customers.Values.Any(c => c.Email.Equals(email, StringComparison.OrdinalIgnoreCase));
    }

    public Customer AddCustomer(CreateCustomerRequest req, DateTimeOffset? createdAt = null)
    {
        lock (_gate)
        {
            var customer = new Customer
            {
                Id = _nextCustomerId++,
                Name = req.Name.Trim(),
                Email = req.Email.Trim(),
                Company = req.Company?.Trim(),
                Phone = req.Phone?.Trim(),
                BillingAddress = req.BillingAddress,
                CreatedAt = createdAt ?? _time.GetUtcNow(),
            };
            _customers[customer.Id] = customer;
            return customer;
        }
    }

    // ---------- Invoices ----------

    public IReadOnlyList<Invoice> GetInvoices(int? customerId, InvoiceStatus? status)
    {
        lock (_gate)
        {
            IEnumerable<Invoice> q = _invoices.Values.Select(WithEffectiveStatus);
            if (customerId is not null) q = q.Where(i => i.CustomerId == customerId);
            if (status is not null) q = q.Where(i => i.Status == status);
            return q.OrderBy(i => i.Id).ToList();
        }
    }

    public Invoice? GetInvoice(int id)
    {
        lock (_gate)
        {
            return _invoices.TryGetValue(id, out var inv) ? WithEffectiveStatus(inv) : null;
        }
    }

    public Invoice AddInvoice(CreateInvoiceRequest req, InvoiceStatus status = InvoiceStatus.Draft, DateTimeOffset? paidAt = null)
    {
        lock (_gate)
        {
            var issue = req.IssueDate ?? Today;
            var id = _nextInvoiceId++;
            var invoice = new Invoice
            {
                Id = id,
                Number = $"INV-{issue.Year}-{id:D4}",
                CustomerId = req.CustomerId,
                Status = status,
                Currency = req.Currency.ToUpperInvariant(),
                IssueDate = issue,
                DueDate = req.DueDate ?? issue.AddDays(30),
                PaidAt = paidAt,
                Notes = req.Notes,
                TaxRate = req.TaxRate,
                LineItems = req.LineItems
                    .Select(li => new LineItem(li.Description.Trim(), li.Quantity, li.UnitPrice))
                    .ToList(),
            };
            _invoices[id] = invoice;
            return invoice;
        }
    }

    /// <summary>Applies a status transition. Returns the updated invoice, or an error message.</summary>
    public (Invoice? Invoice, string? Error) Transition(int id, InvoiceStatus target)
    {
        lock (_gate)
        {
            if (!_invoices.TryGetValue(id, out var inv)) return (null, null);
            var current = WithEffectiveStatus(inv).Status;

            var allowed = (current, target) switch
            {
                (InvoiceStatus.Draft, InvoiceStatus.Sent) => true,
                (InvoiceStatus.Sent or InvoiceStatus.Overdue, InvoiceStatus.Paid) => true,
                (InvoiceStatus.Draft or InvoiceStatus.Sent or InvoiceStatus.Overdue, InvoiceStatus.Void) => true,
                _ => false,
            };
            if (!allowed) return (null, $"Cannot change invoice from {current} to {target}.");

            var updated = inv with
            {
                Status = target,
                PaidAt = target == InvoiceStatus.Paid ? _time.GetUtcNow() : inv.PaidAt,
            };
            _invoices[id] = updated;
            return (WithEffectiveStatus(updated), null);
        }
    }

    /// <summary>A sent invoice whose due date has passed is reported as Overdue.</summary>
    private Invoice WithEffectiveStatus(Invoice inv) =>
        inv.Status == InvoiceStatus.Sent && inv.DueDate < Today
            ? inv with { Status = InvoiceStatus.Overdue }
            : inv;
}
