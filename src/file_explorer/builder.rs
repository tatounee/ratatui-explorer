use std::{io::Result, path::PathBuf};

use crate::{FileExplorer, Theme};

/// Builder for creating a [`FileExplorer`](FileExplorer).
///
/// By default, the builder create `FileExplorer` with a working directory set to the current one.
#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
pub struct FileExplorerBuilder {
    cwd: Option<PathBuf>,
    theme: Option<Theme>,
    show_hidden: bool,
}

impl FileExplorerBuilder {
    /// Set the current working directory for the `FileExplorer`.
    /// If not set, it defaults to the current directory.
    ///
    /// # Examples
    /// Suppose you have this tree file:
    /// ```plaintext
    /// /
    /// ├── .git
    /// └── Documents
    ///     ├── passport.png
    ///     └── resume.pdf
    /// ```
    /// You can create a new `FileExplorer` like this:
    /// ```no_run
    /// use ratatui_explorer::FileExplorerBuilder;
    ///
    /// let file_explorer = FileExplorerBuilder::default()
    ///     .working_dir("/Documents")
    ///     .build()
    ///     .unwrap();
    /// assert_eq!(file_explorer.cwd().display().to_string(), "/Documents");
    /// ```
    pub fn working_dir<P: Into<PathBuf>>(mut self, working_dir: P) -> Self {
        self.cwd = Some(working_dir.into());
        self
    }

    /// Set the theme for the `FileExplorer`.
    /// If not set, it defaults to [`Theme::new`](Theme::new).
    /// 
    /// # Examples
    ///
    /// ```no_run
    /// use ratatui_explorer::{FileExplorerBuilder, Theme};
    ///
    /// let file_explorer = FileExplorerBuilder::default()
    ///     .theme(Theme::default().add_default_title())
    ///     .build()
    ///     .unwrap();
    /// ```    
    pub fn theme(mut self, theme: Theme) -> Self {
        self.theme = Some(theme);
        self
    }

    /// Set whether to show hidden files in the `FileExplorer`. Defaults to `false`.
    pub fn show_hidden(mut self, show: bool) -> Self {
        self.show_hidden = show;
        self
    }

    /// Build the `FileExplorer` instance based on the provided configuration.
    ///
    /// # Errors
    ///
    /// Will return `Err` if the setted working directory can not be listed.
    /// 
    /// Will return `Err` if NO working directory have been setted and current working directory can not be listed.
    /// See [`current_dir`](https://doc.rust-lang.org/stable/std/env/fn.current_dir.html) for more information.
    ///
    #[allow(clippy::unwrap_or_default)]
    pub fn build(self) -> Result<FileExplorer> {
        let cwd = self.cwd.unwrap_or(std::env::current_dir()?);
        let show_hidden = self.show_hidden;
        let files = FileExplorer::get_files(&cwd, show_hidden)?;
        let theme = self.theme.unwrap_or_else(Theme::new);

        let file_explorer = FileExplorer {
            cwd,
            files,
            show_hidden,
            selected: 0,
            theme,
        };

        Ok(file_explorer)
    }

    /// Shortcut method to create a `FileExplorer` with a custom theme.
    /// See [`FileExplorerBuilder::theme`] for more information about the theme configuration.
    pub fn build_with_theme(theme: Theme) -> Result<FileExplorer> {
        Self::default().theme(theme).build()
    }

    /// Shortcut method to create a `FileExplorer` with a custom working directory.
    /// See [`FileExplorerBuilder::working_dir`] for more information about the working directory configuration.
    pub fn build_with_working_dir<P: Into<PathBuf>>(working_dir: P) -> Result<FileExplorer> {
        Self::default().working_dir(working_dir).build()
    }
}
