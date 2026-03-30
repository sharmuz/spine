fn main() -> anyhow::Result<()> {
    if std::env::args().any(|arg| arg == "--tui") {
        let mut terminal = ratatui::init();
        let term_size = terminal.get_frame().area();
        let tui_result = spine::tui::Tui::new(term_size)
            .expect("library should load")
            .run(terminal);
        ratatui::restore();
        Ok(tui_result?)
    } else {
        let cli_args = std::env::args();
        spine::cli::main(cli_args)
    }
}
