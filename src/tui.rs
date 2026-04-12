use std::{collections::HashSet, io, path::Path};

use ratatui::{
    DefaultTerminal, Frame,
    buffer::Buffer,
    crossterm::event::{self, Event, KeyCode, KeyEventKind, KeyModifiers},
    layout::{Constraint, Flex, Layout, Rect},
    style::{Style, Stylize},
    symbols::border,
    text::Line,
    widgets::{Block, Cell, Clear, Paragraph, Row, StatefulWidget, Table, TableState, Widget},
};
use tui_input::Input;
use tui_input::backend::crossterm::EventHandler;
use uuid::Uuid;

use crate::{Book, Library, LibrarySearch, Status};

const ROW_HEIGHT: u16 = 3;
const CHROME_HEIGHT: u16 = 3;

#[derive(Debug, Default)]
pub struct Tui {
    is_running: bool,
    library: Library,
    table_state: TableState,
    num_visible: usize,
    filtered: Vec<Uuid>,
    popup: Option<Popup>,
    input_mode: InputMode,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
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
    ClosePopup,
    HandleInput(Event),
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
        let num_visible = term_size.height.saturating_sub(CHROME_HEIGHT) / ROW_HEIGHT;

        Ok(Self {
            library: my_lib,
            table_state: TableState::default().with_selected(0),
            num_visible: num_visible.into(),
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

    // TODO: Refactor to call draw_popup and draw_cursor
    fn draw(&mut self, frame: &mut Frame) {
        frame.render_widget(&mut *self, frame.area());

        if self.popup.is_some() {
            let popup_area = Popup::popup_area(frame.area(), 60, 20);
            frame.render_widget(self.popup.as_ref(), popup_area);
        };

        if self.input_mode == InputMode::Editing {
            if let Some(popup) = &self.popup {
                let popup_area = Popup::popup_area(frame.area(), 60, 20);
                let x_offset = (popup.input.visual_cursor() + 7) as u16;
                frame.set_cursor_position((popup_area.x + x_offset, popup_area.y + 1));
            }
        }
    }

    fn handle_events(&mut self) -> io::Result<Option<Message>> {
        let event = event::read()?;
        match event {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                Ok(self.handle_key_event(event))
            }
            Event::Resize(_, height) => Ok(Some(Message::Resize(
                (height.saturating_sub(CHROME_HEIGHT) / ROW_HEIGHT).into(),
            ))),
            _ => Ok(None),
        }
    }

    fn handle_key_event(&mut self, event: Event) -> Option<Message> {
        if let Event::Key(key) = event {
            match self.input_mode {
                InputMode::Normal => match (key.modifiers, key.code) {
                    (KeyModifiers::CONTROL, KeyCode::Char('c')) => Some(Message::Quit),
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
                },
                InputMode::Editing => match &mut self.popup {
                    Some(popup) => match (key.modifiers, key.code) {
                        (_, KeyCode::Enter) => Some(Message::ApplyFilter(LibrarySearch {
                            tags: Some(vec![popup.input.value_and_reset()]),
                            ..Default::default()
                        })),
                        (_, KeyCode::Esc) => Some(Message::ClosePopup),
                        (KeyModifiers::CONTROL, KeyCode::Char('c')) => Some(Message::Quit),
                        _ => Some(Message::HandleInput(event.clone())),
                    },
                    None => None,
                },
            }
        } else {
            None
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
            Message::ApplyFilter(filter) => {
                self.apply_filter(filter);
                self.update(Message::ClosePopup);
            }
            Message::ClearFilters => self.clear_filters(),
            // TODO: Create Popup::new or other method to create filter popup instance?
            Message::ShowPopup => {
                self.popup = Some(Popup {
                    title: " Filter by... ".into(),
                    content: " Tag: ".into(),
                    ..Default::default()
                });
                self.input_mode = InputMode::Editing;
            }
            Message::ClosePopup => {
                self.popup = None;
                self.input_mode = InputMode::Normal;
            }
            Message::HandleInput(event) => {
                if let Some(popup) = &mut self.popup {
                    popup.input.handle_event(&event);
                }
            }
        }
    }

    fn move_cursor_up(&mut self) {
        if let Some(i) = self.table_state.selected() {
            self.table_state.select(Some(i.saturating_sub(1)))
        } else {
            self.table_state.select(Some(0))
        }
    }

    fn move_cursor_down(&mut self) {
        if let Some(i) = self.table_state.selected() {
            self.table_state.select(Some(i.saturating_add(1)))
        } else {
            self.table_state.select(Some(0))
        }
    }

    fn move_page_up(&mut self) {
        *self.table_state.offset_mut() = self.table_state.offset().saturating_sub(self.num_visible);

        if let Some(i) = self.table_state.selected() {
            self.table_state
                .select(Some(i.saturating_sub(self.num_visible)))
        } else {
            self.table_state.select(Some(0))
        }
    }

    fn move_page_down(&mut self) {
        let top_next_page = self.table_state.offset() + self.num_visible;
        let top_last_full_page = self.filtered.len().saturating_sub(self.num_visible);
        *self.table_state.offset_mut() = top_next_page.min(top_last_full_page);

        if let Some(i) = self.table_state.selected() {
            let one_page_down = i.saturating_add(self.num_visible);
            let final_item = self.filtered.len().saturating_sub(1);
            self.table_state.select(Some(one_page_down.min(final_item)))
        } else {
            self.table_state.select(Some(0))
        }
    }

    fn apply_filter(&mut self, filter: LibrarySearch) {
        self.filtered = self.library.search(&filter).map(|b| b.id).collect();
        self.table_state.select(Some(0));
    }

    fn clear_filters(&mut self) {
        self.filtered = self.library.all().map(|b| b.id).collect();
        self.table_state.select(Some(0));
    }
}

impl Widget for &mut Tui {
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
            "<Ctrl+c> ".blue().bold(),
        ]);
        let block = Block::bordered()
            .title(title.centered())
            .title_bottom(instructions.centered())
            .border_set(border::THICK);

        let filtered_set: HashSet<Uuid> = self.filtered.iter().copied().collect();
        let header = Row::new(vec![
            Cell::new("Title"),
            Cell::new("Author"),
            Cell::new("Status"),
            Cell::new("Tags"),
        ])
        .black()
        .on_white();
        let table = self
            .library
            .all()
            .filter(|b| filtered_set.contains(&b.id))
            .map(|b| book_to_row(b))
            .collect::<Table>()
            .widths(Constraint::from_percentages([40, 20, 10, 30]))
            .column_spacing(2)
            .flex(Flex::SpaceEvenly)
            .row_highlight_style(Style::new().green().on_dark_gray().bold())
            .header(header)
            .block(block);

        StatefulWidget::render(table, area, buf, &mut self.table_state);
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

