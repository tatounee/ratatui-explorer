use std::{io::Result, path::PathBuf, sync::Arc};

use super::{File, FileExplorer, Filter};
use crate::Theme;

/// Builder for creating a [`FileExplorer`](FileExplorer).
///
/// By default, the builder create `FileExplorer` with a working directory set to the current one.
#[derive(Clone, Default, educe::Educe)]
#[educe(Debug, PartialEq, Eq, Hash)]
pub struct FileExplorerBuilder {
    cwd: Option<PathBuf>,
    theme: Option<Theme>,
    show_hidden: bool,
    #[educe(Debug(ignore), PartialEq(ignore), Hash(ignore))]
    filter: Option<Arc<Filter>>,
    custom_selected: bool,
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
    /// # use ratatui_explorer::FileExplorerBuilder;
    /// let file_explorer = FileExplorerBuilder::default()
    ///     .working_dir("/Documents")
    ///     .build()
    ///     .unwrap();
    ///
    /// assert_eq!(file_explorer.cwd().display().to_string(), "/Documents");
    /// ```
    pub fn working_dir<P: Into<PathBuf>>(mut self, working_dir: P) -> Self {
        self.cwd = Some(working_dir.into());
        self
    }

    /// Same as [`working_dir`](FileExplorerBuilder::working_dir) but will pre-select the file in the working directory.
    ///
    /// This method set the working directory to the parent directory of the provided file and select the file in the file explorer.
    /// You can also select a directory (eg. select `/Documents` inside `/`).
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
    /// You can create a new `FileExplorer` selecting `passport.png` like this:
    /// ```no_run
    /// # use ratatui_explorer::FileExplorerBuilder;
    /// let file_explorer = FileExplorerBuilder::default()
    ///     .working_file("/Documents/passport.png")
    ///     .build()
    ///     .unwrap();
    ///
    /// assert_eq!(file_explorer.cwd().display().to_string(), "/Documents");
    /// assert_eq!(file_explorer.current().path.display().to_string(), "/Documents/passport.png");
    /// ```
    pub fn working_file<P: Into<PathBuf>>(mut self, working_file: P) -> Self {
        self.custom_selected = true;
        self.working_dir(working_file)
    }

    /// Set whether to show hidden files in the `FileExplorer`. Defaults to `false`.
    pub fn show_hidden(mut self, show: bool) -> Self {
        self.show_hidden = show;
        self
    }

    /// Set a filter and map for the `FileExplorer`.
    ///
    /// If not set, all files are shown. Hidden files are filtered **before** this
    /// filter will be apply.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use ratatui_explorer::FileExplorerBuilder;
    /// let file_explorer = FileExplorerBuilder::default()
    ///     .filter_map(|file| if file.is_dir { Some(file) } else { None })
    ///     .build()
    ///     .unwrap();
    ///
    /// /* Only directories are shown */
    /// ```
    pub fn filter_map(mut self, f: impl Fn(File) -> Option<File> + Send + Sync + 'static) -> Self {
        self.filter = Some(Arc::new(f));
        self
    }

    /// Set the theme for the `FileExplorer`.
    /// If not set, it defaults to [`Theme::new`](Theme::new).
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use ratatui_explorer::{FileExplorerBuilder, Theme};
    /// let file_explorer = FileExplorerBuilder::default()
    ///     .theme(Theme::default().add_default_title())
    ///     .build()
    ///     .unwrap();
    /// ```
    pub fn theme(mut self, theme: Theme) -> Self {
        self.theme = Some(theme);
        self
    }

    /// Build the `FileExplorer` instance based on the provided configuration.
    ///
    /// # Errors
    ///
    /// Will return `Err` if the setted working directory can not be listed.
    ///
    /// Will return `Err` if **NO** working directory have been setted and current working directory can not be listed.
    /// See [`current_dir`](https://doc.rust-lang.org/stable/std/env/fn.current_dir.html) for more information.
    ///
    #[allow(clippy::unwrap_or_default)]
    pub fn build(self) -> Result<FileExplorer> {
        let show_hidden = self.show_hidden;
        let theme = self.theme.unwrap_or_else(Theme::new);
        let filter = self.filter;

        let mut file_explorer = FileExplorer {
            cwd: PathBuf::new(),
            files: Vec::new(),
            show_hidden,
            selected: 0,
            theme,
            filter,
        };

        if self.custom_selected {
            file_explorer.set_working_file(self.cwd.unwrap())?;
        } else {
            file_explorer.set_cwd(self.cwd.clone().unwrap_or(std::env::current_dir()?))?;
        }

        Ok(file_explorer)
    }

    /// Shortcut method to create a `FileExplorer` with a custom theme.
    /// See [`theme`](FileExplorerBuilder::theme) for more information about the theme configuration.
    pub fn build_with_theme(theme: Theme) -> Result<FileExplorer> {
        Self::default().theme(theme).build()
    }

    /// Shortcut method to create a `FileExplorer` with a custom working directory.
    /// See [`working_dir`](FileExplorerBuilder::working_dir) for more information about the working directory configuration.
    pub fn build_with_working_dir<P: Into<PathBuf>>(working_dir: P) -> Result<FileExplorer> {
        Self::default().working_dir(working_dir).build()
    }

    /// Shortcut method to create a `FileExplorer` with a custom working directory and file.
    /// See [`working_file`](FileExplorerBuilder::working_file) for more information about the working directory configuration.
    pub fn build_with_working_file<P: Into<PathBuf>>(working_dir: P) -> Result<FileExplorer> {
        Self::default().working_file(working_dir).build()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::fs::{self, File};
    use tempfile::TempDir;

    /// Build this temporary file system:
    /// ```plaintext
    /// <unknow>
    /// └ root
    ///   ├── .git
    ///   └── Documents
    ///       ├── passport.png
    ///       └── resume.pdf
    /// ```
    fn build_tmp_file_system() -> Result<TempDir> {
        let root = TempDir::new()?;

        let git_path = root.path().join(".git");
        let documents_path = root.path().join("Documents");
        let passport_path = root.path().join("Documents/passport.png");
        let resume_path = root.path().join("Documents/resume.pdf");

        fs::create_dir(git_path)?;
        fs::create_dir(documents_path)?;
        File::create(passport_path)?;
        File::create(resume_path)?;

        Ok(root)
    }

    #[test]
    fn test_thread_safe() {
        fn is_sync<T: Sync>() {}

        fn is_send<T: Send>() {}

        is_send::<FileExplorerBuilder>();
        is_sync::<FileExplorerBuilder>();
    }

    #[test]
    fn test_working_file_correcty_set_selected_file() -> Result<()> {
        let root = build_tmp_file_system()?;
        let documents_path = root.path().join("Documents");
        let passport_path = documents_path.join("passport.png");

        let file_explorer = FileExplorerBuilder::default()
            .working_file(&passport_path)
            .build()
            .unwrap();

        assert_eq!(*file_explorer.cwd(), documents_path);
        assert_eq!(file_explorer.current().path, passport_path);

        Ok(())
    }

    #[test]
    fn test_working_file_correcty_set_selected_dir() -> Result<()> {
        let root = build_tmp_file_system()?;
        let documents_path = root.path().join("Documents");

        let file_explorer = FileExplorerBuilder::default()
            .show_hidden(true)
            .working_file(&documents_path)
            .build()
            .unwrap();

        assert_eq!(*file_explorer.cwd(), root.path());
        assert_eq!(*file_explorer.current().path, documents_path);

        Ok(())
    }
}
