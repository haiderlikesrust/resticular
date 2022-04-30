use pulldown_cmark::{html, Parser};

use super::{
    fs::{Data, Html, Markdown},
    IntoInner,
};

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
}
