use std::{fs::FileType, path::PathBuf};

/// A file or directory in the file explorer.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct File {
    /// The name of the file or directory.
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
    /// assert_eq!(file.name, "passport.png");
    /// ```
    pub name: String,

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
    /// assert_eq!(file.path.display().to_string(), "/Documents/passport.png");
    /// ```
    pub path: PathBuf,

    /// Is `true` is the file is a directory.
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
    /// assert_eq!(file.is_dir, false);
    ///
    /// /* user select `Documents` */
    ///
    /// let file = file_explorer.current();
    /// assert_eq!(file.is_dir, true);
    /// ```
    pub is_dir: bool,

    /// Is `true` if the file or directory is hidden.
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
    /// assert_eq!(file.is_hidden, false);
    ///
    /// /* user select `.git` */
    ///
    /// let file = file_explorer.current();
    /// assert_eq!(file.is_hidden, true);
    /// ```
    pub is_hidden: bool,

    /// The [`FileType`](https://doc.rust-lang.org/stable/std/fs/struct.FileType.html) of the file, when available.
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
    /// assert_eq!(file.file_type.unwrap().is_dir(), false);
    ///
    /// /* user select `Documents` */
    ///
    /// let file = file_explorer.current();
    /// assert_eq!(file.file_type.unwrap().is_dir(), true);
    /// ```
    pub file_type: Option<FileType>,
}

impl File {
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

    #[allow(missing_docs)]
    #[inline]
    #[must_use]
    #[deprecated(
        since = "0.3.0",
        note = "`name` field is public and should be acceded with `.name`"
    )]
    pub fn name(&self) -> &str {
        &self.name
    }

    #[allow(missing_docs)]
    #[inline]
    #[must_use]
    #[deprecated(
        since = "0.3.0",
        note = "`path` field is public and should be acceded with `.path`"
    )]
    pub const fn path(&self) -> &PathBuf {
        &self.path
    }

    #[allow(missing_docs)]
    #[inline]
    #[must_use]
    #[deprecated(
        since = "0.3.0",
        note = "`is_dir` field is public and should be acceded with `.is_dir`"
    )]
    pub const fn is_dir(&self) -> bool {
        self.is_dir
    }

    #[allow(missing_docs)]
    #[inline]
    #[must_use]
    #[deprecated(
        since = "0.3.0",
        note = "`is_hidden` field is public and should be acceded with `.is_hidden`"
    )]
    pub fn is_hidden(&self) -> bool {
        self.is_hidden
    }

    #[allow(missing_docs)]
    #[inline]
    #[must_use]
    #[deprecated(
        since = "0.3.0",
        note = "`file_type` field is public and should be acceded with `.file_type`"
    )]
    pub const fn file_type(&self) -> Option<FileType> {
        self.file_type
    }
}
