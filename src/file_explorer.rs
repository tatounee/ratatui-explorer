use std::{fs::FileType, io::Result, path::PathBuf};

use ratatui::widgets::WidgetRef;

use crate::{input::Input, widget::Renderer, Theme};

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
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FileExplorer {
    cwd: PathBuf,
    files: Vec<File>,
    show_hidden: bool,
    selected: usize,
    theme: Theme,
}

impl FileExplorer {
    /// Creates a new instance of `FileExplorer`.
    ///
    /// This method initializes a `FileExplorer` with the current working directory.
    /// By default, hidden files are not shown.
    ///
    /// # Errors
    ///
    /// Will return `Err` if the current working directory can not be listed. See [`current_dir`](https://doc.rust-lang.org/stable/std/env/fn.current_dir.html) for more information.
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

        let mut file_explorer = Self {
            cwd,
            files: vec![],
            show_hidden: false,
            selected: 0,
            theme: Theme::default(),
        };

        file_explorer.get_and_set_files()?;

        Ok(file_explorer)
    }

    /// Creates a new instance of `FileExplorer` with a specific theme.
    ///
    /// This method initializes a `FileExplorer` with the current working directory.
    ///
    /// # Errors
    ///
    /// Will return `Err` if the current working directory can not be listed.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use ratatui_explorer::{FileExplorer, Theme};
    ///
    /// let file_explorer = FileExplorer::with_theme(Theme::default().add_default_title()).unwrap();
    /// ```
    #[inline]
    pub fn with_theme(theme: Theme) -> Result<FileExplorer> {
        let mut file_explorer = Self::new()?;

        file_explorer.theme = theme;

        Ok(file_explorer)
    }

    /// Build a ratatui widget to render the file explorer. The widget can then
    /// be rendered with [`Frame::render_widget`](https://docs.rs/ratatui/latest/ratatui/terminal/struct.Frame.html#method.render_widget).
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use ratatui::{Terminal, backend::CrosstermBackend};
    /// use ratatui_explorer::FileExplorer;
    ///
    /// let mut file_explorer = FileExplorer::new().unwrap();
    ///
    /// let mut terminal = Terminal::new(CrosstermBackend::new(std::io::stdout())).unwrap();
    ///
    /// loop {
    ///     terminal.draw(|f| {
    ///         let widget = file_explorer.widget(); // Get the widget to render the file explorer
    ///         f.render_widget(&widget, f.area());
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
    /// and [termwiz](https://docs.rs/termwiz/latest/termwiz/input/enum.InputEvent.html) (`InputEvent` in this case).
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
    ///
    /// /* user select `password.png` */
    ///
    /// file_explorer.handle(Input::Down).unwrap();
    /// assert_eq!(file_explorer.current().name(), "resume.pdf");
    ///
    /// file_explorer.handle(Input::Up).unwrap();
    /// file_explorer.handle(Input::Up).unwrap();
    /// assert_eq!(file_explorer.current().name(), "Documents");
    ///
    /// file_explorer.handle(Input::Left).unwrap();
    /// assert_eq!(file_explorer.cwd().display().to_string(), "/");
    ///
    /// file_explorer.handle(Input::Right).unwrap();
    /// assert_eq!(file_explorer.cwd().display().to_string(), "/Documents");
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
                    self.cwd = parent.to_path_buf();
                    self.get_and_set_files()?;
                    self.selected = 0;
                }
            }
            Input::Right => {
                if self.files[self.selected].path.is_dir() {
                    self.cwd = self.files.swap_remove(self.selected).path;
                    self.get_and_set_files()?;
                    self.selected = 0;
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
        self.cwd = cwd.into();
        self.get_and_set_files()?;
        self.selected = 0;

        Ok(())
    }

    /// Sets whether hidden files should be shown in the file explorer.
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
    /// file_explorer.set_show_hidden(true).unwrap();
    /// assert_eq!(file_explorer.show_hidden(), true);
    /// ```
    #[inline]
    pub fn set_show_hidden(&mut self, show_hidden: bool) -> Result<()> {
        self.show_hidden = show_hidden;
        self.get_and_set_files()?;
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

    /// Sets the selected file or directory index inside the current [`Vec`](https://doc.rust-lang.org/stable/std/vec/struct.Vec.html) of files
    /// and directories if the file explorer.
    ///
    /// The file explorer add the parent directory at the beginning of the
    /// [`Vec`](https://doc.rust-lang.org/stable/std/vec/struct.Vec.html) of files, so setting the selected index to 0 will select the parent directory
    /// (if the current working directory not the root directory).
    ///
    /// # Panics
    ///
    /// Panics if `selected` is greater or equal to the number of files (plus the parent directory if it exist) in the current
    /// working directory.
    ///
    /// # Examples
    ///
    /// Suppose you have this tree file, with `passport.png` selected inside `file_explorer`:
    /// ```plaintext
    /// /
    /// ├── .git
    /// └── Documents
    ///     ├── passport.png  <- selected (index 2)
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
    /// // of the [`Vec`](https://doc.rust-lang.org/stable/std/vec/struct.Vec.html) of files, index 0 is indeed the parent directory.
    /// file_explorer.set_selected_idx(0);
    /// assert_eq!(file_explorer.current().path().display().to_string(), "/");
    ///
    /// file_explorer.set_selected_idx(1);
    /// assert_eq!(file_explorer.current().path().display().to_string(), "/Documents");
    ///
    /// #[test]
    /// #[should_panic]
    /// fn index_out_of_bound() {
    ///    let mut file_explorer = FileExplorer::new().unwrap();
    ///    file_explorer.set_selected_idx(4);
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

    /// Returns the a [`Vec`](https://doc.rust-lang.org/stable/std/vec/struct.Vec.html) of files and directories in the current working directory
    /// of the file explorer, plus the parent directory if it exist.
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
    /// assert_eq!(files.len(), 4); // 3 files/directory and the parent directory
    /// ```
    #[inline]
    #[must_use]
    pub const fn files(&self) -> &Vec<File> {
        &self.files
    }

    /// Returns the index of the selected file or directory in the current [`Vec`](https://doc.rust-lang.org/stable/std/vec/struct.Vec.html) of files
    /// and directories in the current working directory of the file explorer.
    ///
    /// # Examples
    ///
    /// Suppose you have this tree file, with `passport.png` selected inside `file_explorer`:
    /// ```plaintext
    /// /
    /// ├── .git
    /// └── Documents
    ///     ├── passport.png  <- selected (index 2)
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
    /// // of the [`Vec`](https://doc.rust-lang.org/stable/std/vec/struct.Vec.html) of files, the selected index will be 2.
    /// assert_eq!(selected_idx, 2);
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
    /// It add the parent directory at the beginning of the [`Vec`](https://doc.rust-lang.org/stable/std/vec/struct.Vec.html) of files if it exist.
    fn get_and_set_files(&mut self) -> Result<()> {
        let (mut dirs, mut none_dirs): (Vec<_>, Vec<_>) = std::fs::read_dir(&self.cwd)?
            .filter_map(|entry| {
                let entry = entry.ok()?;
                let path = entry.path();
                let metadata = path.metadata().ok();
                let file_type = metadata.as_ref().map(|f| f.file_type());
                let is_dir = file_type.is_some_and(|f| f.is_dir());

                let name = entry.file_name().to_string_lossy().into_owned();
                let name = if is_dir { format!("{}/", name) } else { name };

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
                if !self.show_hidden && file.is_hidden() {
                    None
                } else {
                    Some(file)
                }
            })
            .partition(File::is_dir);

        dirs.sort_unstable_by(|f1, f2| f1.name.cmp(&f2.name));
        none_dirs.sort_unstable_by(|f1, f2| f1.name.cmp(&f2.name));

        if let Some(parent) = self.cwd.parent() {
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

            self.files = files;
        } else {
            let mut files = Vec::with_capacity(dirs.len() + none_dirs.len());

            files.extend(dirs);
            files.extend(none_dirs);

            self.files = files;
        };

        Ok(())
    }
}

/// A file or directory in the file explorer.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct File {
    name: String,
    path: PathBuf,
    is_dir: bool,
    is_hidden: bool,
    file_type: Option<FileType>,
}

