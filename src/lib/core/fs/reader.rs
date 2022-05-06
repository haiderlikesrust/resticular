use super::{Content, Data, Html, Markdown};
use crate::core::config::Config;
use crate::core::markdown::MarkdownParser;
use crate::core::IntoInner;
use crate::error::Error;
use minifier::css;
use minifier::js;
use std::fmt::Debug;
use std::fs::create_dir;
use std::fs::File;
use std::fs::{read_dir, remove_file};
use std::io::{BufReader, BufWriter, Read, Write};
use std::path::PathBuf;
use tracing::info;

#[derive(Debug, Clone)]
pub struct FileContent(String);
impl IntoInner for FileContent {
    type Output = String;

    fn into_inner(&self) -> Self::Output {
        self.0.to_owned()
    }
}
#[derive(Debug, Clone)]
pub struct FileHolder<T> {
    pub path: PathBuf,
    pub content: T,
    pub ext: String,
    pub file_name: String,
}

impl PartialEq for FileHolder<Data<Html>> {
    fn eq(&self, other: &Self) -> bool {
        self.path == other.path
            && self.content == other.content
            && self.ext == other.ext
            && self.file_name == other.file_name
    }
}
impl<T> FileHolder<T> {
    pub fn new(path: PathBuf, content: T, ext: String, file_name: String) -> Self {
        Self {
            path,
            content,
            ext,
            file_name,
        }
    }
}

impl<T: 'static + Debug> Content for FileHolder<T> {}
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

pub struct FolderBuilder;

impl FolderBuilder {
    pub fn check_build_dir() -> bool {
        let config = Config::read_config().unwrap();
        let dir = read_dir(config.out_dir);
        match dir {
            Ok(_) => true,
            Err(_) => false,
        }
    }
    pub fn create_folder() -> Result<(), Error> {
        if FolderBuilder::check_build_dir() {
            let config = Config::read_config()?;
            let dir_content = read_dir(config.out_dir)?.collect::<Vec<_>>();
            for file in dir_content {
                let file = file?.path();
                remove_file(file)?;
            }

            return Ok(());
        }
        let config = Config::read_config()?;
        info!("Creating {}.", &config.out_dir);
        create_dir(PathBuf::from(config.out_dir))?;
        Ok(())
    }

