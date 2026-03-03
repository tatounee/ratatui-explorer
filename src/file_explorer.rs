use std::{
    io::Result,
    path::{Path, PathBuf},
    sync::Arc,
};

use ratatui::widgets::WidgetRef;

use crate::{input::Input, widget::Renderer, Theme};

mod builder;
mod file;

pub use builder::FileExplorerBuilder;
pub use file::File;

type Predicate = dyn Fn(&File) -> bool + Send + Sync;

/// A file explorer that allows browsing and selecting files and directories.
///
/// The `FileExplorer` struct represents a file explorer widget that can be used to navigate
/// through the file system.
/// You can obtain a renderable widget from it with the [`widget`](#method.widget) method.
/// It provides methods for handling user input from [crossterm](https://crates.io/crates/crossterm),
/// [termion](https://crates.io/crates/termion) and [termwiz](https://crates.io/crates/termwiz) (depending on what feature is enabled).
///
/// # Examples
///
/// Creating a new `FileExplorer` widget:
///
/// ```no_run
/// use ratatui_explorer::FileExplorer;
///
/// let file_explorer = FileExplorer::new().unwrap();
/// let widget = file_explorer.widget();
/// ```
///
/// Handling user input:
///
/// ```no_run
/// # fn get_event() -> ratatui_explorer::Input {
/// #   unimplemented!()
/// # }
/// use ratatui_explorer::FileExplorer;
///
/// let mut file_explorer = FileExplorer::new().unwrap();
/// let event = get_event(); // Get the event from the terminal (with crossterm, termion or termwiz)
/// file_explorer.handle(event).unwrap();
/// ```
///
/// Accessing information about the current file selected and or the current working directory:
///
/// ```no_run
/// use ratatui_explorer::FileExplorer;
///
/// let file_explorer = FileExplorer::new().unwrap();
///
/// let current_file = file_explorer.current();
/// let current_working_directory = file_explorer.cwd();
/// println!("Current Directory: {}", current_working_directory.display());
/// println!("Name: {}", current_file.name());
/// ```
#[derive(Clone, educe::Educe)]
#[educe(Debug, PartialEq, Eq, Hash)]
pub struct FileExplorer {
    cwd: PathBuf,
    files: Vec<File>,
    show_hidden: bool,
    selected: usize,
    theme: Theme,
    #[educe(Debug(ignore), PartialEq(ignore), Hash(ignore))]
    filter: Option<Arc<Predicate>>,
}

impl FileExplorer {
    /// Creates a new instance of `FileExplorer`.
    ///
    /// This method initializes a `FileExplorer` with the current working directory.
    /// By default, hidden files are not shown.
    ///
    /// You can use the [`FileExplorerBuilder`](FileExplorerBuilder) to create a `FileExplorer` with a custom working
    /// directory, theme, and other options. See its documentation for more information.
    ///
    /// # Errors
    ///
    /// Will return `Err` if the current working directory can not be listed.
    /// See [`current_dir`](https://doc.rust-lang.org/stable/std/env/fn.current_dir.html) for more information.
    ///
    /// # Examples
    /// Suppose you have this tree file and your current working directory is `/Documents`:
    /// ```plaintext
    /// /
    /// ├── .git
    /// └── Documents  <- current working directory
    ///     ├── passport.png
    ///     └── resume.pdf
    /// ```
    /// You can create a new `FileExplorer` like this:
    /// ```no_run
    /// use ratatui_explorer::FileExplorer;
    ///
    /// let file_explorer = FileExplorer::new().unwrap();
    /// assert_eq!(file_explorer.cwd().display().to_string(), "/Documents");
    /// ```
    pub fn new() -> Result<FileExplorer> {
        let cwd = std::env::current_dir()?;
        let files = Self::get_files(&cwd, false)?;
        let file_explorer = Self {
            cwd,
            files,
            show_hidden: false,
            selected: 0,
            theme: Theme::default(),
            filter: None,
        };

        Ok(file_explorer)
    }

