#[derive(Debug, PartialEq)]
pub enum Token {
    Text(String),
    BoldOpen,
    BoldClose,
    ItalicOpen,
    ItalicClose,
    Image(String),
    Escape(char),
    Title(usize, String), // New token type for titles with level and content
}

pub fn tokenize(input: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut lines = input.lines().peekable();
    
    while let Some(line) = lines.next() {
        let mut chars = line.chars().peekable();
        let mut text = String::new();
        let mut bold_open = false;
        let mut italic_open = false;

        if let Some('#') = chars.peek() {
            let mut level = 0;
            while let Some('#') = chars.peek() {
                chars.next();
                level += 1;
            }
            if let Some(&next_ch) = chars.peek() {
                if next_ch.is_whitespace() {
                    chars.next(); // Skip the space after the title marker
                    let mut title_text = String::new();
                    while let Some(next_ch) = chars.next() {
                        title_text.push(next_ch);
                    }
                    tokens.push(Token::Title(level, title_text.trim().to_string()));
                    continue;
                } else {
                    text.push_str(&"#".repeat(level));
                }
            }
        }

        while let Some(ch) = chars.next() {
            match ch {
                '*' => {
                    if let Some('*') = chars.peek() {
                        chars.next(); // Skip the next '*'
                        if !text.is_empty() {
                            tokens.push(Token::Text(text.clone()));
                            text.clear();
                        }
                        if bold_open {
                            tokens.push(Token::BoldClose);
                        } else {
                            tokens.push(Token::BoldOpen);
                        }
                        bold_open = !bold_open;
                    } else {
                        text.push('*');
                    }
                }
                '_' => {
                    if let Some('_') = chars.peek() {
                        chars.next(); // Skip the next '_'
                        if !text.is_empty() {
                            tokens.push(Token::Text(text.clone()));
                            text.clear();
                        }
                        if italic_open {
                            tokens.push(Token::ItalicClose);
                        } else {
                            tokens.push(Token::ItalicOpen);
                        }
                        italic_open = !italic_open;
                    } else {
                        text.push('_');
                    }
                }
                '!' => {
                    if let Some('[') = chars.peek() {
                        chars.next(); // Skip the '['
                        let mut alt_text = String::new();
                        while let Some(next_ch) = chars.next() {
                            if next_ch == ']' {
                                break;
                            }
                            alt_text.push(next_ch);
                        }
                        if let Some('(') = chars.next() {
                            let mut url = String::new();
                            while let Some(next_ch) = chars.next() {
                                if next_ch == ')' {
                                    break;
                                }
                                url.push(next_ch);
                            }
                            tokens.push(Token::Image(url));
                        } else {
                            text.push('!');
                            text.push('[');
                            text.push_str(&alt_text);
                        }
                    } else {
                        text.push('!');
                    }
                }
                '\\' => {
                    if let Some(next_ch) = chars.next() {
                        text.push(ch);
                        text.push(next_ch);
                    }
                }
                _ => {
                    text.push(ch);
                }
            }
        }

        if !text.is_empty() {
            tokens.push(Token::Text(text));
        }
    }

    tokens
}