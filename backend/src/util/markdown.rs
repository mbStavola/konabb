use std::io::Cursor;

use pulldown_cmark::{html, Event, Options, Parser, Tag};

pub fn parse_markdown(input_text: &str) -> Option<String> {
    let parser = Parser::new_ext(input_text, Options::empty())
        .map(|event| match event {
            Event::Html(h) | Event::InlineHtml(h) => Event::Text(h),
            _ => event,
        })
        .filter(|event| match event {
            Event::Start(Tag::Image(..)) | Event::End(Tag::Image(..)) => false,
            Event::Start(Tag::HtmlBlock) | Event::End(Tag::HtmlBlock) => false,
            _ => true,
        });

    let mut bytes = Vec::new();
    let r = html::write_html(Cursor::new(&mut bytes), parser);

    if r.is_ok() {
        Some(String::from_utf8_lossy(&bytes).to_string())
    } else {
        // Log this?
        None
    }
}
