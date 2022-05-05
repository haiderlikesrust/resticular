pub mod minify;
use regex::Regex;
use scraper::Selector;
use std::cell::RefCell;
use std::rc::Rc;

use lol_html::{element, HtmlRewriter, Settings};
use lol_html::{rewrite_str, RewriteStrSettings};

use crate::error::Error;

use super::config::Config;

use super::{
    fs::{reader::FileHolder, Data, Html},
    IntoInner,
};

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
            rewriter.end();
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

    pub fn markdown_replace_writer(
        html_page: &FileHolder<Data<Html>>,
        markdown_page: &FileHolder<Data<Html>>,
    ) -> Result<FileHolder<Data<Html>>, Error> {
        let file_name = HtmlWriter::get_file_attr_val(&html_page)?;
        let config = Config::read_config().unwrap();

        if format!("{}/{}", config.dir, &file_name)
            == format!("{}", markdown_page.path.to_str().unwrap())
        {
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
        HtmlWriter::start_replacing(html_pages, markdown_pages)
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

    fn get_file_attr_val(page: &FileHolder<Data<Html>>) -> Result<String, Error> {
        let content = &page.content.into_inner().into_inner();
        let selector = Selector::parse("restic-markdown").unwrap();
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

        Ok(file.into_inner().to_owned())
    }
}

#[cfg(test)]
mod test {

    use crate::core::{
        fs::{reader::FileHolder, Data, Html},
        IntoInner,
    };

    use super::HtmlWriter;

    #[test]
    pub fn check_tag() {
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

    #[test]
    pub fn check_start_replacing() {
        let markdown_pages = vec![
            FileHolder::new(
                "source/some.md".into(),
                Data::new(Html::new(r#"<p><strong>Hello People</strong></p>"#.trim())),
                "md".to_string(),
                "source/some".to_string(),
            ),
            FileHolder::new(
                "source/some2.md".into(),
                Data::new(Html::new(r#"<p><strong>Hello Some2</strong></p>"#.trim())),
                "md".to_string(),
                "source/some2".to_string(),
            ),
        ];

        let html_pages = vec![
            FileHolder::new(
                "/some.html".into(),
                Data::new(Html::new(
                    r#"
                    <div>
                        <restic-markdown file="some.md"></restic-markdown>
                    </div>
                "#
                    .trim(),
                )),
                "html".to_string(),
                "some".to_string(),
            ),
            FileHolder::new(
                "/some.html".into(),
                Data::new(Html::new(
                    r#"
                    <div>
                        <restic-markdown file="some2.md"></restic-markdown>
                    </div>
                "#
                    .trim(),
                )),
                "html".to_string(),
                "some".to_string(),
            ),
        ];

        let replaced = HtmlWriter::start_replacing(html_pages, markdown_pages);
        let expected_output = vec![
            FileHolder::new(
                "source/some.md".into(),
                Data::new(Html::new(
                    r#"
                <div>
                        <restic-markdown file="some.md">
                        <p><strong>Hello People</strong></p>
                        </restic-markdown>
                    </div>
                "#,
                )),
                "md".to_string(),
                "source/some".to_string(),
            ),
            FileHolder::new(
                "source/some.md".into(),
                Data::new(Html::new(
                    r#"
                <div>
                        <restic-markdown file="some2.md">
                        <p><strong>Hello Some2</strong></p>
                        </restic-markdown>
                    </div>
                "#,
                )),
                "md".to_string(),
                "source/some".to_string(),
            ),
        ];
        assert_eq!(replaced.len(), 2);
        assert_eq!(
            expected_output[0].content.into_inner().into_inner().trim(),
            replaced[0].content.into_inner().into_inner().trim()
        );
    }
}

// press -> Files Read -> Markdown Parse -> HTML Convert -> l