impl File {
    /// Returns the name of the file or directory.
    ///
    /// # Examples
    /// Suppose you have this tree file, with `passport.png` selected inside `file_explorer`:
    /// ```plaintext
    /// /
    /// ├── .git
    /// └── Documents
    ///     ├── passport.png  <- selected
    ///     └── resume.pdf
    /// ```
    /// You can get the name of the selected file like this:
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
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns the path of the file or directory.
    ///
    /// # Examples
    /// Suppose you have this tree file, with `passport.png` selected inside `file_explorer`:
    /// ```plaintext
    /// /
    /// ├── .git
    /// └── Documents
    ///     ├── passport.png  <- selected
    ///     └── resume.pdf
    /// ```
    /// You can get the path of the selected file like this:
    /// ```no_run
    /// use ratatui_explorer::FileExplorer;
    ///
    /// let file_explorer = FileExplorer::new().unwrap();
    ///
    /// /* user select `password.png` */
    ///
    /// let file = file_explorer.current();
    /// assert_eq!(file.path().display().to_string(), "/Documents/passport.png");
    /// ```
    #[inline]
    #[must_use]
    pub const fn path(&self) -> &PathBuf {
        &self.path
    }

    /// Returns `true` is the file is a directory.
    ///
    /// # Examples
    /// Suppose you have this tree file, with `passport.png` selected inside `file_explorer`:
    /// ```plaintext
    /// /
    /// ├── .git
    /// └── Documents
    ///     ├── passport.png  <- selected
    ///     └── resume.pdf
    /// ```
    /// You can know if the selected file is a directory like this:
    /// ```no_run
    /// use ratatui_explorer::FileExplorer;
    ///
    /// let file_explorer = FileExplorer::new().unwrap();
    ///
    /// /* user select `password.png` */
    ///
    /// let file = file_explorer.current();
    /// assert_eq!(file.is_dir(), false);
    ///
    /// /* user select `Documents` */
    ///
    /// let file = file_explorer.current();
    /// assert_eq!(file.is_dir(), true);
    /// ```
    #[inline]
    #[must_use]
    pub const fn is_dir(&self) -> bool {
        self.is_dir
    }

