use strum::{Display, EnumIter, FromRepr, IntoEnumIterator};

use ratatui::{
    prelude::*,
    style::{palette::tailwind, Style},
    widgets::{block::*, *},
};
use throbber_widgets_tui::{Throbber, ThrobberState};

use crate::dependency_builder::CrateToAdd;

#[derive(Default, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum ItemListStatus {
    Selected,
    #[default]
    Unselected,
}

#[derive(Debug, Default, Clone)]
pub struct Popup {
    pub message: String,
}

impl StatefulWidget for Popup {
    type State = ThrobberState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        Block::bordered().title("").render(area, buf);

        let inner_area = area.inner(&Margin {
            vertical: 1,
            horizontal: 1,
        });

        let message = if self.message.is_empty() {
            "Adding dependencies, this may take a while"
        } else {
            self.message.as_ref()
        };

        let loader = Throbber::default()
            .label(message)
            .throbber_set(throbber_widgets_tui::BRAILLE_SIX)
            .use_type(throbber_widgets_tui::WhichUse::Spin);

        StatefulWidget::render(loader, inner_area, buf, state)
    }
}

#[derive(Default, Clone)]
pub struct CrateItemList {
    pub name: String,
    pub description: String,
    pub features: Option<Vec<String>>,
    pub status: ItemListStatus,
}

#[derive(Clone, Default)]
pub struct DependenciesListWidget {
    pub dependencies: Vec<CrateToAdd>,
}

impl DependenciesListWidget {
    pub fn new(dependencies: Vec<CrateToAdd>) -> Self {
        Self { dependencies }
    }
}

impl StatefulWidget for DependenciesListWidget {
    type State = ListState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        let items: Vec<Line<'_>> = self
            .dependencies
            .iter()
            .cloned()
            .map(|dep| Line::from(vec![dep.crate_name.into(), " ✓ ".blue()]))
            .collect();

        let list = List::new(items)
            .block(
                Block::bordered()
                    .padding(Padding::uniform(2))
                    .title("Dependencies to add"),
            )
            .highlight_style(Style::default().blue())
            .highlight_symbol("* ")
            .direction(ListDirection::TopToBottom);

        StatefulWidget::render(list, area, buf, state);
    }
}

impl CrateItemList {
    pub fn new(
        name: String,
        description: String,
        status: ItemListStatus,
        features: Option<Vec<String>>,
    ) -> Self {
        Self {
            name,
            description,
            status,
            features,
        }
    }
}

#[derive(Default, Clone)]
pub struct CratesListWidget {
    pub crates: Vec<CrateItemList>,
}

impl Into<ListItem<'_>> for CrateItemList {
    fn into(self) -> ListItem<'static> {
        let (is_selected, bg_color) = match self.status {
            ItemListStatus::Selected => ("✓", tailwind::BLUE.c300),
            ItemListStatus::Unselected => ("☐", Color::default()),
        };

        let line = Line::from(vec![
            self.name.bold().blue(),
            " ".into(),
            self.description.into(),
            " ".into(),
            is_selected.into(),
        ]);

        ListItem::new(line).style(Style::default().bg(bg_color))
    }
}

impl StatefulWidget for CratesListWidget {
    type State = ListState;
    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        let block = Block::default().padding(Padding::uniform(1));

        let list = List::new(self.crates)
            .block(block)
            .highlight_style(Style::default())
            .highlight_symbol(">> ")
            .highlight_spacing(HighlightSpacing::Always)
            .direction(ListDirection::TopToBottom);

        StatefulWidget::render(list, area, buf, state);
    }
}

impl CratesListWidget {
    pub fn new(crates: &Vec<CrateItemList>) -> Self {
        Self {
            crates: crates.to_vec(),
        }
    }
}

#[derive(Default, Clone, Copy, Display, FromRepr, EnumIter)]
pub enum CategoriesTabs {
    #[strum(to_string = "General")]
    #[default]
    General,

    #[strum(to_string = "Common")]
    Common,
    #[strum(to_string = "Math-scientific")]
    Math,

    #[strum(to_string = "FFI")]
    FFI,

    #[strum(to_string = "Cryptography")]
    Cryptography,

    #[strum(to_string = "Concurrency")]
    Concurrency,

    #[strum(to_string = "Networking")]
    Networking,

    #[strum(to_string = "Databases")]
    Databases,

    #[strum(to_string = "Cli-tools")]
    Clis,

    #[strum(to_string = "Graphics")]
    Graphics,
}

impl CategoriesTabs {
    pub fn next(self) -> Self {
        let current_index = self as usize;
        let previous_index = current_index.saturating_add(1);
        Self::from_repr(previous_index).unwrap_or(Self::from_repr(0).unwrap())
    }

    pub fn previous(self) -> Self {
        let current_index = self as usize;
        let previous_index = current_index.saturating_sub(1);
        Self::from_repr(previous_index).unwrap_or(self)
    }
}

impl StatefulWidget for CategoriesTabs {
    type State = ListState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        let categories: Vec<String> = CategoriesTabs::iter()
            .map(|category| format!("{category}"))
            .collect();

        let list = List::new(categories)
            .style(Style::default().white())
            .highlight_style(Style::default().blue())
            .highlight_symbol(">> ")
            .highlight_style(Style::default().yellow());

        StatefulWidget::render(list, area, buf, state);
    }
}

#[derive(Debug)]
pub struct FooterInstructions<'a> {
    instructions: Vec<Span<'a>>,
}

impl<'a> FooterInstructions<'a> {
    pub fn new(instructions: Vec<Span<'a>>) -> Self {
        FooterInstructions { instructions }
    }
}

impl<'a> Widget for FooterInstructions<'a> {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        let instructions = Title::from(Line::from(self.instructions));

        let info = Title::from(Line::from(vec!["V0.1.3".into()]))
            .position(Position::Top)
            .alignment(Alignment::Right);

        let block = Block::bordered()
            .title(
                instructions
                    .alignment(Alignment::Center)
                    .position(Position::Bottom),
            )
            .title(info);

        block.render(area, buf);
    }
}
