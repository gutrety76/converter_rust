use converter::parser::parser::{parse, ParserType};
use converter::tokenizer::tokenizer::{self, Token};
fn main() {
    let input = r#"# Title 1
## Title 2
### Title 3
#### Title 4
##### Title 5
###### Title 6
![Image](http://url/a.png)
**bold**
__italic__"#;
    let tokens: Vec<Token> = tokenizer::tokenize(input);
    let parsed_text_to_html = parse(tokens, ParserType::ToHtmlConverter);
    println!("{}", parsed_text_to_html);
}
