#![cfg(feature = "html")]

use pulldown_cmark::{Options, Parser};

#[rustfmt::skip]
mod suite;

pub struct TestMarkdownHtmlParams {
    pub input: &'static str,
    pub output: &'static str,
    pub smart_punct: bool,
    pub metadata_blocks: bool,
    pub old_footnotes: bool,
    pub subscript: bool,
    pub wikilinks: bool,
    pub deflists: bool,
}

#[inline(never)]
pub fn test_markdown_html(params: TestMarkdownHtmlParams) {
    let mut s = String::new();

    let mut opts = Options::empty();
    opts.insert(Options::ENABLE_MATH);
    opts.insert(Options::ENABLE_TABLES);
    opts.insert(Options::ENABLE_STRIKETHROUGH);
    opts.insert(Options::ENABLE_SUPERSCRIPT);
    if params.wikilinks {
        opts.insert(Options::ENABLE_WIKILINKS);
    }
    if params.subscript {
        opts.insert(Options::ENABLE_SUBSCRIPT);
    }
    opts.insert(Options::ENABLE_TASKLISTS);
    opts.insert(Options::ENABLE_GFM);
    if params.old_footnotes {
        opts.insert(Options::ENABLE_OLD_FOOTNOTES);
    } else {
        opts.insert(Options::ENABLE_FOOTNOTES);
    }
    if params.metadata_blocks {
        opts.insert(Options::ENABLE_YAML_STYLE_METADATA_BLOCKS);
        opts.insert(Options::ENABLE_PLUSES_DELIMITED_METADATA_BLOCKS);
    }
    if params.smart_punct {
        opts.insert(Options::ENABLE_SMART_PUNCTUATION);
    }
    opts.insert(Options::ENABLE_HEADING_ATTRIBUTES);
    if params.deflists {
        opts.insert(Options::ENABLE_DEFINITION_LIST);
    }

    let p = Parser::new_ext(params.input, opts);
    pulldown_cmark::html::push_html(&mut s, p);

    // normalizing the HTML using html5ever may hide actual errors
    // assert_eq!(html_standardize(output), html_standardize(&s));
    assert_eq!(html_standardize(params.output), html_standardize(&s));
}

fn html_standardize(s: &str) -> String {
    s.replace("<br>", "<br />")
        .replace("<br/>", "<br />")
        .replace("<hr>", "<hr />")
        .replace("<hr/>", "<hr />")
        // permit extra or missing line breaks only between tags
        .replace(">\n<", "><")
}
