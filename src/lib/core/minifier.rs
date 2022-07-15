use crate::core::IntoInner;
use std::sync::Arc;

use super::fs::{reader::FileContent, Data};
use parcel_css::{
    stylesheet::{MinifyOptions, ParserOptions, PrinterOptions, StyleSheet},
    targets::Browsers,
};
use swc::{
    self,
    config::{Config, JsMinifyFormatOptions, JsMinifyOptions, JscConfig, Options},
    BoolOrDataConfig,
};
use swc_common::{
    errors::{ColorConfig, Handler},
    SourceMap,
};
use swc_ecma_minifier::option::terser::TerserEcmaVersion;
pub enum MinifyType {
    JavaScript,
    CSS,
}
pub struct Minifier;

impl Minifier {
    fn js_core_minify(v: &str, file_name: &str) -> String {
        let cm = Arc::<SourceMap>::default();
        let handler = Arc::new(Handler::with_tty_emitter(
            ColorConfig::Auto,
            true,
            false,
            Some(cm.clone()),
        ));
        let c = swc::Compiler::new(cm.clone());

        let fm = cm.new_source_file(
            swc_common::FileName::Custom(file_name.to_string()),
            v.to_string(),
        );
        c.process_js_file(
            fm,
            &handler,
            &Options {
                config: Config {
                    minify: true.into(),
                    ..Default::default()
                },
                ..Default::default()
            },
        )
        .expect(&format!("Look into {} file, there is an error", &file_name))
        .code
    }
    // pub fn css_core_minify(v: &str, file_name: &str) -> String {
    //     let stylesheet = StyleSheet::parse(
    //         &file_name,
    //         &v,
    //         ParserOptions {
    //             ..Default::default()
    //         },
    //     );
    //     match stylesheet {
    //         Ok(mut val) => {
    //             val.minify(MinifyOptions::default()).unwrap();
    //             let res = val.to_css(PrinterOptions::default()).unwrap();
    //             res.code
    //         }
    //         Err(e) => {
    //             panic!(
    //                 "Look into {} file, there is an error. [ERROR: {}]",
    //                 &file_name,
    //                 e.to_string()
    //             );
    //         }
    //     }
    // }
    pub fn minify(content: &mut Data<FileContent>, file_name: &str, tt: MinifyType) {
        match tt {
            MinifyType::JavaScript => {
                let c = content.into_inner().into_inner();
                let update_content = Minifier::js_core_minify(&c, file_name);
                content.file_content = FileContent::new(update_content);
            }
            _ => ()
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn check_file() {
        let code = r#"
        function foo() {
            let a = "";
            a = "e";
            a = 1;
            console.log(a)
            console.log([])
        }
        "#;
        let mut c = Data::new(FileContent::new(code.to_owned()));
        Minifier::minify(&mut c, "script.js", MinifyType::JavaScript);
        assert_eq!(
            c.into_inner().into_inner(),
            "function foo(){var a=\"\";a=\"e\";a=1;console.log(a);console.log([])}".to_owned()
        )
    }
    #[test]
    fn minfiy_css() {
        let code = r#"
        
  .foo {
    color: red;
  }

  .bar {
    color: red;
  }
        "#;
        let mut c = Data::new(FileContent::new(code.to_owned()));
        Minifier::minify(&mut c, "script.css", MinifyType::CSS);
        assert_eq!(
            c.into_inner().into_inner(),
            "function foo(){var a=\"\";a=\"e\";a=1;console.log(a);console.log([])}".to_owned()
        )
    }
}
