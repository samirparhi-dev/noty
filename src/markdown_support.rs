use pulldown_cmark::{html, Parser};

pub fn render_markdown(input: &str) -> String {
    let parser = Parser::new(input);
    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);
    html_output
}