impl Widget for &Popup {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        Clear.render(area, buf);
        let instructions = Line::from(vec![
            " Cancel ".into(),
            "<Esc> ".blue().bold(),
            " Apply ".into(),
            "<Enter> ".blue().bold(),
        ]);
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

fn book_to_row(book: &Book) -> Row<'_> {
    let vert_pad = '\n'.to_string().repeat(usize::from(ROW_HEIGHT - 1) / 2);
    vec![
        book.title.to_string(),
        book.author.surname.to_string(),
        format!("{:?}", book.status),
        book.tags
            .iter()
            .map(|t| t.to_string())
            .collect::<Vec<_>>()
            .join(", "),
    ]
    .into_iter()
    .map(|s| Cell::new(format!("{}{}", vert_pad, s)))
    .collect::<Row>()
    .height(ROW_HEIGHT)
}

#[cfg(test)]
mod tests {
    use super::*;
    use ratatui::crossterm::event::KeyEvent;

    #[test]
    fn handle_key_event_quits_on_ctrl_c() {
        let term_size = Rect::new(1, 2, 3, 4);
        let mut tui = Tui::new(term_size).unwrap();
        tui.handle_key_event(Event::Key(KeyEvent::new(
            KeyCode::Char('c'),
            KeyModifiers::CONTROL,
        )));

        assert!(!tui.is_running);
    }
}
