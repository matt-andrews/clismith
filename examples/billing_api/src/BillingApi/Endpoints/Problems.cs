global using ProblemDetails = Microsoft.AspNetCore.Mvc.ProblemDetails;

namespace BillingApi.Endpoints;

internal static class Problems
{
    public static ProblemDetails NotFound(string detail) => new()
    {
        Status = StatusCodes.Status404NotFound,
        Title = "Not Found",
        Detail = detail,
    };

    public static ProblemDetails Conflict(string detail) => new()
    {
        Status = StatusCodes.Status409Conflict,
        Title = "Conflict",
        Detail = detail,
    };
}
