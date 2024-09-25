use crate::game::Kadeu;
use crate::Pin;
use core::fmt;
use ratatui::{
    backend::Backend,
    layout::{Columns, Constraint, Direction, Flex, Layout, Rect},
    prelude::CrosstermBackend,
    style::Styled,
    text,
    widgets::{Block, Paragraph, Widget, WidgetRef},
    Frame, Terminal,
};
use std::collections::VecDeque;

pub type SlideShow<T> = VecDeque<Box<T>>;

#[derive(Clone)]
pub struct CardSide {
    deck_title: Option<String>,
    front: String,
    back: String,
    revealed: bool,
}

impl CardSide {
    pub fn new(front: String, back: String) -> Self {
        Self {
            deck_title: None,
            front: front,
            back: back,
            revealed: false,
        }
    }

    pub fn with_title(mut self, title: &str) -> Self {
        self.deck_title = Some(title.to_string());
        self
    }

    pub fn reveal(&mut self) {
        self.revealed = true;
    }

    pub fn is_revealed(&self) -> bool {
        self.revealed
    }
}

impl Widget for CardSide {
    fn render(self, area: Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        let content = if self.is_revealed() {
            self.back
        } else {
            self.front
        };
        let mut text = Text::new(&content).bordered(&[]).centered();
        if let Some(title) = self.deck_title.as_ref() {
            text = text.with_border_title(title);
        }
        text.render(area, buf)
    }
}

impl Widget for Text {
    fn render(self, area: Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        let text = text::Text::from(self.text);
        let center_area = if self.centered {
            center(
                area,
                Constraint::Length(text.width() as u16),
                Constraint::Length(1),
            )
        } else {
            area
        };
        text.render_ref(center_area, buf);

        if self.bordered {
            let block = if let Some(title) = &self.border_title {
                Block::bordered().title(title.to_string())
            } else {
                Block::bordered()
            };

            block.render_ref(area, buf);
        }
    }
}

#[derive(Default)]
pub struct Text {
    text: String,
    centered: bool,
    bordered: bool,
    border_title: Option<String>,
    border_styles: Vec<String>,
}

impl Text {
    pub fn new(text: &str) -> Self {
        let mut this = Self::default();
        this.text = text.to_string();
        this
    }
    pub fn centered(mut self) -> Self {
        self.centered = true;
        self
    }

    pub fn with_border_title(mut self, title: &str) -> Self {
        self.border_title = Some(title.to_string());
        self
    }

    pub fn bordered(mut self, styles: &[String]) -> Self {
        self.bordered = true;
        self.border_styles = Vec::from(styles);
        self
    }

    pub fn render(&self, frame: &mut Frame) {
        let text = text::Text::from(self.text.to_string());
        let area = if self.centered {
            center(
                frame.area(),
                Constraint::Length(text.width() as u16),
                Constraint::Length(1),
            )
        } else {
            frame.area()
        };

        if self.bordered {
            let block = if let Some(title) = &self.border_title {
                Block::bordered().title(title.to_string())
            } else {
                Block::bordered()
            };

            frame.render_widget(block, frame.area());
        }

        frame.render_widget(text, area)
    }
}

fn center(area: Rect, horizontal: Constraint, vertical: Constraint) -> Rect {
    let [area] = Layout::horizontal([horizontal])
        .flex(Flex::Center)
        .areas(area);
    let [area] = Layout::vertical([vertical]).flex(Flex::Center).areas(area);
    area
}

use std::io::Stdout;

pub struct Container<T> {
    elements: Vec<Box<T>>,
}
impl<T> Default for Container<T> {
    fn default() -> Self {
        Self { elements: vec![] }
    }
}
impl<T> Container<T> {
    fn grid(direction: Direction, cols: u32) -> Layout {
        let columns: Vec<Constraint> = (0..cols).map(|_| Constraint::Ratio(1, cols)).collect();
        Layout::default().direction(direction).constraints(columns)
    }

    pub fn push(&mut self, widget: T) {
        self.elements.push(Box::new(widget))
    }
}

impl<T> WidgetRef for Container<T>
where
    T: WidgetRef,
{
    fn render_ref(&self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer) {
        let cols = self.elements.len() as u32;
        let layout = Self::grid(Direction::Horizontal, cols).split(area);
        for (i, child) in self.elements.iter().enumerate() {
            child.render_ref(layout[i], buf)
        }
    }
}
struct Ui<B>
where
    B: Backend,
{
    terminal: Terminal<B>,
}

impl<B> Ui<B>
where
    B: Backend,
{
    pub fn render_container(&mut self, container: impl WidgetRef) {
        self.terminal
            .draw(|frame| container.render_ref(frame.size(), frame.buffer_mut()));
    }
}
//todo this can be generic
impl TryFrom<CrosstermBackend<Stdout>> for Ui<CrosstermBackend<Stdout>> {
    type Error = std::io::Error;

    fn try_from(value: CrosstermBackend<Stdout>) -> std::io::Result<Self> {
        let terminal = Terminal::new(value)?;
        Ok(Self { terminal })
    }
}

impl<T, U> Widget for Pin<T>
where
    T: Kadeu<Front = U, Back = U>,
    U: fmt::Display,
{
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        let content = if self.1 {
            self.0.front()
        } else {
            self.0.back()
        }
        .to_string();
        let text = text::Text::from(content);
        let p = Paragraph::new(text);
        p.render(area, buf)
    }
}

#[cfg(test)]
mod tests {
    use std::io::stdout;

    use ratatui::prelude::CrosstermBackend;

    use super::Ui;

    #[test]
    fn make_ui_crossterm() {
        let ui = Ui::try_from(CrosstermBackend::new(stdout()));
    }
}