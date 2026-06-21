use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Flex, Layout, Rect},
    style::Stylize,
    text::Line,
    widgets::{Block, Clear, Paragraph, Widget},
};
use tui_input::Input;

#[derive(Debug)]
pub(super) enum Popup {
    Filter(FilterPopup),
}

impl Popup {
    pub(super) fn area(area: Rect, percent_x: u16, percent_y: u16) -> Rect {
        let vertical = Layout::vertical([Constraint::Percentage(percent_y)]).flex(Flex::Center);
        let horizontal = Layout::horizontal([Constraint::Percentage(percent_x)]).flex(Flex::Center);
        let [area] = vertical.areas(area);
        let [area] = horizontal.areas(area);
        area
    }

    pub(super) const fn is_editable(&self) -> bool {
        match self {
            Self::Filter(_) => true,
        }
    }
}

impl Widget for &Popup {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        match self {
            Popup::Filter(p) => p.render(area, buf),
        }
    }
}

#[derive(Debug, Default)]
pub(super) struct FilterPopup {
    pub(super) title: String,
    pub(super) content: String,
    pub(super) input: Input,
}

impl FilterPopup {
    pub(super) fn render(&self, area: Rect, buf: &mut Buffer) {
        Clear.render(area, buf);
        let instructions = Line::from(vec![
            " Cancel ".into(),
            "<Esc> ".blue().bold(),
            " Apply ".into(),
            "<Enter> ".blue().bold(),
        ]);
        let block = Block::bordered()
            .title(Line::from(self.title.clone()).centered())
            .title_bottom(instructions.centered());
        Paragraph::new(Line::from(vec![
            self.content.clone().into(),
            self.input.value().into(),
        ]))
        .block(block)
        .render(area, buf);
    }
}
