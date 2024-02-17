use std::rc::Rc;

use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Rect},
    style::{Color, Modifier, Style},
    symbols::border::Set,
    text::{Span, Text},
    widgets::{Block, BorderType, Borders, HighlightSpacing, List, ListState, Padding, WidgetRef},
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
                    .title_top((self.0.theme().title_top)(self.0))
                    .title_bottom((self.0.theme().title_bottom)(self.0))
                    .title_alignment(self.0.theme().title_alignement)
                    .title_style(self.0.theme().title_style)
                    .padding(self.0.theme().padding)
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
    border_set: Option<Set>,
    border: Borders,
    #[derivative(Debug = "ignore")]
    title_top: Rc<dyn Fn(&FileExplorer) -> String>,
    #[derivative(Debug = "ignore")]
    title_bottom: Rc<dyn Fn(&FileExplorer) -> String>,
    title_style: Style,
    title_alignement: Alignment,
    padding: Padding,
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
    border_set, Option<Set>;
    title_style, Style;
    title_alignement, Alignment;
    padding, Padding;
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
    pub fn title_top(&self, file_explorer: &FileExplorer) -> String {
        (self.title_top)(file_explorer)
    }

    #[inline]
    pub fn title_bottom(&self, file_explorer: &FileExplorer) -> String {
        (self.title_bottom)(file_explorer)
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
    border_set, Option<Set>;
    border, Borders;
    title_style, Style;
    title_alignement, Alignment;
    padding, Padding;
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

    #[inline]
    pub fn with_title_top(mut self, title_top: impl Fn(&FileExplorer) -> String + 'static) -> Self {
        self.title_top = Rc::new(title_top);
        self
    }

    #[inline]
    pub fn with_title_bottom(mut self, title_bottom: impl Fn(&FileExplorer) -> String + 'static) -> Self {
        self.title_bottom = Rc::new(title_bottom);
        self
    }
);

impl Default for Theme {
    fn default() -> Self {
        let title_top =
            Rc::new(|file_explorer: &FileExplorer| file_explorer.pwd().display().to_string());

        let title_bottom = Rc::new(|_: &FileExplorer| String::new());

        Self {
            block_style: Style::default(),
            border_type: BorderType::Plain,
            border: Borders::ALL,
            border_set: None,
            title_top,
            title_bottom,
            title_style: Style::default(),
            title_alignement: Alignment::Left,
            padding: Padding::default(),
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
