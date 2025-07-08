/// Lexical analyzer for Redcode assembly language
///
/// This module tokenizes Redcode source code into a stream of tokens
/// for the parser to consume.
use crate::error::{CoreWarError, Result};

/// Token types for Redcode assembly
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TokenType {
    /// Instruction mnemonic (e.g., "live", "ld", "st")
    Instruction,
    /// Register (e.g., "r1", "r16")
    Register,
    /// Direct parameter (e.g., "%42")
    Direct,
    /// Direct label parameter (e.g., "%:label")
    DirectLabel,
    /// Indirect parameter (e.g., "42")
    Indirect,
    /// Label definition (e.g., "loop:")
    Label,
    /// Label reference (e.g., ":loop")
    LabelRef,
    /// Directive (e.g., ".name", ".comment")
    Directive,
    /// String literal (e.g., "Hello World")
    String,
    /// Number literal (e.g., "42", "-100")
    Number,
    /// Comma separator
    Comma,
    /// Newline
    Newline,
    /// Comment (e.g., "# This is a comment")
    Comment,
    /// End of file
    Eof,
}

/// A single token
#[derive(Debug, Clone)]
pub struct Token {
    /// The token type
    pub token_type: TokenType,
    /// The token value/text
    pub value: String,
    /// Line number in source file
    pub line: usize,
    /// Column number in source file
    pub column: usize,
}

impl Token {
    /// Create a new token
    pub fn new(token_type: TokenType, value: String, line: usize, column: usize) -> Self {
        Self {
            token_type,
            value,
            line,
            column,
        }
    }
}

/// Lexical analyzer for Redcode
#[derive(Debug)]
pub struct Lexer {
    /// The source code being tokenized
    source: Vec<char>,
    /// Current position in source
    position: usize,
    /// Current line number
    line: usize,
    /// Current column number
    column: usize,
}

impl Lexer {
    /// Create a new lexer
    ///
    /// # Arguments
    /// * `source` - The source code to tokenize
    ///
    /// # Returns
    /// A new Lexer instance
    pub fn new(source: &str) -> Self {
        Self {
            source: source.chars().collect(),
            position: 0,
            line: 1,
            column: 1,
        }
    }

    /// Tokenize the source code
    ///
    /// # Returns
    /// A vector of tokens, or an error if tokenization failed
    pub fn tokenize(&mut self) -> Result<Vec<Token>> {
        let mut tokens = Vec::new();

        while !self.is_at_end() {
            if let Some(token) = self.next_token()? {
                tokens.push(token);
            }
        }

        tokens.push(Token::new(
            TokenType::Eof,
            String::new(),
            self.line,
            self.column,
        ));
        Ok(tokens)
    }

    /// Get the next token from the source
    fn next_token(&mut self) -> Result<Option<Token>> {
        self.skip_whitespace();

        if self.is_at_end() {
            return Ok(None);
        }

        let start_line = self.line;
        let start_column = self.column;
        let ch = self.advance();

        let token_type_and_value = match ch {
            '\n' => {
                self.line += 1;
                self.column = 1;
                Some((TokenType::Newline, "\n".to_string()))
            }
            ',' => Some((TokenType::Comma, ",".to_string())),
            '#' | ';' => {
                let comment = self.read_comment();
                Some((TokenType::Comment, comment))
            }
            '"' => {
                let string_value = self.read_string()?;
                Some((TokenType::String, string_value))
            }
            '%' => {
                if self.peek() == ':' {
                    self.advance(); // Consume the ':'
                    let label = self.read_identifier()?;
                    Some((TokenType::DirectLabel, format!("%:{}", label)))
                } else {
                    let number = self.read_number()?;
                    Some((TokenType::Direct, format!("%{}", number)))
                }
            }
            ':' => {
                let label = self.read_identifier()?;
                Some((TokenType::LabelRef, format!(":{}", label)))
            }
            '.' => {
                let directive = self.read_identifier()?;
                Some((TokenType::Directive, format!(".{}", directive)))
            }
            _ if ch.is_ascii_alphabetic() || ch == '_' => {
                // Put the character back and read the full identifier
                self.position -= 1;
                self.column -= 1;
                let identifier = self.read_identifier()?;
                self.classify_identifier(identifier)
            }
            _ if ch.is_ascii_digit() || ch == '-' => {
                // Put the character back and read the full number
                self.position -= 1;
                self.column -= 1;
                let number = self.read_number()?;
                Some((TokenType::Indirect, number))
            }
            _ => {
                return Err(CoreWarError::assembler(format!(
                    "Unexpected character '{}' at line {}, column {}",
                    ch, start_line, start_column
                )));
            }
        };

        if let Some((token_type, value)) = token_type_and_value {
            Ok(Some(Token::new(
                token_type,
                value,
                start_line,
                start_column,
            )))
        } else {
            Ok(None)
        }
    }

    /// Skip whitespace characters (except newlines)
    fn skip_whitespace(&mut self) {
        while !self.is_at_end() && self.peek().is_whitespace() && self.peek() != '\n' {
            self.advance();
        }
    }

    /// Read a comment (from # to end of line)
    fn read_comment(&mut self) -> String {
        let mut comment = String::new();
        while !self.is_at_end() && self.peek() != '\n' {
            comment.push(self.advance());
        }
        comment
    }

