use std::{collections::HashSet, io, path::Path};

use ratatui::{
    DefaultTerminal, Frame,
    buffer::Buffer,
    crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind},
    layout::{Constraint, Flex, Layout, Rect},
    style::Stylize,
    symbols::border,
    text::Line,
    widgets::{Block, Clear, List, ListItem, Paragraph, Widget},
};
use tui_input::Input;
use tui_input::backend::crossterm::EventHandler;
use uuid::Uuid;

use crate::{Library, LibrarySearch, Status};

#[derive(Debug, Default)]
pub struct Tui {
    is_running: bool,
    library: Library,
    cursor: usize,
    scroll_offset: usize,
    num_visible: usize,
    filtered: Vec<Uuid>,
    popup: Option<Popup>,
    input_mode: InputMode,
}

#[derive(Debug, Default)]
enum InputMode {
    #[default]
    Normal,
    Editing,
}

enum Message {
    Quit,
    Resize(usize),
    CursorUp,
    CursorDown,
    PageUp,
    PageDown,
    SelectItem,
    ApplyFilter(LibrarySearch),
    ClearFilters,
    ShowPopup,
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
        let all_ids = my_lib.all().map(|b| b.id).collect();

        Ok(Self {
            library: my_lib,
            num_visible: term_size.height.saturating_sub(2).into(),
            filtered: all_ids,
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

        if self.popup.is_some() {
            let popup_area = Popup::popup_area(frame.area(), 60, 20);
            frame.render_widget(self.popup.as_ref(), popup_area);
        };
    }

    // TODO: Change to handle Normal vs. Editing mode ?
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
            (_, KeyCode::Enter) => Some(Message::SelectItem),
            (_, KeyCode::Char('w')) => Some(Message::ApplyFilter(LibrarySearch {
                status: Some(Status::Want),
                ..Default::default()
            })),
            (_, KeyCode::Char('r')) => Some(Message::ApplyFilter(LibrarySearch {
                status: Some(Status::Read),
                ..Default::default()
            })),
            (_, KeyCode::Char('g')) => Some(Message::ApplyFilter(LibrarySearch {
                status: Some(Status::Reading),
                ..Default::default()
            })),
            (_, KeyCode::Char('c')) => Some(Message::ClearFilters),
            (_, KeyCode::Char('f')) => Some(Message::ShowPopup),
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
            Message::SelectItem => {
                if self.popup.is_some() {
                    self.popup = None
                }
            }
            Message::ApplyFilter(filter) => self.apply_filter(filter),
            Message::ClearFilters => self.clear_filters(),
            // TODO: Create Popup::new or other method to create filter popup instance?
            Message::ShowPopup => {
                self.popup = Some(Popup {
                    title: " Filter by... ".into(),
                    content: " Tag: ".into(),
                    ..Default::default()
                })
            }
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
        let is_last_overall = self.cursor == self.filtered.len().saturating_sub(1);
        if is_last_visible && !is_last_overall {
            self.scroll_offset += 1;
        }
        self.cursor = (self.cursor + 1).min(self.filtered.len().saturating_sub(1));
    }

    fn move_page_up(&mut self) {
        self.scroll_offset = self.scroll_offset.saturating_sub(self.num_visible);

        self.cursor = self.cursor.saturating_sub(self.num_visible);
    }

    fn move_page_down(&mut self) {
        let top_next_page = self.scroll_offset + self.num_visible;
        let top_last_full_page = self.filtered.len().saturating_sub(self.num_visible);
        self.scroll_offset = top_next_page.min(top_last_full_page);

        let next_page_cursor = self.cursor + self.num_visible;
        self.cursor = next_page_cursor.min(self.filtered.len().saturating_sub(1));
    }

    fn apply_filter(&mut self, filter: LibrarySearch) {
        self.filtered = self.library.search(&filter).map(|b| b.id).collect();
        self.cursor = 0;
        self.scroll_offset = 0;
    }

    fn clear_filters(&mut self) {
        self.filtered = self.library.all().map(|b| b.id).collect();
        self.cursor = 0;
        self.scroll_offset = 0;
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
            "<Up/PgUp>".blue().bold(),
            " Move down ".into(),
            "<Down/PgDn>".blue().bold(),
            " Filters ".into(),
            "[W]ant/[R]ead/Readin[G] ".blue().bold(),
            " Quit ".into(),
            "<Esc> ".blue().bold(),
        ]);
        let block = Block::bordered()
            .title(title.centered())
            .title_bottom(instructions.centered())
            .border_set(border::THICK);

        let filtered_set: HashSet<Uuid> = self.filtered.iter().copied().collect();
        let books = self
            .library
            .all()
            .filter(|b| filtered_set.contains(&b.id))
            .enumerate()
            .skip(self.scroll_offset)
            .take(usize::from(area.height))
            .map(|(i, b)| (i, ListItem::from(b.to_string())))
            .map(|(i, t)| if i == self.cursor { t.green() } else { t })
            .collect::<List>();

        books.block(block).render(area, buf);
    }
}

#[derive(Debug, Default)]
struct Popup {
    title: String,
    content: String,
    input: Input,
}

impl Popup {
    fn popup_area(area: Rect, percent_x: u16, percent_y: u16) -> Rect {
        let vertical = Layout::vertical([Constraint::Percentage(percent_y)]).flex(Flex::Center);
        let horizontal = Layout::horizontal([Constraint::Percentage(percent_x)]).flex(Flex::Center);
        let [area] = vertical.areas(area);
        let [area] = horizontal.areas(area);
        area
    }
}

// TODO: Update for Editing mode based on self.input and cursor position
impl Widget for &Popup {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        Clear.render(area, buf);
        let instructions = Line::from(vec![" Apply ".into(), "<Enter> ".blue().bold()]);
        let block = Block::bordered()
            .title(Line::from(self.title.clone()).centered())
            .title_bottom(Line::from(instructions).centered());
        Paragraph::new(Line::from(vec![
            self.content.clone().into(),
            self.input.value().into(),
        ]))
        .block(block)
        .render(area, buf);
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
