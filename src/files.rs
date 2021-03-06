use crate::{Page, Meta};

use pulldown_cmark::{html, Options, Parser};

// convert a markdown file to html via a template and of the dynamic data
pub fn build_html(md: String, meta: Meta, page: Page) -> String {
    let mut html_template: String = match page {
        Page::Article        => include_str!("../assets/html/article.html").into(),
		Page::EditingArticle => include_str!("../assets/html/editing_article.html").into(),
        _                    => include_str!("../assets/html/default.html").into(),
    };

	html_template = match page {
		// sending html to the editing/adding article page
		// requires the plain markdown
		Page::EditingArticle => html_template.replace("{{ CONTENT }}", &md),
		
		_ => {
			// convert the markdown to html
			let mut options = Options::empty();
			options.insert(Options::ENABLE_STRIKETHROUGH);
			options.insert(Options::ENABLE_TABLES);
			let parser = Parser::new_ext(&md, options);
			let mut html_output: String = String::with_capacity(md.len() * 3 / 2);
			html::push_html(&mut html_output, parser);
		
			// inject the html template with content and metadata
			html_template.replace("{{ CONTENT }}", &html_output)
		}
	};
    
	// inject additional metadata
    for (key, val) in meta.into_iter() {
        html_template = html_template.replace(&format!("{{{{ {} }}}}", key), &val)
    }

    html_template
}
