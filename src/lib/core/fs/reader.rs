use super::DataMap;
use super::{Content, Data, Html, Markdown};
use crate::alert_cli;
use crate::core::config::Config;
use crate::core::data::PageData;
use crate::core::markdown::MarkdownDataExtractor;
use crate::core::markdown::MarkdownParser;
use crate::core::IntoInner;
use crate::error::Error;

use std::fmt::Debug;

use std::fs::copy;
use std::fs::create_dir;

use std::fs::File;

use colored::Colorize;
use std::fs::read_dir;
use std::fs::remove_dir_all;
use std::io::{BufReader, BufWriter, Read, Write};
use std::path::PathBuf;
use tera::{Context, Tera};
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
    pub data: Option<DataMap>,
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
    pub fn new(
        path: PathBuf,
        content: T,
        ext: String,
        file_name: String,
        data: Option<DataMap>,
    ) -> Self {
        Self {
            path,
            content,
            ext,
            file_name,
            data,
        }
    }

    pub fn data_as_context(&self) -> Option<Context> {
        match &self.data {
            Some(d) => {
                let mut c = Context::new();
                d.iter().for_each(|i| c.insert(&i.0.clone(), &i.1));

                Some(c)
            }
            None => None,
        }
    }

    pub fn get_data(&self) -> Option<DataMap> {
        self.data.clone()
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
            remove_dir_all(&config.out_dir)?;
            create_dir(&config.out_dir)?;
            return Ok(());
        }
        let config = Config::read_config()?;
        alert_cli!(
            format!("\u{1F916} | Creating {}.", &config.out_dir.green()),
            bold
        );
        create_dir(PathBuf::from(config.out_dir))?;
        Ok(())
    }

    pub fn start_creating_files(pages: &Vec<FileHolder<Data<Html>>>) -> Result<(), Error> {
        let config = Config::read_config()?;
        for page in pages {
            let _ = page
                .path
                .to_str()
                .unwrap()
                .replace(&config.source, &config.out_dir);
            alert_cli!(format!("\u{1F916} | Creating {}.", &page.file_name), bold);
            Writer::write(
                PathBuf::from(format!("{}/{}", config.out_dir, page.file_name)),
                page.content.clone(),
            )?;
        }
        create_dir(format!("{}/assets", &config.out_dir))?;
        copy_images()?;
        let other_file = Reader::new(config.clone().source.into()).read_other()?;
        for files in other_file {
            let _ = files
                .path
                .to_str()
                .unwrap()
                .replace(&config.source, &config.out_dir);
            alert_cli!(
                format!("\u{1F916} | Creating {}.", &files.file_name.green()),
                bold
            );
            match files.ext.as_str() {
                "png" | "jpeg" => continue,
                _ => {
                    Writer::write(
                        PathBuf::from(format!("{}/{}", config.out_dir, files.file_name)),
                        files.content.clone(),
                    )?;
                }
            }
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
        alert_cli!(
            format!("\u{1F4DD} | Writing {}.", path.to_str().unwrap().green()),
            bold
        );
        let content = content.into_inner().into_inner();
        let file = File::create(path)?;
        let mut writer = BufWriter::new(file);
        writer.write_all(content.as_bytes())?;
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

    pub fn read_other(&self) -> Result<Vec<FileHolder<Data<FileContent>>>, Error> {
        let config = Config::read_config()?;
        let mut data = vec![];
        read_push_other_files(&config.source.into(), &mut data)?;
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
            let file_name = path.to_str().unwrap().split('.').collect::<Vec<_>>()[0];
            let path_ext = path.extension().unwrap().to_str().unwrap();

            match path_ext {
                "html" => {
                    let file_data: Data<Html> = Reader::reader_out(path.to_path_buf())?.into();

                    let file_holder = FileHolder::new(
                        path.clone(),
                        file_data,
                        "html".to_owned(),
                        file_name.to_string(),
                        None,
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
                        None,
                    );
                    data.push(Box::new(file_holder));
                }
                _ => continue,
            }
        }
        Ok(data)
    }
}

