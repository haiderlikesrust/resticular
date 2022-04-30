use crate::core::config::Config;
use crate::core::IntoInner;
use crate::error::Error;
use std::fs::File;
use std::io::{BufReader, BufWriter, Read, Write};
use std::path::PathBuf;

use super::{Content, Data, Html, Markdown};

pub struct FileContent(String);
impl IntoInner for FileContent {
    type Output = String;

    fn into_inner(self) -> Self::Output {
        self.0
    }
}

pub struct FileHolder<T> {
    pub path: PathBuf,
    pub content: T
}
impl<T> FileHolder<T> {
    pub fn new(path: PathBuf, content: T) -> Self {
        Self {
            path,
            content
        }
    }
}

impl<T> Content for FileHolder<T> {}
impl FileContent {
    pub fn new(content: String) -> Self {
        Self(content)
    }
}
impl From<FileContent> for Html {
    fn from(d: FileContent) -> Self {
        Self(d.0)
    }
}
impl From<FileContent> for Markdown {
    fn from(d: FileContent) -> Self {
        Self(d.0)
    }
}

impl From<Data<FileContent>> for Data<Html> {
    fn from(d: Data<FileContent>) -> Self {
        Self {
            file_content: d.into_inner().into(),
        }
    }
}

impl From<Data<FileContent>> for Data<Markdown> {
    fn from(d: Data<FileContent>) -> Self {
        Self {
            file_content: d.into_inner().into(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Writer;
impl Writer {
    pub fn write<T: IntoInner<Output = String>>(path: PathBuf, content: Data<T>) -> Result<(), Error> {
        let content = content.into_inner().into_inner();
        let file = File::create(path)?;
        let mut writer = BufWriter::new(file);
        writer.write(content.as_bytes())?;
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct Reader {
    pub path: PathBuf,
}

impl Reader {
    pub fn new(path: PathBuf) -> Self {
        Self { path }
    }

    pub fn reader(&self) -> Result<Data<FileContent>, Error> {
        let mut buffer = String::new();
        let f = File::open(&self.path)?;
        let mut reader = BufReader::new(f);
        reader.read_to_string(&mut buffer)?;
        Ok(Data::new(FileContent::new(buffer)))
    }

    pub fn reader_out(path: PathBuf) -> Result<Data<FileContent>, Error> {
        let mut buffer = String::new();
        let f = File::open(path)?;
        let mut reader = BufReader::new(f);
        reader.read_to_string(&mut buffer)?;
        Ok(Data::new(FileContent::new(buffer)))
    }

    pub fn read_dir_files(&self) -> Result<Vec<Box<dyn Content>>, Error> {
        let mut data: Vec<Box<dyn Content>> = Vec::new();
        let dir_data = std::fs::read_dir(&self.path)?
            .map(|f| f.unwrap())
            .map(|f| f.path())
            .collect::<Vec<_>>();

        for path in &dir_data {
            let path_ext = path.extension().unwrap().to_str().unwrap();
            match path_ext {
                "html" => {
                    let file_data: Data<Html> = Reader::reader_out(path.to_path_buf())?.into();
                    let file_holder = FileHolder::new(path.clone(), file_data);
                    data.push(Box::new(file_holder));
                }
                "md" => {
                    let file_data: Data<Markdown> = Reader::reader_out(path.to_path_buf())?.into();
                    let file_holder = FileHolder::new(path.clone(), file_data);
                    data.push(Box::new(file_holder));
                }
                _ => continue,
            }
        }
        Ok(data)
    }
}
