using Microsoft.VisualStudio.LanguageServer.Client;
using Microsoft.VisualStudio.Shell;
using Microsoft.VisualStudio.Utilities;
using StreamJsonRpc;
using System;
using System.Collections.Generic;
using System.Diagnostics;
using System.IO;
using System.Reflection;
using System.Threading;
using System.Threading.Tasks;
using Microsoft.VisualStudio.Threading;
using Task = System.Threading.Tasks.Task;
using System.ComponentModel.Composition;

namespace vs_log_extension
{
    //[ContentType("any")] // activates on opening any file (text, plaintext, code...) https://docs.microsoft.com/en-us/visualstudio/extensibility/language-service-and-editor-extension-points?view=vs-2022
    //[ContentType("code")]
    [ContentType("TypeScript")] // it seems it also gets enabled by JavaScript files...
    [Export(typeof(ILanguageClient))]
    [RunOnContext(RunningContext.RunOnHost)]
    public class LanguageClient : ILanguageClient, ILanguageClientCustomMessage2
    {

        [Import]
        internal IContentTypeRegistryService ContentTypeRegistryService { get; set; }

        public LanguageClient()
        {
            Instance = this;
        }

        internal static LanguageClient Instance
        {
            get;
            set;
        }

        internal JsonRpc Rpc
        {
            get;
            set;
        }

        public event AsyncEventHandler<EventArgs> StartAsync;
        public event AsyncEventHandler<EventArgs> StopAsync;

        public string Name => "JS Language Extension";

        public IEnumerable<string> ConfigurationSections
        {
            get
            {
                yield return "js";
            }
        }

        public object InitializationOptions => null;

        public IEnumerable<string> FilesToWatch => null;

        public object MiddleLayer
        {
            get;
            set;
        }

        public object CustomMessageTarget => null;

        public bool ShowNotificationOnInitializeFailed => true;

        public async Task<Connection> ActivateAsync(CancellationToken token)
        {
            // Debugger.Launch();

            await Task.Yield();

            ProcessStartInfo info = new ProcessStartInfo();
            var assemblyLocation = Path.GetDirectoryName(Assembly.GetExecutingAssembly().Location);
            var programPath = Path.Combine(assemblyLocation,  @"language-server.exe");
            // IMPORTANT: CHANGE THE PATH HERE ACCORDINGLY while developing
            // var programPath = @"H:\GIT\github\robertohuertasm\datadog-ls\language-server\target\release\language-server.exe";
            info.FileName = programPath;
            info.WorkingDirectory = Path.GetDirectoryName(programPath);
            info.RedirectStandardInput = true;
            info.RedirectStandardOutput = true;
            info.UseShellExecute = false;
            // uncomment this when releasing to production. This is only useful to see the logs from the lang server
            //info.CreateNoWindow = true;

            Process process = new Process
            {
                StartInfo = info
            };

            if (process.Start())
            {
                return new Connection(process.StandardOutput.BaseStream, process.StandardInput.BaseStream);
            }


            return null;
        }

        public async Task OnLoadedAsync()
        {
            if (StartAsync != null)
            {
                await StartAsync.InvokeAsync(this, EventArgs.Empty);
            }
        }

        public async Task StopServerAsync()
        {
            if (StopAsync != null)
            {
                await StopAsync.InvokeAsync(this, EventArgs.Empty);
            }
        }

        public Task OnServerInitializedAsync()
        {
            return Task.CompletedTask;
        }

        public Task AttachForCustomMessageAsync(JsonRpc rpc)
        {
            this.Rpc = rpc;

            return Task.CompletedTask;
        }

        public Task<InitializationFailureContext> OnServerInitializeFailedAsync(ILanguageClientInitializationInfo initializationState)
        {
            string message = "Oh no! Language Client failed to activate, now we can't test LSP! :(";
            string exception = initializationState.InitializationException?.ToString() ?? string.Empty;
            message = $"{message}\n {exception}";

            var failureContext = new InitializationFailureContext()
            {
                FailureMessage = message,
            };

            return Task.FromResult(failureContext);
        }

        //internal class MyMiddleLayer : ILanguageClientMiddleLayer
        //{
        //    public bool CanHandle(string methodName)
        //    {
        //        return methodName == Methods.TextDocumentCompletionName;
        //    }

        //    public Task HandleNotificationAsync(string methodName, JToken methodParam, Func<JToken, Task> sendNotification)
        //    {
        //        throw new NotImplementedException();
        //    }

        //    public async Task<JToken> HandleRequestAsync(string methodName, JToken methodParam, Func<JToken, Task<JToken>> sendRequest)
        //    {
        //        var result = await sendRequest(methodParam);
        //        return result;
        //    }
        //}
    }
}
