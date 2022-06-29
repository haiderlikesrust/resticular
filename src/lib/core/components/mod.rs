use crate::core::fs::Html;
use crate::error::Error;
use axum::http::uri::PathAndQuery;
use fs_extra::file;
use lol_html::{element, HtmlRewriter, Settings};
use scraper::Selector;
use soup::NodeExt;
use soup::{QueryBuilderExt, Soup};
use std::cell::RefCell;
use std::{fs::read_dir, path::PathBuf};
use tracing_subscriber::fmt::format;

use super::{
    config::Config,
    fs::{
        reader::{FileHolder, Reader},
        Data,
    },
    IntoInner,
};
pub struct Component {
    pub name: String,
    pub value: String,
    pub path: PathBuf,
}

struct ComponentReader;

impl ComponentReader {
    fn read(path: &str, data: &mut Vec<Component>) -> Result<(), Error> {
        let components = read_dir(path)?
            .map(|f| f.unwrap())
            .map(|f| f.path())
            .collect::<Vec<_>>();

        for path in &components {
            match path.is_dir() {
                false => {
                    let file_name = path.file_name().unwrap().to_str().unwrap();
                    let path_ext = path.extension().unwrap().to_str().unwrap();
                    match path_ext {
                        "md" | "png" | "jpeg" => continue,
                        _ => {
                            let file_data = Reader::reader_out(path.to_path_buf())?;
                            // let soup = Soup::new(&file_data.into_inner().into_inner());

                            let tag =
                                scraper::Html::parse_fragment(&file_data.into_inner().into_inner());
                            let selector = Selector::parse("restic-component").unwrap();
                            let mut name = String::new();
                            for element in tag.select(&selector) {
                                name = element.value().attr("name").unwrap().to_owned();
                            }
                            let c = Component {
                                name,
                                value: file_data.into_inner().into_inner(),
                                path: path.clone(),
                            };
                            data.push(c);
                        }
                    }
                }
                true => {
                    ComponentReader::read(path.to_str().unwrap(), data).unwrap();
                }
            }
        }

        Ok(())
    }
}

impl Component {
    pub fn read() -> Result<Vec<Component>, Error> {
        let mut data = vec![];
        let config = Config::read_config()?;
        ComponentReader::read(&format!("{}/components", config.source), &mut data)?;
        Ok(data)
    }
    pub fn name(&self) -> String {
        self.name.clone()
    }

    pub fn path(&self) -> PathBuf {
        self.path.clone()
    }

    pub fn data(&self) -> String {
        self.value.clone()
    }

    pub fn replace(
        components: Vec<Component>,
        pages: &Vec<FileHolder<Data<Html>>>,
    ) -> Result<Vec<FileHolder<Data<Html>>>, Error> {
        let mut oc = vec![];

        for page in pages {
            let mut output = vec![];

            let mut ele = vec![];
            for component in &components {
                ele.push(element!(&component.name(), |el| {
                    el.append(&component.data(), lol_html::html_content::ContentType::Html);
                    Ok(())
                }));
            }

            let mut rewriter = HtmlRewriter::new(
                Settings {
                    element_content_handlers: ele,
                    ..Settings::default()
                },
                |c: &[u8]| output.extend_from_slice(c),
            );
            rewriter.write(page.content.into_inner().into_inner().as_bytes())?;
            let page_clone = page.clone();
            let holder = FileHolder::new(
                page_clone.path,
                Data::new(Html::new(
                    &String::from_utf8(output.clone()).unwrap(),
                )),
                page_clone.ext,
                page_clone.file_name,
                page_clone.data,
            );
            oc.push(holder);
        }

        Ok(oc)
    }
}
