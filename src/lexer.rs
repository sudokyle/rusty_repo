const KEYWORDS: [TokenType; 5] = [TokenType::IF, TokenType::DO, TokenType::WHILE, TokenType::THEN, TokenType::PRINT];

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum TokenType {
    NIL,
    EndOfFile,
    IF,
    WHILE,
    DO,
    THEN,
    PRINT,
    PLUS,
    MINUS,
    DIV,
    MULT,
    EQUAL,
    COLON,
    COMMA,
    SEMICOLON,
    LBRAC,
    RBRAC,
    LPAREN,
    RPAREN,
    NOTEQUAL,
    GREATER,
    LESS,
    LTEQ,
    GTEQ,
    DOT,
    NUM,
    ID,
    ERROR
}

pub struct Lexer {
    token: Token,
    data: String,
    current_index: usize,
    active_token: bool,
}

#[derive(Debug)]
pub struct Token {
    pub value: String,
    pub token_type: TokenType
}

// ============ ⬇ Implementations ⬇ ============ \\

impl Token {
    pub fn new(value: &str, token_type: TokenType) -> Self {
        return Token{value: String::from(value), token_type};
    }
    pub fn default() -> Self {
        return Token{value: String::new(), token_type: TokenType::NIL}
    }
}

impl Clone for Token {
    fn clone(&self) -> Self {
        Token{value: self.value.clone(), token_type: self.token_type.clone()}
    }
}

impl Lexer {
    pub fn new(data: &str) -> Self {
        Lexer{data: String::from(data), token: Token::default(), current_index: 0, active_token: true}
    }

    /// Starting from [current_index], skips space characters.
    /// returns index of first non-space character
    fn skip_space(data: &str, current_index: usize) -> usize {
        for (index, char) in data[current_index..].char_indices() {
            if !char.is_whitespace() {
                return current_index + index;
            }
        }
        return data.len()-1;
    }

    /// Determines if [token_type] is a keyword in the reserved terms.
    /// returns true if it is, false otherwise.
    fn is_keyword(token_type: &TokenType) -> bool {
        return KEYWORDS.contains(token_type);
    }

    /// Get the Keyword [TokenType] that corresponds with [data].
    /// If [data] isn't a [KEYWORDS] [TokenType], [TokenType::ERROR] is returned.
    fn keyword_from(data:&str) -> TokenType {
        match data {
            "if" => TokenType::IF,
            "do" => TokenType::DO,
            "while" => TokenType::WHILE,
            "then" => TokenType::THEN,
            "print" => TokenType::PRINT,
            _ => TokenType::ERROR,
        }
    }

    /// Starting at [current_index] iterates over [data] to build a [TokenType::NUM] token.
    /// returns the first non-NUM character index and the NUM token it built.
    fn scan_id_keyword(data: &str, current_index: usize) -> (usize, Token) {
        let chars: Vec<char> = data.chars().collect();
        let mut next_index = current_index;
        let mut char = chars[next_index];
        let mut token = String::new();
        if char.is_alphabetic() {
            while char.is_alphanumeric() {
                token.push(char);
                next_index += 1;
                char = chars[next_index];
            }
            let token_type = Lexer::keyword_from(&token);
            return if Lexer::is_keyword(&token_type) {
                (next_index, Token::new(&token, token_type))
            } else {
                (next_index, Token::new(&token, TokenType::ID))
            }
        }
        return (next_index, Token::new("", TokenType::ERROR));
    }

    /// Starting at [current_index] iterates over [data] to build a token.
    ///
    /// If the built token value isn't a [KEYWORDS] [TokenType], then then returned Token is of [TokenType::ID],
    /// otherwise the token type of the returned token is the corresponding keywords token type.
    ///
    /// returns the first non-ID/KEYWORD character index and the ID/KEYWORD token it built.
    fn scan_number(data: &str, current_index: usize) -> (usize, Token) {
        let chars: Vec<char> = data[current_index..].chars().collect();
        let mut return_string = String::from(chars[0]);
        let mut next_index = 1;
        while next_index < chars.len() {
            let char = chars[next_index];
            if char.is_digit(10) {
                return_string.push(char);
            } else {
                break;
            }
            next_index += 1;
        }
        return (current_index + next_index, Token::new(&return_string, TokenType::NUM));
    }

    /// Causes the next call of [get_token] to return the same token that it returned in its
    /// last call.
    ///
    /// Example:
    /// "one two three four five"
    /// get_token() => "one"
    /// get_token() => "two"
    /// unget_token();
    /// get_token() => "two"
    /// get_token() => "three"
    pub fn unget_token(&mut self) {
        self.active_token = false;
    }

    /// Returns the next token in [self.data].
    ///
    /// If [Lexer::unget_token] was called before, then last token found is returned and [self.active_token] is reset
    /// to true.
    pub fn get_token(&mut self) -> Token {
        if !self.active_token {
            self.active_token = true;
            return self.token.clone();
        }
        let chars: Vec<char> = self.data.chars().collect();

        if self.current_index == chars.len() {
            self.token = Token::new("", TokenType::EndOfFile);
            return self.token.clone();
        }

        self.current_index = Lexer::skip_space(&self.data, self.current_index);
        let char = chars[self.current_index];
        self.current_index += 1;
        match char {
            '.' => self.token = Token::new(".", TokenType::DOT),
            '+' => self.token = Token::new("+", TokenType::PLUS),
            '-' => self.token = Token::new("-", TokenType::MINUS),
            '/' => self.token = Token::new("/", TokenType::DIV),
            '*' => self.token = Token::new("*", TokenType::MULT),
            '=' => self.token = Token::new("=", TokenType::EQUAL),
            ':' => self.token = Token::new(":", TokenType::COLON),
            ',' => self.token = Token::new(",", TokenType::COMMA),
            ';' => self.token = Token::new(";", TokenType::SEMICOLON),
            '{' => self.token = Token::new("{", TokenType::LBRAC),
            '}' => self.token = Token::new("}", TokenType::RBRAC),
            '(' => self.token = Token::new("(", TokenType::LPAREN),
            ')' => self.token = Token::new(")", TokenType::RPAREN),
            '<' => {
                let next_char = chars[self.current_index];
                if next_char == '=' {
                    self.token = Token::new("<=", TokenType::LTEQ);
                    self.current_index += 1;
                } else if next_char == '>' {
                    self.token = Token::new("<>", TokenType::NOTEQUAL);
                    self.current_index += 1;
                } else {
                    self.token = Token::new("<", TokenType::LESS);
                }
            }
            '>' => {
                let next_char = chars[self.current_index];
                if next_char == '=' {
                    self.token = Token::new(">=", TokenType::GTEQ);
                    self.current_index += 1;
                } else {
                    self.token = Token::new(">", TokenType::GREATER);
                }
            }
            _ => {
                if char.is_digit(10) {
                    // subtract current_index by one since we incremented it after getting char, want to ensure char is part of the token returned.
                    let (next_index, token) = Lexer::scan_number(&self.data, self.current_index-1);
                    self.current_index = next_index;
                    self.token = token;
                } else if char.is_alphanumeric() {
                    // subtract current_index by one since we incremented it after getting char, want to ensure char is part of the token returned.
                    let (next_index, token) = Lexer::scan_id_keyword(&self.data, self.current_index-1);
                    self.current_index = next_index;
                    self.token = token;
                } else {
                    self.token = Token::new("", TokenType::ERROR);
                }
            }
        }

        return self.token.clone();
    }
}