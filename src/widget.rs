use std::sync::Arc;

use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span, Text},
    widgets::{Block, Borders, HighlightSpacing, List, ListState, WidgetRef},
};

use crate::{File, FileExplorer};

type LineFactory = Arc<dyn Fn(&FileExplorer) -> Line<'static>>;

// macro_rules! impl_theme {
//     (GETTER; $($name:ident, $typ:ty);* ; $(| $($t:tt)*)?) => {
//         impl Theme {
//             $(
//                 #[inline]
//                 pub const fn $name(&self) -> &$typ {
//                     &self.$name
//                 }
//             )*
//             $($($t)*)?
//         }
//     };
//     (SETTER; $($name:ident, $typ:ty);* ; $(| $($t:tt)*)?) => {
//         paste::paste! {
//             impl Theme {
//                 $(
//                     #[inline]
//                     #[must_use = "method moves the value of self and returns the modified value"]
//                     pub fn [<with_ $name>](mut self, $name: $typ) -> Self {
//                         self.$name = $name;
//                         self
//                     }
//                 )*
//                 $($($t)*)?
//             }
//         }
//     };
// }

pub struct Renderer<'a>(pub(crate) &'a FileExplorer);

impl WidgetRef for Renderer<'_> {
    fn render_ref(&self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        let mut state = ListState::default().with_selected(Some(self.0.selected_idx()));

        let highlight_style = if self.0.current().is_dir() {
            self.0.theme().highlight_dir_style
        } else {
            self.0.theme().highlight_item_style
        };

        let mut list = List::new(self.0.files().iter().map(|file| file.text(self.0.theme())))
            .style(self.0.theme().style)
            .highlight_spacing(self.0.theme().highlight_spacing.clone())
            .highlight_style(highlight_style);

        if let Some(symbol) = self.0.theme().highlight_symbol.as_deref() {
            list = list.highlight_symbol(symbol);
        }

        if let Some(block) = self.0.theme().block.as_ref() {
            let mut block = block.clone();

            for title_top in self.0.theme().title_top(self.0) {
                block = block.title_top(title_top)
            }
            for title_bottom in self.0.theme().title_bottom(self.0) {
                block = block.title_bottom(title_bottom)
            }

            list = list.block(block);
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
#[derivative(Debug, PartialEq, Eq, Hash)]
pub struct Theme {
    block: Option<Block<'static>>,
    #[derivative(Debug = "ignore", PartialEq = "ignore", Hash = "ignore")]
    title_top: Vec<LineFactory>,
    #[derivative(Debug = "ignore", PartialEq = "ignore", Hash = "ignore")]
    title_bottom: Vec<LineFactory>,
    style: Style,
    item_style: Style,
    dir_style: Style,
    highlight_spacing: HighlightSpacing,
    highlight_item_style: Style,
    highlight_dir_style: Style,
    highlight_symbol: Option<String>,
}

impl Theme {
    #[inline]
    #[must_use = "method moves the value of self and returns the modified value"]
    pub fn add_default_title(self) -> Self {
        self.with_title_top(|file_explorer: &FileExplorer| {
            Line::from(file_explorer.pwd().display().to_string())
        })
    }
    #[inline]
    #[must_use = "method moves the value of self and returns the modified value"]
    pub fn with_block(mut self, block: Block<'static>) -> Self {
        self.block = Some(block);
        self
    }
    #[inline]
    #[must_use = "method moves the value of self and returns the modified value"]
    pub fn with_style(mut self, style: Style) -> Self {
        self.style = style;
        self
    }
    #[inline]
    #[must_use = "method moves the value of self and returns the modified value"]
    pub fn with_item_style(mut self, item_style: Style) -> Self {
        self.item_style = item_style;
        self
    }
    #[inline]
    #[must_use = "method moves the value of self and returns the modified value"]
    pub fn with_dir_style(mut self, dir_style: Style) -> Self {
        self.dir_style = dir_style;
        self
    }

    #[inline]
    #[must_use = "method moves the value of self and returns the modified value"]
    pub fn with_highlight_item_style(mut self, highlight_item_style: Style) -> Self {
        self.highlight_item_style = highlight_item_style;
        self
    }
    #[inline]
    #[must_use = "method moves the value of self and returns the modified value"]
    pub fn with_highlight_dir_style(mut self, highlight_dir_style: Style) -> Self {
        self.highlight_dir_style = highlight_dir_style;
        self
    }
    #[inline]
    #[must_use = "method moves the value of self and returns the modified value"]
    pub fn with_highlight_symbol(mut self, highlight_symbol: Option<&str>) -> Self {
        self.highlight_symbol = highlight_symbol.map(|s| s.to_owned());
        self
    }
    #[inline]
    #[must_use = "method moves the value of self and returns the modified value"]
    pub fn with_highlight_spacing(mut self, highlight_spacing: HighlightSpacing) -> Self {
        self.highlight_spacing = highlight_spacing;
        self
    }
    #[inline]
    #[must_use = "method moves the value of self and returns the modified value"]
    pub fn with_title_top(
        mut self,
        title_top: impl Fn(&FileExplorer) -> Line<'static> + 'static,
    ) -> Self {
        self.title_top.push(Arc::new(title_top));
        self
    }
    #[inline]
    #[must_use = "method moves the value of self and returns the modified value"]
    pub fn with_title_bottom(
        mut self,
        title_bottom: impl Fn(&FileExplorer) -> Line<'static> + 'static,
    ) -> Self {
        self.title_bottom.push(Arc::new(title_bottom));
        self
    }
    #[inline]
    pub const fn block(&self) -> Option<&Block<'static>> {
        self.block.as_ref()
    }
    #[inline]
    pub const fn style(&self) -> &Style {
        &self.style
    }
    #[inline]
    pub const fn item_style(&self) -> &Style {
        &self.item_style
    }
    #[inline]
    pub const fn dir_style(&self) -> &Style {
        &self.dir_style
    }
    #[inline]
    pub const fn highlight_item_style(&self) -> &Style {
        &self.highlight_item_style
    }
    #[inline]
    pub const fn highlight_dir_style(&self) -> &Style {
        &self.highlight_dir_style
    }
    #[inline]
    pub fn highlight_symbol(&self) -> Option<&str> {
        self.highlight_symbol.as_deref()
    }
    #[inline]
    pub const fn highlight_spacing(&self) -> &HighlightSpacing {
        &self.highlight_spacing
    }
    #[inline]
    pub fn title_top(&self, file_explorer: &FileExplorer) -> Vec<Line> {
        self.title_top
            .iter()
            .map(|title_top| title_top(file_explorer))
            .collect()
    }
    #[inline]
    pub fn title_bottom(&self, file_explorer: &FileExplorer) -> Vec<Line> {
        self.title_bottom
            .iter()
            .map(|title_bottom| title_bottom(file_explorer))
            .collect()
    }
}

// impl_theme!(
//     SETTER;
//     style, Style;
//     item_style, Style;
//     dir_style, Style;
//     highlight_spacing, HighlightSpacing;
//     highlight_item_style, Style;
//     highlight_dir_style, Style; |
//     #[inline]
//     #[must_use = "method moves the value of self and returns the modified value"]
//     pub fn with_highlight_symbol(mut self, highlight_symbol: Option<&str>) -> Self {
//         self.highlight_symbol = highlight_symbol.map(|s| s.to_owned());
//         self
//     }
//     #[inline]
//     #[must_use = "method moves the value of self and returns the modified value"]
//     pub fn with_title_top(mut self, title_top: impl Fn(&FileExplorer) -> Line<'static> + 'static) -> Self {
//         self.title_top.push(Arc::new(title_top));
//         self
//     }
//     #[inline]
//     #[must_use = "method moves the value of self and returns the modified value"]
//     pub fn with_title_bottom(mut self, title_bottom: impl Fn(&FileExplorer) -> Line<'static> + 'static) -> Self {
//         self.title_bottom.push(Arc::new(title_bottom));
//         self
//     }
//     #[inline]
//     #[must_use = "method moves the value of self and returns the modified value"]
//     pub fn with_block(mut self, block: Block<'static>) -> Self {
//         self.block = Some(block);
//         self
//     }
//     #[inline]
//     #[must_use = "method moves the value of self and returns the modified value"]
//     pub fn add_default_title(self) -> Self {
//         self.with_title_top(|file_explorer: &FileExplorer| Line::from(file_explorer.pwd().display().to_string()))
//     }
// );
// impl_theme!(
//     GETTER;
//     style, Style;
//     item_style, Style;
//     dir_style, Style;
//     highlight_spacing, HighlightSpacing;
//     highlight_item_style, Style;
//     highlight_dir_style, Style; |
//     #[inline]
//     pub fn highlight_symbol(&self) -> Option<&str> {
//         self.highlight_symbol.as_deref()
//     }
//     #[inline]
//     pub fn title_top(&self, file_explorer: &FileExplorer) -> Vec<Line> {
//         self.title_top
//             .iter()
//             .map(|title_top| title_top(file_explorer))
//             .collect()
//     }
//     #[inline]
//     pub fn title_bottom(&self, file_explorer: &FileExplorer) -> Vec<Line> {
//         self.title_bottom
//             .iter()
//             .map(|title_bottom| title_bottom(file_explorer))
//             .collect()
//     }
// );

impl Default for Theme {
    fn default() -> Self {
        Self {
            block: Some(Block::default().borders(Borders::ALL)),
            title_top: Vec::new(),
            title_bottom: Vec::new(),
            style: Style::default(),
            item_style: Style::default().fg(Color::White),
            dir_style: Style::default().fg(Color::Blue),
            highlight_spacing: HighlightSpacing::Always,
            highlight_item_style: Style::default().add_modifier(Modifier::BOLD),
            highlight_dir_style: Style::default()
                .fg(Color::Blue)
                .add_modifier(Modifier::BOLD),
            highlight_symbol: None,
        }
    }
}
