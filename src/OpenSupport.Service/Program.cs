using Microsoft.Extensions.DependencyInjection;
using Microsoft.Extensions.Hosting;
using Microsoft.Extensions.Logging;
using OpenSupport.Service;

var builder = Host.CreateApplicationBuilder(args);

// Configuración nativa del Servicio de Windows (SCM)
builder.Services.AddWindowsService(options =>
{
    options.ServiceName = "OpenSupportAgent";
});

builder.Services.AddHostedService<Worker>();

var host = builder.Build();
host.Run();
