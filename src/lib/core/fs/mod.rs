use std::fmt::Debug;


use mopa::{Any, mopafy};
use super::IntoInner;
pub mod reader;

/// A generic structure for holding `data`
#[derive(Debug, Clone, PartialEq)]
pub struct Data<T> {
    /// `file_content` will be taken by the [`super::markdown::MarkdownParser`] struct and then
    /// the html will be parsed and will return another [`Data`] struct with HTML content.
    pub file_content: T,
}

impl<T> Data<T> {
    /// Function for creating a `[Data]` struct.
    pub fn new(d: T) -> Self {
        Self { file_content: d }
    }
}
#[derive(Debug, Clone, PartialEq)]
/// A newtype on top string, which is responsible for holding the html data
pub struct Html(String);
impl Html {
    /// Function for creating a `[Html]` struct.
    pub fn new(d: &str) -> Self {
        Self(d.to_string())
    }
}

pub trait Content: Any + Debug { }
mopafy!{Content}
#[derive(Debug, Clone)]
/// A newtype on top string, which is responsible for holding the markdown data
pub struct Markdown(String);
impl Markdown {
    /// Function for creating a `[Html]` struct.
    pub fn new(d: &str) -> Self {
        Self(d.to_string())
    }
}
impl From<String> for Markdown {
    fn from(d: String) -> Self {
        Self(d)
    }
}

impl From<String> for Html {
    fn from(d: String) -> Self {
        Self(d)
    }
}

impl From<&str> for Markdown {
    fn from(d: &str) -> Self {
        Self(d.to_string())
    }
}

impl From<&str> for Html {
    fn from(d: &str) -> Self {
        Self(d.to_string())
    }
}

impl IntoInner for Markdown {
    type Output = String;

    fn into_inner(&self) -> Self::Output {
        self.0.to_owned()
    }
}

impl IntoInner for Html {
    type Output = String;

    fn into_inner(&self) -> Self::Output {
        self.0.to_owned()
    }
}

impl<T: Clone> IntoInner for Data<T> {
    type Output = T;

    fn into_inner(&self) -> Self::Output {
        self.file_content.clone()
    }
}


impl Content for Html {}
impl Content for Markdown {}
impl<T: 'static + Debug> Content  for Data<T> {}