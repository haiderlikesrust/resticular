use std::collections::HashMap;

use super::{
    fs::{Data, Html, Markdown},
    IntoInner,
};
use pulldown_cmark::{html, Parser};
use regex::Regex;

/// Structure responsible for parsing markdown into html.
/// ```
/// use resticular::core::{fs::*, markdown::*};
/// use crate::resticular::core::IntoInner;
/// fn main() {
///      let content: Data<Markdown> = Data::new("**Hello World**".into());
///      let parser = MarkdownParser::new(content);
///      let html = parser.convert().into_inner().into_inner();
///      assert_eq!("<p><strong>Hello World</strong></p>\n", html);
/// }
/// ```
///
pub struct MarkdownDataExtractor {
    pub content: Data<Markdown>,
}

pub struct ExtractorData {
    pub content: Data<Markdown>,
    pub data: Option<HashMap<String, String>>
}

impl MarkdownDataExtractor {
    pub fn new(content: Data<Markdown>) -> Self {
        Self { content }
    }

    pub fn extract(&mut self) -> ExtractorData {
        let mut data = HashMap::new();
        let regex = Regex::new(r"(?s)---\n(.*)\n---").unwrap();
        let text = self.content.into_inner().into_inner();
        let captures = regex.captures(&text);
        match captures {
            Some(c) => {
                let m = c.get(1).unwrap().as_str();
                let vec_of_data = m.split('\n').collect::<Vec<_>>();
                let into_hash = vec_of_data
                    .iter()
                    .map(|m| m.split('=').collect::<Vec<_>>())
                    .collect::<Vec<_>>();

                into_hash.iter().for_each(|d| {
                    data.insert(
                        d[0].trim().to_string(),
                        d[1].replace('"', "").trim().to_string(),
                    );
                });
                let new_content = regex.replace_all(&self.content.into_inner().into_inner(), "").to_string();
                self.content = Data::new(Markdown::new(&new_content));

            }
            None => return ExtractorData {
                content: self.content.clone(),
                data: None
            } ,
        }

        ExtractorData {
            content: self.content.clone(),
            data: Some(data)
        }
    }
}
pub struct MarkdownParser {
    /// This field holds the file content which should be a type of `Data<Markdown>`
    pub content: Data<Markdown>,
}

impl MarkdownParser {
    /// Function for creating a [`MarkdownParser`]` struct.
    pub fn new(content: Data<Markdown>) -> Self {
        Self { content }
    }

    /// This method does the conversion, first a `buffer` is created in this fucntion to hold the output data
    /// the content is given to the Parser which parses markdown to html and returns html.
    ///
    pub fn convert(&self) -> Data<Html> {
        let mut buffer = String::new();
        let content = self.content.clone().file_content.into_inner();
        let parser = Parser::new(&content);
        self.push_html(&mut buffer, parser);
        Data::new(Html::new(&buffer))
    }
    /// This method takes the parser containing the markdown data and pushes the html.
    fn push_html(&self, buffer: &mut String, parser: Parser) {
        html::push_html(buffer, parser);
    }
}


#[cfg(test)]
mod test {
    use std::collections::HashMap;

    use crate::core::{
        fs::{Data, Markdown},
        IntoInner,
    };

    #[test]
    fn check_html() {
        use super::MarkdownParser;

        let content: Data<Markdown> = Data::new("**Hello World**".into());
        let parser = MarkdownParser::new(content);
        let html = parser.convert().into_inner().into_inner();
        assert_eq!("<p><strong>Hello World</strong></p>\n", html)
    }

    #[test]

    fn extract_data() {
        use super::MarkdownDataExtractor;
        let content: Data<Markdown> = Data::new(
            r#"
---
name = "Haider"
---
**Hello World**"#
                .into(),
        );
        let mut extractor = MarkdownDataExtractor::new(content);
        let mut m = HashMap::new();
        m.insert("name".to_string(), "Haider".to_string());
        let c = extractor.extract();
        assert_eq!(m, c.data.unwrap());
        assert_eq!("\n\n**Hello World**", c.content.into_inner().into_inner())
    }
}
