using BillingApi.Models;

namespace BillingApi.Data;

/// <summary>
/// Sample customers and invoices. Dates are relative to "today" so the data
/// always has a realistic mix of paid, open, overdue and draft invoices.
/// </summary>
internal static class SeedData
{
    public static void Load(BillingStore store, DateOnly today)
    {
        var now = new DateTimeOffset(today.ToDateTime(TimeOnly.MinValue), TimeSpan.Zero);

        var acme = store.AddCustomer(new()
        {
            Name = "Wile E. Rivera", Email = "billing@acmeanvils.example", Company = "Acme Anvils Co.",
            Phone = "+1-555-0101",
            BillingAddress = new("100 Desert Rd", "Suite 4", "Tucson", "AZ", "85701", "US"),
        }, now.AddDays(-240));

        var globex = store.AddCustomer(new()
        {
            Name = "Hank Scorpio", Email = "accounts@globex.example", Company = "Globex Corporation",
            Phone = "+1-555-0102",
            BillingAddress = new("1 Cypress Creek Pkwy", null, "Cypress Creek", "OR", "97001", "US"),
        }, now.AddDays(-200));

        var initech = store.AddCustomer(new()
        {
            Name = "Bill Lumbergh", Email = "ap@initech.example", Company = "Initech",
            Phone = "+1-555-0103",
            BillingAddress = new("4120 Freidrich Ln", null, "Austin", "TX", "78744", "US"),
        }, now.AddDays(-180));

        var umbrella = store.AddCustomer(new()
        {
            Name = "Alice Abernathy", Email = "finance@umbrella.example", Company = "Umbrella Health",
            Phone = "+44-20-7946-0958",
            BillingAddress = new("22 Raccoon St", null, "London", null, "EC1A 1BB", "GB"),
        }, now.AddDays(-150));

        var stark = store.AddCustomer(new()
        {
            Name = "Pepper Potts", Email = "invoices@starkindustries.example", Company = "Stark Industries",
            Phone = "+1-555-0105",
            BillingAddress = new("10880 Malibu Point", null, "Malibu", "CA", "90265", "US"),
        }, now.AddDays(-120));

        var wayne = store.AddCustomer(new()
        {
            Name = "Lucius Fox", Email = "payables@wayne.example", Company = "Wayne Enterprises",
            BillingAddress = new("1007 Mountain Dr", null, "Gotham", "NJ", "07001", "US"),
        }, now.AddDays(-90));

        var freelancer = store.AddCustomer(new()
        {
            Name = "Maya Chen", Email = "maya.chen@example.com", Phone = "+1-555-0107",
        }, now.AddDays(-45));

        var newCo = store.AddCustomer(new()
        {
            Name = "Diego Alvarez", Email = "hello@tinybakery.example", Company = "Tiny Bakery",
            BillingAddress = new("8 Calle Mayor", null, "Madrid", null, "28013", "ES"),
        }, now.AddDays(-7));

        // ---- Invoices ----
        Paid(store, acme.Id, today.AddDays(-210), 0.087m, "USD",
            ("Anvil, heavy duty", 12, 149.99m), ("Rocket skates", 2, 399.00m));
        Paid(store, acme.Id, today.AddDays(-120), 0.087m, "USD",
            ("Giant magnet", 1, 1250.00m), ("Shipping & handling", 1, 85.00m));
        Open(store, acme.Id, today.AddDays(-50), 0.087m, "USD", net: 30,
            ("Instant hole (portable)", 5, 79.50m));

        Paid(store, globex.Id, today.AddDays(-170), 0.0m, "USD",
            ("Consulting — Q2 strategy", 40, 225.00m));
        Open(store, globex.Id, today.AddDays(-10), 0.0m, "USD", net: 30,
            ("Consulting — Q4 strategy", 32, 225.00m), ("Travel expenses", 1, 1840.35m));

        Paid(store, initech.Id, today.AddDays(-150), 0.0825m, "USD",
            ("TPS report software license", 25, 48.00m));
        Open(store, initech.Id, today.AddDays(-75), 0.0825m, "USD", net: 30,
            ("Red stapler (premium)", 3, 24.99m), ("Printer maintenance", 1, 310.00m));
        Void(store, initech.Id, today.AddDays(-60), 0.0825m, "USD", "Duplicate of previous invoice",
            ("Red stapler (premium)", 3, 24.99m));

        Paid(store, umbrella.Id, today.AddDays(-100), 0.20m, "GBP",
            ("Lab equipment rental (monthly)", 3, 2400.00m));
        Open(store, umbrella.Id, today.AddDays(-15), 0.20m, "GBP", net: 45,
            ("Lab equipment rental (monthly)", 1, 2400.00m), ("Biohazard disposal", 1, 650.00m));

        Paid(store, stark.Id, today.AddDays(-80), 0.095m, "USD",
            ("Arc reactor maintenance", 1, 18500.00m));
        Open(store, stark.Id, today.AddDays(-5), 0.095m, "USD", net: 15,
            ("Suit polishing service", 8, 125.00m));

        Open(store, wayne.Id, today.AddDays(-40), 0.06625m, "USD", net: 30,
            ("Security audit", 1, 9800.00m), ("Kevlar samples", 10, 45.00m));
        Draft(store, wayne.Id, today, 0.06625m, "USD",
            ("Vehicle armor upgrade (estimate)", 1, 32000.00m));

        Paid(store, freelancer.Id, today.AddDays(-30), 0.0m, "USD",
            ("Logo design", 1, 750.00m), ("Brand guidelines PDF", 1, 300.00m));

        Draft(store, newCo.Id, today, 0.21m, "EUR",
            ("Point-of-sale setup", 1, 499.00m), ("Monthly subscription", 1, 39.00m));
    }

    private static CreateInvoiceRequest Req(int customerId, DateOnly issue, decimal tax, string currency, int net,
        string? notes, (string Desc, decimal Qty, decimal Price)[] items) => new()
    {
        CustomerId = customerId,
        IssueDate = issue,
        DueDate = issue.AddDays(net),
        TaxRate = tax,
        Currency = currency,
        Notes = notes,
        LineItems = items.Select(i => new CreateLineItemRequest
            { Description = i.Desc, Quantity = i.Qty, UnitPrice = i.Price }).ToList(),
    };

    private static void Paid(BillingStore s, int cid, DateOnly issue, decimal tax, string cur,
        params (string, decimal, decimal)[] items)
    {
        var paidOn = issue.AddDays(12).ToDateTime(new TimeOnly(15, 30));
        s.AddInvoice(Req(cid, issue, tax, cur, 30, null, items), InvoiceStatus.Paid,
            new DateTimeOffset(paidOn, TimeSpan.Zero));
    }

    // "Sent" invoices; ones past their due date are reported as Overdue automatically.
    private static void Open(BillingStore s, int cid, DateOnly issue, decimal tax, string cur, int net,
        params (string, decimal, decimal)[] items) =>
        s.AddInvoice(Req(cid, issue, tax, cur, net, null, items), InvoiceStatus.Sent);

    private static void Draft(BillingStore s, int cid, DateOnly issue, decimal tax, string cur,
        params (string, decimal, decimal)[] items) =>
        s.AddInvoice(Req(cid, issue, tax, cur, 30, null, items), InvoiceStatus.Draft);

    private static void Void(BillingStore s, int cid, DateOnly issue, decimal tax, string cur, string notes,
        params (string, decimal, decimal)[] items) =>
        s.AddInvoice(Req(cid, issue, tax, cur, 30, notes, items), InvoiceStatus.Void);
}