pub fn start_convert_and_parse(
    files: Vec<Box<dyn Content>>,
) -> Result<Vec<FileHolder<Data<Html>>>, Error> {
    let mut output = Vec::new();
    for file in files {
        let downcasted = file.downcast_ref::<FileHolder<Data<Markdown>>>();
        match downcasted {
            Some(f) => {
                let extracted = MarkdownDataExtractor::new(f.content.clone()).extract();
                let file_content = extracted.content.clone().into_inner();
                let markdown_parser = MarkdownParser::new(Data::new(file_content));
                let f_clone = f.clone();
                output.push(FileHolder::new(
                    f_clone.path,
                    Data::new(markdown_parser.convert().into_inner()),
                    "md".to_string(),
                    f_clone.file_name,
                    extracted.data,
                ));
            }
            None => {
                let downcasted_html = file.downcast_ref::<FileHolder<Data<Html>>>();
                match downcasted_html {
                    Some(f) => {
                        // -- NOT STABLE --
                        // -- Haider-Ali-Dev --
                        // This part is not stable because it is just ignoring the file which has errors, because the data extracted from
                        // markdown files is used as expressions in other HTML files, so it gives error, I have just ignored that error, we might
                        // face other errors which makes this unstable and gives poor UI.
                        let mut temp = Tera::default();
                        let mut context = Context::new();
                        PageData::extract(&mut context)?;
                        let mut file_content = f.clone().content.into_inner();
                        let f_clone = f.clone();
                        temp.add_raw_template(
                            &f_clone.file_name,
                            &f_clone.content.into_inner().into_inner(),
                        )?;
                        if file_content.into_inner().contains("restic-markdown-dir") {
                            output.push(FileHolder::new(
                                f_clone.path,
                                Data::new(file_content),
                                "html".to_string(),
                                f_clone.file_name,
                                f_clone.data,
                            ));
                        } else {
                            let oc = temp.render(&f_clone.file_name, &context)?;
                            file_content = Html::new(&oc);
                            // -- NOT STABLE --
                            output.push(FileHolder::new(
                                f_clone.path,
                                Data::new(file_content),
                                "html".to_string(),
                                f_clone.file_name,
                                f_clone.data,
                            ));
                        }
                    }
                    None => {
                        panic!("Error")
                    }
                }
            }
        }
    }
    Ok(output)
}

pub fn read(path: &str) -> Result<Vec<Box<dyn Content>>, Error> {
    let path = PathBuf::from(path);
    let mut data = Vec::new();
    read_push(&path, &mut data)?;
    Ok(data)
}

fn read_push(path: &PathBuf, data: &mut Vec<Box<dyn Content>>) -> Result<(), Error> {
    let dir_data = std::fs::read_dir(&path)?
        .map(|f| f.unwrap())
        .map(|f| f.path())
        .collect::<Vec<_>>();

    for path in &dir_data {
        match path.is_dir() {
            false => {
                let file_name = path.file_name().unwrap().to_str().unwrap();
                let path_ext = path.extension().unwrap().to_str().unwrap();
                match path_ext {
                    "html" => {
                        let file_data: Data<Html> = Reader::reader_out(path.to_path_buf())?.into();
                        if file_data
                            .file_content
                            .into_inner()
                            .contains("restic-component")
                        {
                            continue;
                        }
                        let file_holder = FileHolder::new(
                            path.clone(),
                            file_data,
                            "html".to_owned(),
                            file_name.to_string(),
                            None,
                        );
                        data.push(Box::new(file_holder));
                    }
                    "md" => {
                        let file_data: Data<Markdown> =
                            Reader::reader_out(path.to_path_buf())?.into();
                        let file_holder = FileHolder::new(
                            path.clone(),
                            file_data,
                            "md".to_owned(),
                            file_name.to_string(),
                            None,
                        );
                        data.push(Box::new(file_holder));
                    }
                    _ => continue,
                }
            }
            true => {
                read_push(path, data).unwrap();
            }
        }
    }

    Ok(())
}

fn read_push_other_files(
    path: &PathBuf,
    data: &mut Vec<FileHolder<Data<FileContent>>>,
) -> Result<(), Error> {
    let dir_data = std::fs::read_dir(&path)?
        .map(|f| f.unwrap())
        .map(|f| f.path())
        .collect::<Vec<_>>();

    for path in &dir_data {
        let config = Config::read_config()?;
        if let Some(v) = config.exclude {
            if v.contains(&path.to_str().unwrap().to_owned()) {
                continue;
            }
        }
        match path.is_dir() {
            false => {
                let file_name = path.file_name().unwrap().to_str().unwrap();
                let path_ext = path.extension().unwrap().to_str().unwrap();
                match path_ext {
                    "html" | "md" | "png" | "jpeg" | "ico" => continue,
                    _ => {
                        let file_data = Reader::reader_out(path.to_path_buf())?;
                        let file_holder = FileHolder::new(
                            path.clone(),
                            file_data,
                            path_ext.to_owned(),
                            file_name.to_string(),
                            None,
                        );
                        data.push(file_holder);
                    }
                }
            }
            true => {
                read_push_other_files(path, data).unwrap();
            }
        }
    }

    Ok(())
}

fn copy_images() -> Result<(), Error> {
    let config = Config::read_config()?;
    let dir_data = match read_dir(format!("{}/assets", config.source)) {
        Ok(a) => a.collect::<Vec<_>>(),
        Err(_) => return Err(
            Error::EssentialFolderNotExist("`assets` folder is an essential folder, it should exist even if its empty or will be kept empty".to_string())
        )
    };
    for image in dir_data {
        let image = image?;
        copy(
            image.path(),
            format!(
                "{}/assets/{}",
                config.out_dir,
                image.file_name().to_str().unwrap()
            ),
        )?;
    }
    Ok(())
}

impl IntoInner for String {
    type Output = String;

    fn into_inner(&self) -> Self::Output {
        self.to_owned()
    }
}
