use std::{fs::FileType, path::PathBuf};

/// A file or directory in the file explorer.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct File {
    pub(super) name: String,
    pub(super) path: PathBuf,
    pub(super) is_dir: bool,
    pub(super) is_hidden: bool,
    pub(super) file_type: Option<FileType>,
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
    /// let mut file_explorer = FileExplorer::new().unwrap();
    /// file_explorer.set_show_hidden(true);
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

    /// Returns the [`FileType`](https://doc.rust-lang.org/stable/std/fs/struct.FileType.html) of the file, when available.
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
    /// assert_eq!(file.file_type().unwrap().is_dir(), false);
    ///
    /// /* user select `Documents` */
    ///
    /// let file = file_explorer.current();
    /// assert_eq!(file.file_type().unwrap().is_dir(), true);
    /// ```
    #[inline]
    #[must_use]
    pub const fn file_type(&self) -> Option<FileType> {
        self.file_type
    }
}
