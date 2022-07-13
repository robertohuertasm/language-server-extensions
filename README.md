# Language Server Demo

This repository is just a test to play with `Language Servers` (LS) and understand how they could be shared between different IDEs.

The LS will detect log lines in a code file and add a `codelens` and a `hover` to them. 

This logic will be shared between VS and VSCode, but unfortunately, VS doesn't support `codelenses`... yet, but it does support `hover`.

## Visual Studio Extension particularities

Note that VS does not support markdown in `Hovers`.

Another issue I found while developing the VS extension is that native hovers will overrule the ones coming from our Language Servers. I still have to find documentation about why this is happening and how to avoid it. In VSCode, the hovers will be added to the ones provided by other providers.

## Project structure

This is a monorepo with 3 projects:

- **language-server**: Written in `Rust` to easily create binaries for different platforms and even webassembly for web extensions support.
- **vscode-log-extension**: VSCode extension written in `TypeScript`.
- **vs-log-extension**: Visual studio extension written in `C#`.
- **test-project**: Project with some `JavaScript` files to test the extension behavior.

## Capabilities

You can take a look at [vs_capabilities.json](./vs_capabilities.json) and [vscode_capabilities.json](./vscode_capabilities.json) if you want to learn which are the capabilities that VS2022 and VSCode support (2022-07-13).

## Presentation

Note that there's a [powerpoint file](./presentation.pptx) with a small presentation about `Language Extensions and language servers`.
