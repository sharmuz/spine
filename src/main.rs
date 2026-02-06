fn main() -> anyhow::Result<()> {
    if std::env::args().any(|arg| arg == "--cli") {
        spine::cli::main()
    } else {
        let terminal = ratatui::init();
        let tui_result = spine::tui::Tui::new()
            .expect("library should load")
            .run(terminal);
        ratatui::restore();
        Ok(tui_result?)
    }
}