    /// Returns `true` is the file is a regular file.
    ///
    /// # Examples
    /// Suppose you have this tree file, with `passport.png` selected inside `file_explorer`:
    /// ```plaintext
    /// /
    /// ├── .git
    /// └── Documents
    ///     ├── passport.png  <- selected
    ///     └── resume.pdf
    /// ```
    /// You can know if the selected file is a directory like this:
    /// ```no_run
    /// use ratatui_explorer::FileExplorer;
    ///
    /// let file_explorer = FileExplorer::new().unwrap();
    ///
    /// /* user select `password.png` */
    ///
    /// let file = file_explorer.current();
    /// assert_eq!(file.is_file(), true);
    ///
    /// /* user select `Documents` */
    ///
    /// let file = file_explorer.current();
    /// assert_eq!(file.is_file(), false);
    /// ```
    #[inline]
    #[must_use]
    pub fn is_file(&self) -> bool {
        self.file_type.is_some_and(|f| f.is_file())
    }

    /// Returns `true` if the file or directory is hidden.
    ///
    /// # Examples
    /// Suppose you have this tree file, with `passport.png` selected inside `file_explorer`:
    /// ```plaintext
    /// /
    /// ├── .git
    /// └── Documents
    ///     ├── passport.png  <- selected
    ///     └── resume.pdf
    /// ```
    /// You can know if the selected file or directory is hidden like this:
    /// ```no_run
    /// use ratatui_explorer::FileExplorer;
    ///
    /// let file_explorer = FileExplorer::new().unwrap();
    ///
    /// /* user select `password.png` */
    ///
    /// let file = file_explorer.current();
    /// assert_eq!(file.is_hidden(), false);
    ///
    /// /* user select `.git` */
    ///
    /// let file = file_explorer.current();
    /// assert_eq!(file.is_hidden(), true);
    /// ```
    #[inline]
    #[must_use]
    pub fn is_hidden(&self) -> bool {
        self.is_hidden
    }

    /// Returns the `FileType` of the file, when available.
    ///
    /// # Examples
    /// Suppose you have this tree file, with `passport.png` selected inside `file_explorer`:
    /// ```plaintext
    /// /
    /// ├── .git
    /// └── Documents
    ///     ├── passport.png  <- selected
    ///     └── resume.pdf
    /// ```
    /// You can know if the selected file is a directory like this:
    /// ```no_run
    /// use std::os::unix::fs::FileTypeExt;
    ///
    /// use ratatui_explorer::FileExplorer;
    ///
    /// let file_explorer = FileExplorer::new().unwrap();
    ///
    /// /* user select `password.png` */
    ///
    /// let file = file_explorer.current();
    /// assert_eq!(file.file_type().unwrap().is_file(), true);
    /// assert_eq!(file.file_type().unwrap().is_socket(), false);
    ///
    /// /* user select `Documents` */
    ///
    /// let file = file_explorer.current();
    /// assert_eq!(file.file_type().unwrap().is_file(), false);
    /// assert_eq!(file.file_type().unwrap().is_socket(), false);
    /// ```
    #[inline]
    #[must_use]
    pub const fn file_type(&self) -> Option<FileType> {
        self.file_type
    }
}
