use crate::{Page, Meta};

use std::fs;
use pulldown_cmark::{html, Options, Parser};


// read a file


// convert a markdown file to html via a template and of the dynamic data
pub fn build_html(md: String, meta: Meta, page: Page) -> String {
    let mut html_template: String = match page {
        Page::Article => include_str!("../assets/html/article.html").into(),
        _             => include_str!("../assets/html/default.html").into(),
    };

    // convert the markdown to html
    let mut options = Options::empty();
    options.insert(Options::ENABLE_STRIKETHROUGH);
    let parser = Parser::new_ext(&md, options);
    let mut html_output: String = String::with_capacity(md.len() * 3 / 2);
    html::push_html(&mut html_output, parser);

    // inject the html template with content and metadata
    html_template = html_template.replace("{{ CONTENT }}", &html_output);
    for (key, val) in meta.into_iter() {
        html_template = html_template.replace(&format!("{{{{ {} }}}}", key), &val)
    }

    html_template
}