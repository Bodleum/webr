use std::path::{Path, PathBuf};

use crate::prelude::*;

pub trait PathBufExt {
    fn file_root(&self) -> Option<&str>;
}

impl PathBufExt for PathBuf {
    fn file_root(&self) -> Option<&str> {
        let fname = self.file_name().and_then(std::ffi::OsStr::to_str);
        fname
            .and_then(|s| s.split_once('.'))
            .map(|(before, _after)| before)
            .or(fname)
    }
}

pub trait PathExt {
    fn is_hidden(&self) -> Result<bool>;
}

impl PathExt for Path {
    fn is_hidden(&self) -> Result<bool> {
        Ok(self
            .file_name()
            .ok_or(Error::Generic(format!("Invalid path {self:?}")))?
            .to_string_lossy()
            .starts_with('.'))
    }
}