    pub fn start_creating_files(pages: &Vec<FileHolder<Data<Html>>>) -> Result<(), Error> {
        let config = Config::read_config()?;
        for page in pages {
            let file_name = page.path.to_str().unwrap().split("/").collect::<Vec<_>>()[1];
            info!("Creating {}.", &file_name);
            Writer::write(
                PathBuf::from(format!("{}/{}", config.out_dir, file_name)),
                page.content.clone(),
            )?;
        }

        let stylesheets = Reader::new(config.clone().dir.into()).read_stylesheets()?;
        for stylesheet in stylesheets {
            let file_name = stylesheet
                .path
                .to_str()
                .unwrap()
                .split("/")
                .collect::<Vec<_>>()[1];
            info!("Creating {}.", &file_name);
            Writer::write(
                PathBuf::from(format!("{}/{}", config.out_dir, file_name)),
                Data::new(css::minify(&stylesheet.content.into_inner()).unwrap()),
            )?;
        }
        let script_files = Reader::new(config.clone().dir.into()).read_script_files()?;
        for script_file in script_files {
            let file_name = script_file
                .path
                .to_str()
                .unwrap()
                .split("/")
                .collect::<Vec<_>>()[1];
            info!("Creating {}.", &file_name);
            Writer::write(
                PathBuf::from(format!("{}/{}", config.out_dir, file_name)),
                Data::new(js::minify(&script_file.content.file_content)),
            )?;
        }

        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct Writer;
impl Writer {
    pub fn write<T: IntoInner<Output = String> + Clone>(
        path: PathBuf,
        content: Data<T>,
    ) -> Result<(), Error> {
        info!("Writing {}.", path.to_str().unwrap());
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

    pub fn read_stylesheets(&self) -> Result<Vec<FileHolder<Data<String>>>, Error> {
        let mut data = vec![];
        let dir_data = std::fs::read_dir(&self.path)?
            .map(|f| f.unwrap())
            .map(|f| f.path())
            .collect::<Vec<_>>();

        for path in dir_data {
            let file_name = path.to_str().unwrap().split(".").collect::<Vec<_>>()[0];
            let path_ext = path.extension();
            match path_ext {
                Some(p) => {
                    if p.to_str().unwrap() == "css" {
                        let file_data = Reader::reader_out(path.to_path_buf())?;
                        let file_holder = FileHolder::new(
                            path.clone(),
                            Data::new(file_data.into_inner().into_inner()),
                            "css".to_owned(),
                            file_name.to_string(),
                        );
                        data.push(file_holder);
                    }
                }
                _ => continue,
            }
        }

        Ok(data)
    }

    pub fn read_script_files(&self) -> Result<Vec<FileHolder<Data<String>>>, Error> {
        let mut data = vec![];
        let dir_data = std::fs::read_dir(&self.path)?
            .map(|f| f.unwrap())
            .map(|f| f.path())
            .collect::<Vec<_>>();

        for path in dir_data {
            let file_name = path.to_str().unwrap().split(".").collect::<Vec<_>>()[0];
            let path_ext = path.extension();
            match path_ext {
                Some(p) => {
                    if p.to_str().unwrap() == "js" {
                        let file_data = Reader::reader_out(path.to_path_buf())?;
                        let file_holder = FileHolder::new(
                            path.clone(),
                            Data::new(file_data.into_inner().into_inner()),
                            "css".to_owned(),
                            file_name.to_string(),
                        );
                        data.push(file_holder);
                    }
                }
                _ => continue,
            }
        }
        Ok(data)
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
            let file_name = path.to_str().unwrap().split(".").collect::<Vec<_>>()[0];
            let path_ext = path.extension().unwrap().to_str().unwrap();
            match path_ext {
                "html" => {
                    let file_data: Data<Html> = Reader::reader_out(path.to_path_buf())?.into();
                    let file_holder = FileHolder::new(
                        path.clone(),
                        file_data,
                        "html".to_owned(),
                        file_name.to_string(),
                    );
                    data.push(Box::new(file_holder));
                }
                "md" => {
                    let file_data: Data<Markdown> = Reader::reader_out(path.to_path_buf())?.into();
                    let file_holder = FileHolder::new(
                        path.clone(),
                        file_data,
                        "md".to_owned(),
                        file_name.to_string(),
                    );
                    data.push(Box::new(file_holder));
                }
                _ => continue,
            }
        }
        Ok(data)
    }

    

    fn find_files(
        path: &PathBuf,
        file_name: &str,
        ext: &str,
    ) -> Result<FileHolder<Data<Html>>, Error> {
        let file_data: Data<Html> = Reader::reader_out(path.to_path_buf())?.into();
        let file_holder = FileHolder::new(
            path.clone(),
            file_data,
            ext.to_owned(),
            file_name.to_string(),
        );
        Ok(file_holder)
    }
}

pub fn start_convert_and_parse(files: Vec<Box<dyn Content>>) -> Vec<FileHolder<Data<Html>>> {
    let mut output = Vec::new();
    for file in files {
        let downcasted = file.downcast_ref::<FileHolder<Data<Markdown>>>();
        match downcasted {
            Some(f) => {
                let file_content = f.clone().content.into_inner();
                let markdown_parser = MarkdownParser::new(Data::new(file_content));
                let f_clone = f.clone();
                output.push(FileHolder::new(
                    f_clone.path,
                    Data::new(markdown_parser.convert().into_inner()),
                    "md".to_string(),
                    f_clone.file_name,
                ));
            }
            None => {
                let downcasted_html = file.downcast_ref::<FileHolder<Data<Html>>>();
                match downcasted_html {
                    Some(f) => {
                        let file_content = f.clone().content.into_inner();
                        let f_clone = f.clone();
                        output.push(FileHolder::new(
                            f_clone.path,
                            Data::new(file_content),
                            "html".to_string(),
                            f_clone.file_name,
                        ));
                    }
                    None => {
                        panic!("Error")
                    }
                }
            }
        }
    }
    output
}

impl IntoInner for String {
    type Output = String;

    fn into_inner(&self) -> Self::Output {
        self.to_owned()
    }
}
