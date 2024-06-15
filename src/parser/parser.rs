use crate::tokenizer::tokenizer::Token;

pub enum ParserType{
    ToHtmlConverter,
}

fn parse_to_html(input: Vec<Token>) -> String{
    let mut html = String::new();

    for token in input{
        match token{
            Token::BoldOpen => {
                html.push_str("<b>")
            }
            Token::BoldClose => {
                html.push_str("<b/>")
            }
            Token::ItalicOpen => {
                html.push_str("<i>")
            }
            Token::ItalicClose => {
                html.push_str("<i/>")
            }
            Token::Text(text) => {
                html.push_str(&text)
            }
            Token::Title(level, text) => {
                html.push_str(&format!("<h{}>{}</h{}>", level, text, level))
            }
            Token::Image(url) => {
                let img_tag = format!(r#"<img src="{}"/>"#, url);
                html.push_str(&img_tag)
            }
            _ => {
                panic!("unhandled token");
            }
        }
    }


    html
}


pub fn parse(input: Vec<Token>, convert_to: ParserType) -> String{
    match convert_to {
        ParserType::ToHtmlConverter => parse_to_html(input),
        
    }
}