    /// Build a ratatui widget to render the file explorer. The widget can then
    /// be rendered with [`Frame::render_widget`](https://docs.rs/ratatui/latest/ratatui/struct.Frame.html#method.render_widget)
    /// or [`FrameExt::render_widget_ref`](https://docs.rs/ratatui/latest/ratatui/widgets/trait.FrameExt.html#tymethod.render_widget_ref).
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use ratatui::{Terminal, backend::CrosstermBackend, widgets::FrameExt as _};
    /// use ratatui_explorer::FileExplorer;
    ///
    /// let mut file_explorer = FileExplorer::new().unwrap();
    ///
    /// let mut terminal = Terminal::new(CrosstermBackend::new(std::io::stdout())).unwrap();
    ///
    /// loop {
    ///     terminal.draw(|f| {
    ///         let widget = file_explorer.widget(); // Get the widget to render the file explorer
    ///         f.render_widget_ref(widget, f.area());
    ///     }).unwrap();
    ///
    ///     // ...
    /// }
    /// ```
    #[inline]
    #[must_use]
    pub const fn widget(&self) -> impl WidgetRef + '_ {
        Renderer(self)
    }

    /// Handles input from user and updates the state of the file explorer.
    /// The different inputs are interpreted as follows:
    /// - `Up`: Move the selection up.
    /// - `Down`: Move the selection down.
    /// - `Left`: Move to the parent directory.
    /// - `Right`: Move to the selected directory.
    /// - `Home`: Select the first entry.
    /// - `End`: Select the last entry.
    /// - `PageUp`: Scroll the selection up.
    /// - `PageDown`: Scroll the selection down.
    /// - `ToggleShowHidden`: Toggle between showing hidden files or not.
    /// - `None`: Do nothing.
    ///
    /// [`Input`](crate::input::Input) implement [`From<Event>`](https://doc.rust-lang.org/stable/std/convert/trait.From.html)
    /// for `Event` from [crossterm](https://docs.rs/crossterm/latest/crossterm/event/enum.Event.html),
    /// [termion](https://docs.rs/termion/latest/termion/event/enum.Event.html)
    /// and [termwiz](https://docs.rs/termwiz/latest/termwiz/input/enum.InputEvent.html) (`InputEvent` in the latter).
    /// Here, the [default bindings](https://docs.rs/ratatui-explorer/latest/ratatui_explorer/#bindings).
    ///
    /// # Errors
    ///
    /// Will return `Err` if the new current working directory can not be listed.
    ///
    /// # Examples
    ///
    /// Suppose you have this tree file, with `passport.png` selected inside `file_explorer`:
    /// ```plaintext
    /// /
    /// ├── .git
    /// └── Documents
    ///     ├── passport.png  <- selected
    ///     └── resume.pdf
    /// ```
    /// You can handle input like this:
    /// ```no_run
    /// use ratatui_explorer::{FileExplorer, Input};
    ///
    /// let mut file_explorer = FileExplorer::new().unwrap();
    /// file_explorer.set_show_hidden(true);
    ///
    /// /* user select `password.png` */
    ///
    /// file_explorer.handle(Input::Down).unwrap();
    /// assert_eq!(file_explorer.current().name(), "resume.pdf");
    ///
    /// file_explorer.handle(Input::Up).unwrap();
    /// file_explorer.handle(Input::Up).unwrap();
    /// assert_eq!(file_explorer.current().name(), "../");
    ///
    /// file_explorer.handle(Input::Left).unwrap();
    /// assert_eq!(file_explorer.cwd().display().to_string(), "/");
    ///
    /// file_explorer.handle(Input::Right).unwrap();
    /// assert_eq!(file_explorer.cwd().display().to_string(), "/.git");
    /// ```
    pub fn handle<I: Into<Input>>(&mut self, input: I) -> Result<()> {
        const SCROLL_COUNT: usize = 12;

        let input = input.into();

        match input {
            Input::Up => {
                self.selected = self.selected.wrapping_sub(1).min(self.files.len() - 1);
            }
            Input::Down => {
                self.selected = (self.selected + 1) % self.files.len();
            }
            Input::Home => {
                self.selected = 0;
            }
            Input::End => {
                self.selected = self.files.len() - 1;
            }
            Input::PageUp => {
                self.selected = self.selected.saturating_sub(SCROLL_COUNT);
            }
            Input::PageDown => {
                self.selected = (self.selected + SCROLL_COUNT).min(self.files.len() - 1);
            }
            Input::Left => {
                let parent = self.cwd.parent();

                if let Some(parent) = parent {
                    let path = parent.to_path_buf();
                    self.set_cwd(path)?;
                }
            }
            Input::Right => {
                if self.files[self.selected].path.is_dir() {
                    let path = self.files.swap_remove(self.selected).path;
                    self.set_cwd(path)?;
                }
            }
            Input::ToggleShowHidden => self.set_show_hidden(!self.show_hidden)?,
            Input::None => (),
        }

        Ok(())
    }

    /// Sets the current working directory of the file explorer.
    ///
    /// # Errors
    ///
    /// Will return `Err` if the directory `cwd` can not be listed.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use ratatui_explorer::FileExplorer;
    ///
    /// let mut file_explorer = FileExplorer::new().unwrap();
    ///
    /// file_explorer.set_cwd("/Documents").unwrap();
    /// assert_eq!(file_explorer.cwd().display().to_string(), "/Documents");
    /// ```
    #[inline]
    pub fn set_cwd<P: Into<PathBuf>>(&mut self, cwd: P) -> Result<()> {
        let cwd = cwd.into();
        self.files = Self::get_files(&cwd, self.show_hidden)?;

        if let Some(filter) = &self.filter {
            self.files.retain(|f| filter(f));
        }

        self.cwd = cwd;
        self.selected = 0;

        Ok(())
    }

    /// Sets whether hidden files should be shown in the file explorer.
    ///
    /// # Errors
    ///
    /// Will return `Err` if the current working directory can not be listed.
    ///
    /// # Examples
    ///
    /// Suppose you have this tree file:
    /// ```plaintext
    /// /
    /// ├── .git
    /// └── Documents
    ///     ├── passport.png
    ///     └── resume.pdf
    /// ```
    /// ```no_run
    /// use ratatui_explorer::FileExplorerBuilder;
    ///
    /// let mut file_explorer = FileExplorerBuilder::build_with_working_dir("/").unwrap();
    /// assert_eq!(file_explorer.files().len(), 1); // Only /Documents is shown
    ///
    /// file_explorer.set_show_hidden(true).unwrap();
    /// assert_eq!(file_explorer.files().len(), 2); // /Documents and /.git are shown
    /// ```
    #[inline]
    pub fn set_show_hidden(&mut self, show_hidden: bool) -> Result<()> {
        self.show_hidden = show_hidden;
        self.files = Self::get_files(&self.cwd, show_hidden)?;
        self.selected = 0;

        Ok(())
    }

    /// Sets the theme of the file explorer.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use ratatui_explorer::{FileExplorer, Theme};
    ///
    /// let mut file_explorer = FileExplorer::new().unwrap();
    ///
    /// file_explorer.set_theme(Theme::default().add_default_title());
    /// ```
    #[inline]
    pub fn set_theme(&mut self, theme: Theme) {
        self.theme = theme;
    }

    /// Sets the selected file or directory index inside the current [`Vec`](https://doc.rust-lang.org/stable/std/vec/struct.Vec.html)
    /// of files and directories in the file explorer.
    ///
    /// The file explorer add the parent directory at the beginning of the
    /// [`Vec`](https://doc.rust-lang.org/stable/std/vec/struct.Vec.html) of files, so setting the selected index to 0
    /// will select the parent directory (if the current working directory not the root directory).
    ///
    /// # Panics
    ///
    /// Panics if `selected` is greater or equal to the number of files (plus the parent directory if it exist) in the
    /// current working directory.
    ///
    /// # Examples
    ///
    /// Suppose you have this tree file, with `passport.png` selected inside `file_explorer`:
    /// ```plaintext
    /// /
    /// ├── .git
    /// └── Documents
    ///     ├── passport.png  <- selected (index 1)
    ///     └── resume.pdf
    /// ```
    /// You can set the selected index like this:
    /// ```no_run
    /// use ratatui_explorer::FileExplorer;
    ///
    /// let mut file_explorer = FileExplorer::new().unwrap();
    ///
    /// /* user select `password.png` */
    ///
    /// // Because the file explorer add the parent directory at the beginning
    /// // of the `Vec` of files, index 0 is indeed the parent directory.
    /// file_explorer.set_selected_idx(0);
    /// assert_eq!(file_explorer.current().path().display().to_string(), "/");
    ///
    /// file_explorer.set_selected_idx(1);
    /// assert_eq!(file_explorer.current().path().display().to_string(), "/Documents/passport.png");
    ///
    /// #[test]
    /// #[should_panic]
    /// fn index_out_of_bound() {
    ///    let mut file_explorer = FileExplorer::new().unwrap();
    ///    file_explorer.set_selected_idx(3);
    /// }
    /// ```
    #[inline]
    pub fn set_selected_idx(&mut self, selected: usize) {
        assert!(selected < self.files.len());
        self.selected = selected;
    }

    /// Returns the current file or directory selected.
    ///
    /// # Examples
    ///
    /// Suppose you have this tree file, with `passport.png` selected inside `file_explorer`:
    /// ```plaintext
    /// /
    /// ├── .git
    /// └── Documents
    ///     ├── passport.png  <- selected
    ///     └── resume.pdf
    /// ```
    /// You can get the current file like this:
    /// ```no_run
    /// use ratatui_explorer::FileExplorer;
    ///
    /// let file_explorer = FileExplorer::new().unwrap();
    ///
    /// /* user select `password.png` */
    ///
    /// let file = file_explorer.current();
    /// assert_eq!(file.name(), "passport.png");
    /// ```
    #[inline]
    #[must_use]
    pub fn current(&self) -> &File {
        &self.files[self.selected]
    }

    /// Filters the files in the current working directory based on a predicate.
    ///
    /// This method mutates the file explorer by filtering the internal list of files using the provided predicate.
    ///
    /// # Examples:
    ///
    /// ```no_run
    /// use ratatui_explorer::FileExplorer;
    ///
    /// const SUPPORTED_FORMATS: [&'static str; 2] = ["wav", "mp3"];
    ///
    /// let mut file_explorer = FileExplorer::new().unwrap();
    /// file_explorer.set_filter(|f| {
    ///     match f.path().extension() {
    ///         Some(extension) => {
    ///             let extension = extension.to_str().unwrap_or_default();
    ///             SUPPORTED_FORMATS.contains(&extension)
    ///         }
    ///         None => f.is_dir()
    ///     }
    /// });
    /// ```
    /// To reset the filter, you can set the directory again:
    /// ```no_run
    /// # use ratatui_explorer::FileExplorer;
    /// # let mut file_explorer = FileExplorer::new().unwrap();
    /// let cwd = file_explorer.cwd().clone();
    /// file_explorer.set_cwd(cwd).unwrap();
    /// ```
    pub fn set_filter(&mut self, predicate: impl Fn(&File) -> bool + Send + Sync + 'static) {
        self.files.retain(|f| predicate(f));
        self.filter = Some(Arc::new(predicate));
        self.selected = 0;
    }

    /// Removes the current filter and returns it if it exist.
    ///
    /// # Errors
    ///
    /// Will return `Err` if the current working directory can not be listed.
    ///
    /// # Examples
    /// ```no_run
    /// use ratatui_explorer::FileExplorer;
    /// let mut file_explorer = FileExplorer::new().unwrap();
    /// file_explorer.set_filter(|f| f.is_dir());
    ///
    ///  /* Only directories are shown */
    ///
    /// let filter = file_explorer.remove_filter();
    ///
    /// /* All files and directories are shown again */
    /// ```
    pub fn remove_filter(&mut self) -> Result<Option<Arc<Predicate>>> {
        let filter = self.filter.take();

        self.files = Self::get_files(&self.cwd, self.show_hidden)?;
        self.selected = 0;

        Ok(filter)
    }

    /// Returns the current working directory of the file explorer.
    ///
    /// # Examples
    ///
    /// Suppose you have this tree file, with `passport.png` selected inside `file_explorer`:
    /// ```plaintext
    /// /
    /// ├── .git
    /// └── Documents
    ///     ├── passport.png  <- selected
    ///     └── resume.pdf
    /// ```
    /// You can get the current working directory like this:
    /// ```no_run
    /// use ratatui_explorer::FileExplorer;
    ///
    /// let file_explorer = FileExplorer::new().unwrap();
    ///
    /// /* user select `password.png` */
    ///
    /// let cwd = file_explorer.cwd();
    /// assert_eq!(cwd.display().to_string(), "/Documents");
    /// ```
    #[inline]
    #[must_use]
    pub const fn cwd(&self) -> &PathBuf {
        &self.cwd
    }

    /// Indicates whether hidden files are currently visible in the file explorer.
    /// # Examples
    ///
    ///
    /// You can get the current value like this:
    /// ```no_run
    /// use ratatui_explorer::FileExplorer;
    ///
    /// let mut file_explorer = FileExplorer::new().unwrap();
    ///
    /// // By default, hidden files are not shown.
    /// assert_eq!(file_explorer.show_hidden(), false);
    ///
    /// file_explorer.set_show_hidden(true);
    /// assert_eq!(file_explorer.show_hidden(), true);
    /// ```
    #[inline]
    #[must_use]
    pub const fn show_hidden(&self) -> bool {
        self.show_hidden
    }

    /// Returns the a [`Vec`](https://doc.rust-lang.org/stable/std/vec/struct.Vec.html) of files and directories in the
    /// current working directory of the file explorer, plus the parent directory if it exist.
    ///
    /// # Examples
    ///
    /// Suppose you have this tree file, with `passport.png` selected inside `file_explorer`:
    /// ```plaintext
    /// /
    /// ├── .git
    /// └── Documents
    ///     ├── passport.png  <- selected
    ///     └── resume.pdf
    /// ```
    /// You can get the [`Vec`](https://doc.rust-lang.org/stable/std/vec/struct.Vec.html) of files and directories like this:
    /// ```no_run
    /// use ratatui_explorer::FileExplorer;
    ///
    /// let file_explorer = FileExplorer::new().unwrap();
    ///
    /// /* user select `password.png` */
    ///
    /// let files = file_explorer.files();
    /// assert_eq!(files.len(), 3); // 2 files and 1 parent directory
    /// ```
    #[inline]
    #[must_use]
    pub const fn files(&self) -> &Vec<File> {
        &self.files
    }

    /// Returns the index of the selected file or directory in the current [`Vec`](https://doc.rust-lang.org/stable/std/vec/struct.Vec.html)
    /// of files and directories in the current working directory of the file explorer.
    ///
    /// # Examples
    ///
    /// Suppose you have this tree file, with `passport.png` selected inside `file_explorer`:
    /// ```plaintext
    /// /
    /// ├── .git
    /// └── Documents
    ///     ├── passport.png  <- selected (index 1)
    ///     └── resume.pdf
    /// ```
    /// You can get the selected index like this:
    /// ```no_run
    /// use ratatui_explorer::FileExplorer;
    ///
    /// let file_explorer = FileExplorer::new().unwrap();
    ///
    /// /* user select `password.png` */
    ///
    /// let selected_idx = file_explorer.selected_idx();
    ///
    /// // Because the file explorer add the parent directory at the beginning
    /// // of the `Vec` of files, the selected index will be 1.
    /// assert_eq!(selected_idx, 1);
    /// ```
    #[inline]
    #[must_use]
    pub const fn selected_idx(&self) -> usize {
        self.selected
    }

    /// Returns the theme of the file explorer.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use ratatui_explorer::{FileExplorer, Theme};
    ///
    /// let file_explorer = FileExplorer::new().unwrap();
    ///
    /// assert_eq!(file_explorer.theme(), &Theme::default());
    /// ```
    #[inline]
    #[must_use]
    pub const fn theme(&self) -> &Theme {
        &self.theme
    }

    /// Get the files and directories in the current working directory and set them in the file explorer.
    /// It add the parent directory at the beginning of the [`Vec`](https://doc.rust-lang.org/stable/std/vec/struct.Vec.html) 
    /// of files if it exist.
    fn get_files(working_dir: &Path, show_hidden: bool) -> Result<Vec<File>> {
        let (mut dirs, mut none_dirs): (Vec<_>, Vec<_>) = std::fs::read_dir(working_dir)?
            .filter_map(|entry| {
                let entry = entry.ok()?;
                let path = entry.path();
                let metadata = path.metadata().ok();
                let file_type = metadata.as_ref().map(|f| f.file_type());
                let is_dir = file_type.is_some_and(|f| f.is_dir());

                let name = entry.file_name().to_string_lossy().into_owned();
                let name = if is_dir { format!("{name}/") } else { name };

                let is_hidden = {
                    #[cfg(unix)]
                    {
                        name.starts_with('.')
                    }

                    #[cfg(windows)]
                    {
                        use std::os::windows::fs::MetadataExt;
                        const FILE_ATTRIBUTE_HIDDEN: u32 = 0x2;
                        metadata.is_some_and(|f| f.file_attributes() & FILE_ATTRIBUTE_HIDDEN != 0)
                    }
                };

                let file = File {
                    name,
                    path,
                    is_dir,
                    is_hidden,
                    file_type,
                };
                if !show_hidden && file.is_hidden() {
                    None
                } else {
                    Some(file)
                }
            })
            .partition(File::is_dir);

        dirs.sort_unstable_by(|f1, f2| f1.name.cmp(&f2.name));
        none_dirs.sort_unstable_by(|f1, f2| f1.name.cmp(&f2.name));

        let files = if let Some(parent) = working_dir.parent() {
            let mut files = Vec::with_capacity(1 + dirs.len() + none_dirs.len());

            files.push(File {
                name: "../".to_owned(),
                path: parent.to_path_buf(),
                is_dir: true,
                is_hidden: false,
                file_type: None,
            });

            files.extend(dirs);
            files.extend(none_dirs);

            files
        } else {
            let mut files = Vec::with_capacity(dirs.len() + none_dirs.len());

            files.extend(dirs);
            files.extend(none_dirs);

            files
        };

        Ok(files)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::fs::{self, File};
    use tempdir::TempDir;

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
        let root = TempDir::new("root")?;

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

        is_send::<FileExplorer>();
        is_sync::<FileExplorer>();
    }

    #[test]
    fn test_set_cwd_does_not_change_displayed_path_on_failure() -> Result<()> {
        let tmp_dir = TempDir::new("cwd_does_not_change_on_failure")?;
        let does_not_exist_path = tmp_dir.path().join("does_not_exist");
        assert!(!does_not_exist_path.exists());

        let mut explorer = FileExplorer::new()?;
        let previous_cwd = explorer.cwd().clone();

        let result = explorer.set_cwd(does_not_exist_path);
        assert!(result.is_err());
        assert_eq!(&previous_cwd, explorer.cwd());

        Ok(())
    }

    #[cfg(unix)]
    #[test]
    fn test_hidden_files_are_ignored() -> Result<()> {
        let root = build_tmp_file_system()?;

        let mut explorer = FileExplorerBuilder::build_with_working_dir(root.path())?;
        assert_eq!(explorer.files().len(), 2);

        explorer.set_show_hidden(true)?;
        assert_eq!(explorer.files().len(), 3);

        Ok(())
    }

    #[test]
    fn test_appling_filter_hide_files() -> Result<()> {
        let root = build_tmp_file_system()?;
        let documents_path = root.path().join("Documents");

        let mut explorer = FileExplorerBuilder::build_with_working_dir(documents_path)?;
        assert_eq!(explorer.files().len(), 3);

        explorer.set_filter(|file| file.is_dir);
        assert_eq!(explorer.files().len(), 1);

        Ok(())
    }

    #[test]
    fn test_removing_filter_show_files() -> Result<()> {
        let root = build_tmp_file_system()?;
        let documents_path = root.path().join("Documents");

        let mut explorer = FileExplorerBuilder::build_with_working_dir(documents_path)?;
        assert_eq!(explorer.files().len(), 3);

        explorer.set_filter(|file| file.is_dir);
        assert_eq!(explorer.files().len(), 1);

        explorer.remove_filter()?;
        assert_eq!(explorer.files().len(), 3);

        Ok(())
    }

    #[test]
    fn test_filter_is_apply_when_changing_working_dir() -> Result<()> {
        let root = build_tmp_file_system()?;
        let documents_path = root.path().join("Documents");

        let mut explorer = FileExplorerBuilder::build_with_working_dir(documents_path)?;
        explorer.set_filter(|file| !file.name().ends_with("png"));
        assert_eq!(explorer.files().len(), 2);

        // Exit and re-entre Documents/
        explorer.handle(Input::Left)?;
        explorer.handle(Input::Down)?;
        explorer.handle(Input::Right)?;

        assert_eq!(explorer.files().len(), 2);

        Ok(())
    }
}
