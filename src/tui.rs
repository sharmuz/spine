use std::{io, path::Path};

use ratatui::{
    DefaultTerminal, Frame,
    buffer::Buffer,
    crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind},
    layout::Rect,
    style::Stylize,
    symbols::border,
    text::Line,
    widgets::{Block, List, ListItem, Widget},
};

use crate::Library;

#[derive(Debug, Default)]
pub struct Tui {
    is_running: bool,
    library: Library,
    cursor: usize,
    scroll_offset: usize,
    num_visible: usize,
}

enum Message {
    Quit,
    Resize(usize),
    CursorUp,
    CursorDown,
    PageUp,
    PageDown,
}

impl Tui {
    #[must_use]
    pub fn new(term_size: Rect) -> anyhow::Result<Self> {
        let path = Path::new("spine.json");
        let my_lib = if path.exists() {
            Library::open(path)?
        } else {
            Library::new()
        };

        Ok(Self {
            library: my_lib,
            num_visible: term_size.height.saturating_sub(2).into(),
            ..Default::default()
        })
    }

    pub fn run(mut self, mut terminal: DefaultTerminal) -> io::Result<()> {
        self.is_running = true;
        while self.is_running {
            terminal.draw(|frame| self.draw(frame))?;
            if let Some(message) = self.handle_events()? {
                self.update(message);
            }
        }
        Ok(())
    }

    fn draw(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
    }

    fn handle_events(&mut self) -> io::Result<Option<Message>> {
        match event::read()? {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                Ok(self.handle_key_event(key_event))
            }
            Event::Resize(_, rows) => Ok(Some(Message::Resize((rows.saturating_sub(2)).into()))),
            _ => Ok(None),
        }
    }

    fn handle_key_event(&mut self, key: KeyEvent) -> Option<Message> {
        match (key.modifiers, key.code) {
            (_, KeyCode::Esc) => Some(Message::Quit),
            (_, KeyCode::Up) => Some(Message::CursorUp),
            (_, KeyCode::Down) => Some(Message::CursorDown),
            (_, KeyCode::PageUp) => Some(Message::PageUp),
            (_, KeyCode::PageDown) => Some(Message::PageDown),
            _ => None,
        }
    }

    fn update(&mut self, msg: Message) {
        match msg {
            Message::Quit => self.is_running = false,
            Message::Resize(rows) => self.num_visible = rows,
            Message::CursorUp => self.move_cursor_up(),
            Message::CursorDown => self.move_cursor_down(),
            Message::PageUp => self.move_page_up(),
            Message::PageDown => self.move_page_down(),
        }
    }

    fn move_cursor_up(&mut self) {
        let is_first_visible = self.cursor == self.scroll_offset;
        let is_first_overall = self.cursor == 0;
        if is_first_visible && !is_first_overall {
            self.scroll_offset -= 1;
        }
        self.cursor = self.cursor.saturating_sub(1);
    }

    fn move_cursor_down(&mut self) {
        let is_last_visible =
            self.cursor == (self.scroll_offset + self.num_visible).saturating_sub(1);
        let is_last_overall = self.cursor == self.library.all().len().saturating_sub(1);
        if is_last_visible && !is_last_overall {
            self.scroll_offset += 1;
        }
        self.cursor = (self.cursor + 1).min(self.library.all().len().saturating_sub(1));
    }

    fn move_page_up(&mut self) {
        self.scroll_offset = self.scroll_offset.saturating_sub(self.num_visible);

        self.cursor = self.cursor.saturating_sub(self.num_visible);
    }

    fn move_page_down(&mut self) {
        let top_next_page = self.scroll_offset + self.num_visible;
        let top_last_full_page = self.library.all().len().saturating_sub(self.num_visible);
        self.scroll_offset = top_next_page.min(top_last_full_page);

        let next_page_cursor = self.cursor + self.num_visible;
        self.cursor = next_page_cursor.min(self.library.all().len().saturating_sub(1));
    }
}

impl Widget for &Tui {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        let title = Line::from(" Spine - Your Books ".bold());
        let instructions = Line::from(vec![
            " Move up ".into(),
            "<Up>".blue().bold(),
            " Move down ".into(),
            "<Down>".blue().bold(),
            " Quit ".into(),
            "<Esc> ".blue().bold(),
        ]);
        let block = Block::bordered()
            .title(title.centered())
            .title_bottom(instructions.centered())
            .border_set(border::THICK);

        let books = self
            .library
            .all()
            .enumerate()
            .skip(self.scroll_offset)
            .take(usize::from(area.height))
            .map(|(i, b)| (i, ListItem::from(b.to_string())))
            .map(|(i, t)| if i == self.cursor { t.green() } else { t })
            .collect::<List>();

        books.block(block).render(area, buf);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn handle_key_event_quits_on_esc() {
        let term_size = Rect::new(1, 2, 3, 4);
        let mut tui = Tui::new(term_size).unwrap();
        tui.handle_key_event(KeyCode::Esc.into());

        assert!(!tui.is_running);
    }
}
