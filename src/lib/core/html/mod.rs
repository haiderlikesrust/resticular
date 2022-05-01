pub mod minify;

use lol_html::{element, HtmlRewriter, Settings};

use super::{
    fs::{reader::FileHolder, Content, Data, Html, Markdown},
    IntoInner,
};

pub struct HtmlWriter;

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
            rewriter.end();
            ou.push(FileHolder::new(
                c.path,
                Data::new(Html::new(&String::from_utf8(output).unwrap())),
                c.ext,
            ))
        }
        ou
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
        )];
        let d = HtmlWriter::lazy_images(foo)[0]
            .content
            .into_inner()
            .into_inner();
        assert_eq!(d, r#"<img src="foo.png" loading="lazy" />"#)
    }
}

// press -> Files Read -> Markdown Parse -> HTML Convert -> l
