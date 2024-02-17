use std::rc::Rc;

use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Span, Text},
    widgets::{Block, BorderType, Borders, HighlightSpacing, List, ListState, WidgetRef},
};

use crate::{File, FileExplorer};

macro_rules! impl_theme {
    (GETTER; $($name:ident, $typ:ty);* ; $(| $($t:tt)*)?) => {
        impl Theme {
            $(
                #[inline]
                pub const fn $name(&self) -> &$typ {
                    &self.$name
                }
            )*

            $($($t)*)?
        }
    };

    (SETTER; $($name:ident, $typ:ty);* ; $(| $($t:tt)*)?) => {
        paste::paste! {
            impl Theme {
                $(
                    #[inline]
                    pub fn [<with_ $name>](mut self, $name: $typ) -> Self {
                        self.$name = $name;
                        self
                    }
                )*

                $($($t)*)?
            }
        }
    };
}

pub struct Renderer<'a>(pub(crate) &'a FileExplorer);

impl WidgetRef for Renderer<'_> {
    fn render_ref(&self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        let mut state = ListState::default().with_selected(Some(self.0.selected()));

        let highlight_style = if self.0.current().is_dir() {
            self.0.theme().highlight_dir_style
        } else {
            self.0.theme().highlight_style
        };

        let mut list = List::new(self.0.files().iter().map(|file| file.text(self.0.theme())))
            .block(
                Block::default()
                    .title((self.0.theme().title)(self.0))
                    .borders(self.0.theme().border)
                    .border_type(self.0.theme().border_type)
                    .style(self.0.theme().block_style),
            )
            .style(self.0.theme().style)
            .highlight_spacing(self.0.theme().highlight_spacing.clone())
            .highlight_style(highlight_style);

        if let Some(symbol) = self.0.theme().highlight_symbol.as_deref() {
            list = list.highlight_symbol(symbol);
        }

        ratatui::widgets::StatefulWidgetRef::render_ref(&list, area, buf, &mut state)
    }
}

impl File {
    fn text(&self, theme: &Theme) -> Text<'_> {
        let style = if self.is_dir() {
            *theme.dir_style()
        } else {
            *theme.item_style()
        };
        Span::styled(self.name().to_owned(), style).into()
    }
}

#[derive(Clone, derivative::Derivative)]
#[derivative(Debug)]
pub struct Theme {
    block_style: Style,
    border_type: BorderType,
    border: Borders,
    #[derivative(Debug = "ignore")]
    title: Rc<dyn Fn(&FileExplorer) -> String>,
    style: Style,
    item_style: Style,
    dir_style: Style,
    highlight_spacing: HighlightSpacing,
    highlight_style: Style,
    highlight_dir_style: Style,
    highlight_symbol: Option<String>,
}

impl_theme!(
    GETTER;
    block_style, Style;
    style, Style;
    item_style, Style;
    dir_style, Style;
    highlight_spacing, HighlightSpacing;
    highlight_style, Style;
    highlight_dir_style, Style; |

    #[inline]
    pub const fn border_type(&self) -> BorderType {
        self.border_type
    }
    #[inline]
    pub const fn border(&self) -> Borders {
        self.border
    }

    #[inline]
    pub fn title(&self, file_explorer: &FileExplorer) -> String {
        (self.title)(file_explorer)
    }

    #[inline]
    pub fn highlight_symbol(&self) -> Option<&str> {
        self.highlight_symbol.as_deref()
    }
);

impl_theme!(
    SETTER;
    block_style, Style;
    border_type, BorderType;
    border, Borders;
    title, Rc<dyn Fn(&FileExplorer) -> String>;
    style, Style;
    item_style, Style;
    dir_style, Style;
    highlight_spacing, HighlightSpacing;
    highlight_style, Style;
    highlight_dir_style, Style; |


    #[inline]
    pub fn with_highlight_symbol(mut self, highlight_symbol: Option<&str>) -> Self {
        self.highlight_symbol = highlight_symbol.map(|s| s.to_owned());
        self
    }
);

impl Default for Theme {
    fn default() -> Self {
        let title = Rc::new(|file_explorer: &FileExplorer| {
            format!("Explorer - {}", file_explorer.pwd().display())
        });

        Self {
            block_style: Style::default(),
            border_type: BorderType::Plain,
            border: Borders::ALL,
            title,
            style: Style::default(),
            item_style: Style::default().fg(Color::White),
            dir_style: Style::default().fg(Color::Blue),
            highlight_spacing: HighlightSpacing::Always,
            highlight_style: Style::default().add_modifier(Modifier::BOLD),
            highlight_dir_style: Style::default()
                .fg(Color::Blue)
                .add_modifier(Modifier::BOLD),
            highlight_symbol: None,
        }
    }
}
