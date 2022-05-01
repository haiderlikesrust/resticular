use minify_html::{minify, Cfg};

use crate::{
    core::{fs::Html, IntoInner},
};

pub struct HtmlMinifier;

impl HtmlMinifier {
    pub fn minify(content: Html) -> Html {
        let content = content.into_inner();
        let mut cfg = Cfg::new();
        cfg.minify_js = true;
        cfg.minify_css = true;
        let minified = minify(&content.as_bytes(), &cfg);
        Html::new(&String::from_utf8(minified).unwrap()) 

    }
}

#[cfg(test)]
mod test {
    use crate::core::{fs::Html, IntoInner};

    use super::HtmlMinifier;

    #[test]
    pub fn check_minfiy() {
        let foo_html = Html::new(
            r#"
        <p       >wow<     /p>
        <!-- Test -->
        "#,
        );
        let minify = HtmlMinifier::minify(foo_html);
        assert_eq!(minify.into_inner(), "<p>wow< /p> ")
    }
}