    /// Read a string literal (from " to ")
    fn read_string(&mut self) -> Result<String> {
        let mut string_value = String::new();

        while !self.is_at_end() && self.peek() != '"' {
            let ch = self.advance();
            if ch == '\\' && !self.is_at_end() {
                // Handle escape sequences
                let escaped = self.advance();
                match escaped {
                    'n' => string_value.push('\n'),
                    't' => string_value.push('\t'),
                    '\\' => string_value.push('\\'),
                    '"' => string_value.push('"'),
                    _ => {
                        string_value.push('\\');
                        string_value.push(escaped);
                    }
                }
            } else {
                string_value.push(ch);
            }
        }

        if self.is_at_end() {
            return Err(CoreWarError::assembler(
                "Unterminated string literal".to_string(),
            ));
        }

        // Consume the closing quote
        self.advance();
        Ok(string_value)
    }

    /// Read an identifier (letters, digits, underscores)
    fn read_identifier(&mut self) -> Result<String> {
        let mut identifier = String::new();

        while !self.is_at_end() {
            let ch = self.peek();
            if ch.is_ascii_alphanumeric() || ch == '_' {
                identifier.push(self.advance());
            } else {
                break;
            }
        }

        // Check for label definition (identifier followed by :)
        if !self.is_at_end() && self.peek() == ':' {
            self.advance(); // consume the ':'
            return Ok(format!("{}:", identifier));
        }

        Ok(identifier)
    }

    /// Read a number (integer)
    fn read_number(&mut self) -> Result<String> {
        let mut number = String::new();

        // Handle negative numbers
        if !self.is_at_end() && self.peek() == '-' {
            number.push(self.advance());
        }

        while !self.is_at_end() && self.peek().is_ascii_digit() {
            number.push(self.advance());
        }

        if number.is_empty() || number == "-" {
            return Err(CoreWarError::assembler("Invalid number format".to_string()));
        }

        Ok(number)
    }

    /// Classify an identifier as instruction, register, or label
    fn classify_identifier(&self, identifier: String) -> Option<(TokenType, String)> {
        // Check if it's a label definition
        if identifier.ends_with(':') {
            return Some((TokenType::Label, identifier));
        }

        // Check if it's a register
        if identifier.starts_with('r') && identifier.len() > 1 {
            if let Ok(_) = identifier[1..].parse::<u8>() {
                return Some((TokenType::Register, identifier));
            }
        }

        // Check if it's an instruction
        let instruction_name = identifier.to_lowercase();
        match instruction_name.as_str() {
            "live" | "ld" | "st" | "add" | "sub" | "and" | "or" | "xor" | "zjmp" | "ldi"
            | "sti" | "fork" | "lld" | "lldi" | "lfork" | "aff" => {
                Some((TokenType::Instruction, instruction_name))
            }
            _ => {
                // Assume it's a label reference if not recognized
                Some((TokenType::LabelRef, identifier))
            }
        }
    }

    /// Check if we've reached the end of the source
    fn is_at_end(&self) -> bool {
        self.position >= self.source.len()
    }

    /// Get the current character without advancing
    fn peek(&self) -> char {
        if self.is_at_end() {
            '\0'
        } else {
            self.source[self.position]
        }
    }

    /// Get the current character and advance the position
    fn advance(&mut self) -> char {
        if self.is_at_end() {
            '\0'
        } else {
            let ch = self.source[self.position];
            self.position += 1;
            self.column += 1;
            ch
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_tokenization() {
        let mut lexer = Lexer::new("live %1");
        let tokens = lexer.tokenize().unwrap();

        assert_eq!(tokens.len(), 3); // live, %1, EOF
        assert_eq!(tokens[0].token_type, TokenType::Instruction);
        assert_eq!(tokens[0].value, "live");
        assert_eq!(tokens[1].token_type, TokenType::Direct);
        assert_eq!(tokens[1].value, "%1");
    }

    #[test]
    fn test_register_tokenization() {
        let mut lexer = Lexer::new("r1 r16");
        let tokens = lexer.tokenize().unwrap();

        assert_eq!(tokens[0].token_type, TokenType::Register);
        assert_eq!(tokens[0].value, "r1");
        assert_eq!(tokens[1].token_type, TokenType::Register);
        assert_eq!(tokens[1].value, "r16");
    }

    #[test]
    fn test_label_tokenization() {
        let mut lexer = Lexer::new("loop: zjmp :loop");
        let tokens = lexer.tokenize().unwrap();

        assert_eq!(tokens[0].token_type, TokenType::Label);
        assert_eq!(tokens[0].value, "loop:");
        assert_eq!(tokens[1].token_type, TokenType::Instruction);
        assert_eq!(tokens[1].value, "zjmp");
        assert_eq!(tokens[2].token_type, TokenType::LabelRef);
        assert_eq!(tokens[2].value, ":loop");
    }

    #[test]
    fn test_directive_tokenization() {
        let mut lexer = Lexer::new(".name \"test\"");
        let tokens = lexer.tokenize().unwrap();

        assert_eq!(tokens[0].token_type, TokenType::Directive);
        assert_eq!(tokens[0].value, ".name");
        assert_eq!(tokens[1].token_type, TokenType::String);
        assert_eq!(tokens[1].value, "test");
    }
}
