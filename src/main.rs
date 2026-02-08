fn main() -> anyhow::Result<()> {
    if std::env::args().any(|arg| arg == "--cli") {
        spine::cli::main()
    } else {
        let mut terminal = ratatui::init();
        let term_size = terminal.get_frame().area();
        let tui_result = spine::tui::Tui::new(term_size)
            .expect("library should load")
            .run(terminal);
        ratatui::restore();
        Ok(tui_result?)
    }
}
