using Microsoft.AspNetCore.Hosting.Server;
using Microsoft.AspNetCore.Http.Features;
using Microsoft.AspNetCore.OpenApi;
using Microsoft.OpenApi;

namespace BillingApi;

/// <summary>
/// Writes the OpenAPI document to a YAML file without listening on any port.
/// The build invokes this with: dotnet BillingApi.dll --export-openapi &lt;path.yaml&gt;
/// (see the ExportOpenApiYaml target in BillingApi.csproj).
/// </summary>
internal static class OpenApiExport
{
    private const string Flag = "--export-openapi";

    public static bool IsRequested(string[] args) => args.Contains(Flag);

    /// <summary>Swaps Kestrel for a server that never binds a socket.</summary>
    public static void UseNoOpServer(IServiceCollection services) =>
        services.AddSingleton<IServer, NoOpServer>();

    public static async Task<int> RunAsync(WebApplication app, string[] args, string documentName = "v1")
    {
        var index = Array.IndexOf(args, Flag);
        if (index < 0 || index + 1 >= args.Length)
        {
            Console.Error.WriteLine($"Usage: {Flag} <output.yaml>");
            return 1;
        }

        var outputPath = Path.GetFullPath(args[index + 1]);
        Directory.CreateDirectory(Path.GetDirectoryName(outputPath)!);

        // Starting the host builds the request pipeline, which registers the mapped
        // endpoints with routing so the OpenAPI generator can see them.
        await app.StartAsync();
        try
        {
            var provider = app.Services.GetRequiredKeyedService<IOpenApiDocumentProvider>(documentName);
            var document = await provider.GetOpenApiDocumentAsync();

            await using var file = File.Create(outputPath);
            await document.SerializeAsYamlAsync(file, OpenApiSpecVersion.OpenApi3_1);
        }
        finally
        {
            await app.StopAsync();
        }

        Console.WriteLine($"OpenAPI '{documentName}' written to {outputPath}");
        return 0;
    }

    private sealed class NoOpServer : IServer
    {
        public IFeatureCollection Features { get; } = new FeatureCollection();
        public Task StartAsync<TContext>(IHttpApplication<TContext> application, CancellationToken ct) where TContext : notnull => Task.CompletedTask;
        public Task StopAsync(CancellationToken ct) => Task.CompletedTask;
        public void Dispose() { }
    }
}
