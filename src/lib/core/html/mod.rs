pub mod minify;
use lazy_static::lazy_static;
use regex::Regex;
use scraper::Selector;
use std::cell::RefCell;
use std::collections::HashMap;
use std::path::PathBuf;
use std::rc::Rc;
use tera::{Context, Template, Tera};

use lol_html::{element, HtmlRewriter, Settings};
use lol_html::{rewrite_str, RewriteStrSettings};

use crate::error::Error;

use super::config::Config;

use super::fs::Markdown;
use super::{
    fs::{reader::FileHolder, Data, Html},
    IntoInner,
};

enum ResticTag {
    ResticMarkdown,
    ResticMarkdownDir,
}

lazy_static! {
    pub static ref TEMPLATES: Tera = {
        let config = Config::read_config().unwrap();
        let mut tera = match Tera::new(&format!("{}/templates/**/*", config.out_dir)) {
            Ok(t) => t,
            Err(e) => {
                panic!("Tera error: {}", e);
            }
        };

        tera.autoescape_on(vec!["html"]);
        tera
    };
}

pub struct TemplateManager;
impl TemplateManager {
    pub fn replace(contents: Vec<FileHolder<Data<Html>>>) -> Vec<FileHolder<Data<Html>>> {
        let d = vec![];
        let mut context = Context::new();

        d
    }
}

pub struct HtmlWriter;
pub struct FileAttr(Rc<RefCell<String>>);

impl HtmlWriter {
    pub fn lazy_images(contents: Vec<FileHolder<Data<Html>>>) -> Vec<FileHolder<Data<Html>>> {
        let mut vec_of_outputs = vec![];
        for c in contents {
            let mut output = vec![];
            let mut rewriter = HtmlRewriter::new(
                Settings {
                    element_content_handlers: vec![element!("img", |el| {
                        let _ = el.set_attribute("loading", "lazy");
                        Ok(())
                    })],
                    ..Settings::default()
                },
                |c: &[u8]| output.extend_from_slice(c),
            );
            let cc = c.content.into_inner();
            let cc_clone = cc.clone().into_inner();
            rewriter.write(cc_clone.as_bytes()).unwrap();
            rewriter.end().unwrap();
            vec_of_outputs.push(FileHolder::new(
                c.path,
                Data::new(Html::new(&String::from_utf8(output).unwrap())),
                c.ext,
                c.file_name,
            ))
        }
        vec_of_outputs
    }

    pub fn add_link(contents: Vec<FileHolder<Data<Html>>>) -> Vec<FileHolder<Data<Html>>> {
        let mut ou: Vec<FileHolder<Data<Html>>> = vec![];
        for c in contents {
            let mut output = vec![];
            let mut rewriter = HtmlRewriter::new(
                Settings {
                    element_content_handlers: vec![element!("head", |el| {
                        el.append(
                            r#"<link type="text/css" rel="stylesheet" href="styles.css">"#,
                            lol_html::html_content::ContentType::Html,
                        );
                        Ok(())
                    })],
                    ..Settings::default()
                },
                |c: &[u8]| output.extend_from_slice(c),
            );
            let cc = c.content.into_inner();
            let cc_clone = cc.clone().into_inner();
            rewriter.write(cc_clone.as_bytes()).unwrap();
            rewriter.end().unwrap();
            ou.push(FileHolder::new(
                c.path,
                Data::new(Html::new(&String::from_utf8(output).unwrap())),
                c.ext,
                c.file_name,
            ))
        }
        ou
    }

    fn markdown_replace_writer(
        html_page: &FileHolder<Data<Html>>,
        markdown_page: &FileHolder<Data<Html>>,
    ) -> Result<FileHolder<Data<Html>>, Error> {
        let file_name = HtmlWriter::get_file_attr_val(html_page, ResticTag::ResticMarkdown)?;
        let config = Config::read_config().unwrap();

        if format!("{}/{}", config.source, &file_name) == *markdown_page.path.to_str().unwrap() {
            let element_content_handlers = vec![element!("restic-markdown", |el| {
                el.append(
                    &markdown_page.content.into_inner().into_inner(),
                    lol_html::html_content::ContentType::Html,
                );
                Ok(())
            })];

            let output = rewrite_str(
                &html_page.content.into_inner().into_inner(),
                RewriteStrSettings {
                    element_content_handlers,
                    ..RewriteStrSettings::default()
                },
            )
            .unwrap();
            Ok(FileHolder::new(
                html_page.clone().path,
                Data::new(Html::new(&output)),
                html_page.clone().ext,
                html_page.clone().file_name,
            ))
        } else {
            Err(Error::PageCheckError)
        }
    }
    pub fn replace_markdown(contents: Vec<FileHolder<Data<Html>>>) -> Vec<FileHolder<Data<Html>>> {
        let mut html_pages = vec![];
        let mut markdown_pages = vec![];
        for content in contents {
            match content.ext.as_str() {
                "md" => {
                    markdown_pages.push(content);
                }
                "html" => {
                    html_pages.push(content);
                }
                _ => (),
            }
        }
        // let h_c = html_pages.clone();
        // let m_c = markdown_pages.clone();
        let mut pages = HtmlWriter::start_replacing(html_pages, markdown_pages);
        // println!("{:#?}", &pages);
        // HtmlWriter::start_creating(&h_c, &m_c, &mut pages).unwrap();
        // // println!("{:#?}", &pages);
        pages
    }

