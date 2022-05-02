pub mod minify;
use std::cell::RefCell;
use std::collections::HashMap;
use std::io::Write;
use std::ops::Index;
use std::rc::Rc;

use lol_html::{element, HtmlRewriter, Settings};
use lol_html::{rewrite_str, RewriteStrSettings};

use super::config::Config;
use super::fs::reader::Reader;
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
    ) -> FileHolder<Data<Html>> {
        let mut output = Vec::new();
        let mut rewritter = HtmlRewriter::new(
            Settings {
                element_content_handlers: vec![element!("restic-markdown", |el| {
                    let config = Config::read_config().unwrap();
                    let file = el.get_attribute("file").unwrap();
                    if format!("{}/{}", config.dir, file)
                        == format!("{}", markdown_page.path.to_str().unwrap())
                    {
                        el.append(
                            &markdown_page.content.into_inner().into_inner(),
                            lol_html::html_content::ContentType::Html,
                        );
                    }
                    Ok(())
                })],
                ..Settings::default()
            },
            |c: &[u8]| output.extend_from_slice(c),
        );

        let data = html_page.content.into_inner();
        rewritter.write(data.into_inner().as_bytes()).unwrap();
        println!("Writer");
        let html_page = html_page.clone();
        rewritter.end().unwrap();
        FileHolder::new(
            html_page.path,
            Data::new(Html::new(&String::from_utf8(output.clone()).unwrap())),
            html_page.ext,
            html_page.file_name,
        )
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
    start_replacing(html_pages, markdown_pages)
}

pub fn start_replacing(
    html_pages: Vec<FileHolder<Data<Html>>>,
    markdown_pages: Vec<FileHolder<Data<Html>>>,
) -> Vec<FileHolder<Data<Html>>> {
    let mut pages = vec![];
    html_pages.iter().for_each(|html_page| {
        let markdown_page = markdown_pages.iter().next().unwrap();
        let html_page = HtmlWriter::markdown_replace_writer(html_page, markdown_page);
        pages.push(FileHolder::new(
            html_page.path,
            html_page.content,
            html_page.ext,
            html_page.file_name,
        ));
        println!("Push");
    });

    // for html_page in &html_pages {
    //
    //     for markdown_page in &markdown_pages {
    //

    //         let html_page = HtmlWriter::markdown_replace_writer(html_page, markdown_page);

    //         pages.push(FileHolder::new(
    //             html_page.path,
    //             html_page.content,
    //             html_page.ext,
    //             html_page.file_name,
    //         ));
    //     }
    // }
    // pages.clone().iter().for_each(|_f| {
    //     pages.dedup_by(|a, b| {
    //         if a.file_name == b.file_name  {
    //             a.content.into_inner().into_inner().len()
    //                 > b.content.into_inner().into_inner().len() || a.content.into_inner().into_inner().len()
    //             < b.content.into_inner().into_inner().len()
    //         } else {
    //             false
    //         }
    //     });
    //     pages.dedup();
    // });

    pages
}

#[cfg(test)]
mod test {

    use crate::core::{
        fs::{reader::FileHolder, Data, Html, Markdown},
        IntoInner,
    };

    use super::{start_replacing, HtmlWriter};
    use crate::core::markdown::MarkdownParser;

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

        let replaced = start_replacing(html_pages, markdown_pages);
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
