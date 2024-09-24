use clap::Args;
use tower_lsp::LspService;
use tower_lsp::Server;

#[derive(Args, Debug)]
pub(crate) struct LspOpt;

pub(crate) fn lsp(opt: LspOpt) -> std::io::Result<()> {
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();

    Ok(rt.block_on(async {
        let stdin = tokio::io::stdin();
        let stdout = tokio::io::stdout();

        let (service, socket) = LspService::new(ink_tool::lsp::LspBackend::new);
        Server::new(stdin, stdout, socket).serve(service).await;
    }))
}
