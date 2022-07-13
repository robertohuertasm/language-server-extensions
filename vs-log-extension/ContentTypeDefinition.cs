
using Microsoft.VisualStudio.LanguageServer.Client;
using Microsoft.VisualStudio.Utilities;
using System.ComponentModel.Composition;

namespace vs_log_extension
{
#pragma warning disable 649
    public class JSZContentDefinition
    {
        [Export]
        [Name("jsz")]
        [BaseDefinition(CodeRemoteContentDefinition.CodeRemoteContentTypeName)]
        internal static ContentTypeDefinition ContentTypeDefinition;


        [Export]
        [FileExtension(".jsz")]
        [ContentType("jsz")]
        internal static FileExtensionToContentTypeDefinition FileExtensionDefinition;
    }
#pragma warning restore 649
}