    fn start_creating(
        html_pages: &Vec<FileHolder<Data<Html>>>,
        markdown_pages: &Vec<FileHolder<Data<Html>>>,
        pages: &mut Vec<FileHolder<Data<Html>>>
    ) -> Result<(), Error> {
        for html_page in html_pages {
            if html_page
                .clone()
                .content
                .into_inner()
                .into_inner()
                .contains("restic-markdown-dir")
            {
                for markdown_page in markdown_pages {
                    let file_attr: PathBuf =
                        HtmlWriter::get_file_attr_val(&html_page, ResticTag::ResticMarkdownDir)?
                            .into();
                    let page_path: PathBuf = markdown_page.path.parent().unwrap().into();
                    if page_path == file_attr {
                        HtmlWriter::markdown_replicator(html_page, markdown_page, pages);
                    }
                }
            }
        }
        Ok(())
    }

    fn markdown_replicator(
        html_page: &FileHolder<Data<Html>>,
        markdown_page: &FileHolder<Data<Html>>,
        pages: &mut Vec<FileHolder<Data<Html>>>,
    ) {
        let html_page_clone = html_page.clone();
        let markdown_page_clone = markdown_page.clone();
        let e_l_h = vec![element!("restic-markdown-dir", |e| {
            e.append(
                &markdown_page_clone.content.into_inner().into_inner(),
                lol_html::html_content::ContentType::Html,
            );
            Ok(())
        })];

        let output = rewrite_str(
            &html_page.content.into_inner().into_inner(),
            RewriteStrSettings {
                element_content_handlers: e_l_h,
                ..RewriteStrSettings::default()
            },
        )
        .unwrap();
        let e = format!(
            "{}-{}",
            html_page_clone.file_name, markdown_page_clone.file_name
        );
        let html_file_path: PathBuf = html_page_clone.path.parent().unwrap().into();
        let file_path = format!("{}/{}", html_file_path.display(), e);
        pages.push(FileHolder::new(
            file_path.into(),
            Data::new(Html::new(&output)),
            "html".to_string(),
            e,
        ))
    }

    fn start_replacing(
        html_pages: Vec<FileHolder<Data<Html>>>,
        markdown_pages: Vec<FileHolder<Data<Html>>>,
    ) -> Vec<FileHolder<Data<Html>>> {
        let mut pages = vec![];

        for html_page in &html_pages {
            if html_page
                .clone()
                .content
                .into_inner()
                .into_inner()
                .contains("restic-markdown")
            {
                for markdown_page in &markdown_pages {
                    let html_page = HtmlWriter::markdown_replace_writer(html_page, markdown_page);
                    match html_page {
                        Ok(html) => {
                            pages.push(FileHolder::new(
                                html.path,
                                html.content,
                                html.ext,
                                html.file_name,
                            ));
                        }
                        Err(_) => continue,
                    }
                }
            } else {
                pages.push(html_page.clone());
            }
        }
        pages
    }

    fn get_file_attr_val(page: &FileHolder<Data<Html>>, tty: ResticTag) -> Result<String, Error> {
        let _content = &page.content.into_inner().into_inner();
        match tty {
            ResticTag::ResticMarkdown => {
                let _selector = Selector::parse("restic-markdown").unwrap();
                let file = RefCell::new("".to_owned());
                let element_content_handlers = vec![element!("restic-markdown", |el| {
                    let file_attr = el.get_attribute("file").unwrap();
                    file.replace(file_attr);
                    Ok(())
                })];
                rewrite_str(
                    &page.content.into_inner().into_inner(),
                    RewriteStrSettings {
                        element_content_handlers,
                        ..RewriteStrSettings::default()
                    },
                )?;

                Ok(file.into_inner())
            }
            ResticTag::ResticMarkdownDir => {
                let _selector = Selector::parse("restic-markdown-dir").unwrap();
                let file = RefCell::new("".to_owned());
                let element_content_handlers = vec![element!("restic-markdown-dir", |el| {
                    let file_attr = el.get_attribute("path").unwrap();
                    file.replace(file_attr);
                    Ok(())
                })];
                rewrite_str(
                    &page.content.into_inner().into_inner(),
                    RewriteStrSettings {
                        element_content_handlers,
                        ..RewriteStrSettings::default()
                    },
                )?;

                Ok(file.into_inner())
            }
        }
    }
}

#[cfg(test)]
mod test {

    use std::path::{Path, PathBuf};

    use super::HtmlWriter;
    use crate::core::{
        fs::{reader::FileHolder, Data, Html},
        IntoInner,
    };

    #[test]
    fn check_path() {
        let a: PathBuf = PathBuf::from("/a/b/c.rs").parent().unwrap().into();
        let b = PathBuf::from("/a/b");
        assert_eq!(a, b)
    }

    #[test]
    fn check_tag() {
        let foo = vec![FileHolder::new(
            "/test.html".into(),
            Data::new(Html::new(r#"<img src="foo.png"/>"#)),
            "html".to_owned(),
            "some".to_owned(),
        )];
        let d = HtmlWriter::lazy_images(foo)[0]
            .content
            .into_inner()
            .into_inner();
        assert_eq!(d, r#"<img src="foo.png" loading="lazy" />"#)
    }
}

// press -> Files Read -> Markdown Parse -> HTML Convert -> l
