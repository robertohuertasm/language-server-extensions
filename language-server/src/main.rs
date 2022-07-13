use lsp_server::{Connection, ExtractError, Message, Request, RequestId, Response};
use lsp_types::request::{CodeLensRequest, CodeLensResolve, HoverRequest};
use lsp_types::{
    CodeLens, CodeLensOptions, Hover, HoverContents, HoverProviderCapability, MarkupContent,
    MarkupKind, Range, TextDocumentIdentifier,
};
use lsp_types::{InitializeParams, ServerCapabilities};
use regex::Regex;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error + Sync + Send>> {
    // initialize env file support
    dotenv::dotenv().ok();
    // Note that  we must have our logging only write out to stderr.
    eprintln!("Starting LOG detector LSP server");
    // Create the transport. Includes the stdio (stdin and stdout) versions but this could
    // also be implemented to use sockets or HTTP.
    let (connection, io_threads) = Connection::stdio();
    // Run the server and wait for the two threads to end (typically by trigger LSP Exit event).
    let server_capabilities = serde_json::to_value(&ServerCapabilities {
        code_lens_provider: Some(CodeLensOptions {
            resolve_provider: Some(true),
        }),
        hover_provider: Some(HoverProviderCapability::Simple(true)),
        ..Default::default()
    })?;

    let initialization_params = connection.initialize(server_capabilities)?;

    main_loop(&connection, initialization_params)?;
    io_threads.join()?;
    // Shut down gracefully.
    eprintln!("shutting down server");
    Ok(())
}

fn main_loop(
    connection: &Connection,
    params: serde_json::Value,
) -> Result<(), Box<dyn Error + Sync + Send>> {
    eprintln!("Initialization params: {:?}", &params.to_string());
    let initialize_params: InitializeParams = serde_json::from_value(params)?;
    let is_md_supported = initialize_params
        .capabilities
        .text_document
        .and_then(|x| x.hover)
        .and_then(|x| x.content_format)
        .map_or(false, |x| x.contains(&MarkupKind::Markdown));
    eprintln!("@@## is_md_supported: {}", is_md_supported);

    let re = Regex::new(r"logger\.(warn|info|debug|error)\(")?;
    eprintln!("STARTING MAIN LOOP");
    for msg in &connection.receiver {
        eprintln!("GOT MSG: {:?}\n", msg);
        match msg {
            Message::Request(req) => {
                if connection.handle_shutdown(&req)? {
                    return Ok(());
                }
                let req = match cast::<CodeLensRequest>(req) {
                    Ok((id, params)) => {
                        eprintln!("GOT codeLens request #{}: {:?}\n", id, params);
                        let content = get_text_from_document(&params.text_document)?;

                        let mut lenses = vec![];
                        content.lines().enumerate().for_each(|(i, line)| {
                            if let Some(captures) = re.captures(line) {
                                let lens = CodeLens {
                                    range: Range {
                                        start: lsp_types::Position {
                                            line: i as u32,
                                            character: 0,
                                        },
                                        end: lsp_types::Position {
                                            line: i as u32,
                                            character: 0,
                                        },
                                    },
                                    command: None,
                                    data: serde_json::to_value(format!(
                                        "Log call: {}",
                                        &captures[1]
                                    ))
                                    .ok(),
                                };
                                lenses.push(lens);
                            }
                        });

                        eprintln!("LENSES {:?}\n\n ------ \n", &lenses);
                        let result: Option<Vec<CodeLens>> = Some(lenses);
                        let result = serde_json::to_value(&result).unwrap();
                        let resp = Response {
                            id,
                            result: Some(result),
                            error: None,
                        };
                        connection.sender.send(Message::Response(resp))?;
                        continue;
                    }
                    Err(err @ ExtractError::JsonError { .. }) => panic!("{:?}", err),
                    Err(ExtractError::MethodMismatch(req)) => req,
                };
                let req = match cast::<CodeLensResolve>(req) {
                    Ok((id, mut codelens)) => {
                        eprintln!("GOT codeLens resolve #{}: {:?}\n", id, codelens);

                        let title = codelens
                            .data
                            .clone()
                            .map_or(Ok("No data".to_string()), serde_json::from_value::<String>)?;

                        codelens.command = Some(lsp_types::Command {
                            title,
                            command: "log-server.log".to_string(),
                            arguments: None,
                        });

                        let result = serde_json::to_value(&codelens).unwrap();
                        let resp = Response {
                            id,
                            result: Some(result),
                            error: None,
                        };
                        connection.sender.send(Message::Response(resp))?;
                        continue;
                    }
                    Err(err @ ExtractError::JsonError { .. }) => panic!("{:?}", err),
                    Err(ExtractError::MethodMismatch(req)) => req,
                };
                match cast::<HoverRequest>(req) {
                    Ok((id, params)) => {
                        eprintln!("GOT hover request #{}: {:?}\n", id, params);
                        let content = get_text_from_document(
                            &params.text_document_position_params.text_document,
                        )?;
                        let position = params.text_document_position_params.position;

                        let text = content
                            .lines()
                            .nth(position.line as usize)
                            .and_then(|line| re.captures(line))
                            .map_or_else(
                                || {
                                    if is_md_supported {
                                        "**No log call found** :(".to_string()
                                    } else {
                                        "No log call found :(".to_string()
                                    }
                                },
                                |captures| {
                                    let msg = if is_md_supported {
                                        "## Log call:"
                                    } else {
                                        "Log call:"
                                    };
                                    format!("{} {} at position {:?}", msg, &captures[1], position)
                                },
                            );

                        let hover = Hover {
                            contents: HoverContents::Markup(MarkupContent {
                                kind: MarkupKind::Markdown,
                                value: text,
                            }),
                            range: None,
                        };

                        let result = serde_json::to_value(&hover).unwrap();
                        let resp = Response {
                            id,
                            result: Some(result),
                            error: None,
                        };
                        eprintln!("SENT hover: {:?}", resp);
                        connection.sender.send(Message::Response(resp))?;
                        continue;
                    }
                    Err(err @ ExtractError::JsonError { .. }) => panic!("{:?}", err),
                    Err(ExtractError::MethodMismatch(req)) => req,
                };
            }
            Message::Response(resp) => {
                eprintln!("GOT RESPONSE: {:?}", resp);
            }
            Message::Notification(not) => {
                eprintln!("GOT NOTIFICATION: {:?}", not);
            }
        }
    }
    Ok(())
}

fn get_text_from_document(
    text_document: &TextDocumentIdentifier,
) -> Result<String, Box<dyn Error + Sync + Send>> {
    let path = text_document
        .uri
        .to_file_path()
        .map_err(|()| "url is not a file")?;

    let content = std::fs::read_to_string(&path).map_err(|err| format!("{:?}", err))?;
    Ok(content)
}

fn cast<R>(req: Request) -> Result<(RequestId, R::Params), ExtractError<Request>>
where
    R: lsp_types::request::Request,
    R::Params: serde::de::DeserializeOwned,
{
    req.extract(R::METHOD)
}
