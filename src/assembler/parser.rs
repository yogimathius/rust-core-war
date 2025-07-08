use crate::assembler::lexer::{Token, TokenType};
use crate::assembler::{AstNode, InstructionNode, ParameterNode, ProgramHeader};
/// Parser for Redcode assembly language
///
/// This module parses a stream of tokens into an Abstract Syntax Tree (AST)
/// representing the structure of a Redcode program.
use crate::error::{CoreWarError, Result};

/// Parser for Redcode assembly
#[derive(Debug)]
pub struct Parser {
    /// The tokens to parse
    tokens: Vec<Token>,
    /// Current position in token stream
    current: usize,
    /// Pending label for the next instruction
    pending_label: Option<String>,
}

impl Parser {
    /// Create a new parser
    ///
    /// # Arguments
    /// * `tokens` - The tokens to parse
    ///
    /// # Returns
    /// A new Parser instance
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens,
            current: 0,
            pending_label: None,
        }
    }

    /// Parse the tokens into an AST
    ///
    /// # Returns
    /// The parsed AST, or an error if parsing failed
    pub fn parse(&mut self) -> Result<AstNode> {
        let header = self.parse_header()?;
        let instructions = self.parse_instructions()?;

        Ok(AstNode {
            header,
            instructions,
        })
    }

    /// Parse the program header (.name and .comment directives)
    fn parse_header(&mut self) -> Result<ProgramHeader> {
        let mut name = String::new();
        let mut comment = String::new();

        // Skip any initial newlines and comments
        self.skip_newlines_and_comments();

        // Parse header directives
        while !self.is_at_end() && self.peek().token_type == TokenType::Directive {
            let directive = self.advance();

            match directive.value.as_str() {
                ".name" => {
                    if self.peek().token_type == TokenType::String {
                        name = self.advance().value;
                    } else {
                        return Err(CoreWarError::assembler(format!(
                            "Expected string after .name directive at line {}",
                            directive.line
                        )));
                    }
                }
                ".comment" => {
                    if self.peek().token_type == TokenType::String {
                        comment = self.advance().value;
                    } else {
                        return Err(CoreWarError::assembler(format!(
                            "Expected string after .comment directive at line {}",
                            directive.line
                        )));
                    }
                }
                _ => {
                    return Err(CoreWarError::assembler(format!(
                        "Unknown directive '{}' at line {}",
                        directive.value, directive.line
                    )));
                }
            }

            self.skip_newlines_and_comments();
        }

        if name.is_empty() {
            return Err(CoreWarError::assembler(
                ".name directive is required".to_string(),
            ));
        }

        Ok(ProgramHeader { name, comment })
    }

    /// Parse the program instructions
    fn parse_instructions(&mut self) -> Result<Vec<InstructionNode>> {
        let mut instructions = Vec::new();

        while !self.is_at_end() {
            let current_position = self.current;
            self.skip_newlines();

            if self.is_at_end() {
                break;
            }

            // Skip comments
            if self.peek().token_type == TokenType::Comment {
                self.advance();
                continue;
            }

            if let Some(instruction) = self.parse_instruction()? {
                instructions.push(instruction);
            }

            if !self.is_at_end() && self.peek().token_type == TokenType::Newline {
                self.advance();
            }

            // Guard against infinite loops
            if self.current == current_position {
                // If we haven't advanced, force advance to prevent infinite loop
                if !self.is_at_end() {
                    self.advance();
                }
            }
        }

        Ok(instructions)
    }

    /// Parse a single instruction
    fn parse_instruction(&mut self) -> Result<Option<InstructionNode>> {
        let mut label = self.pending_label.take(); // Take any pending label
        let line_number = self.peek().line;

        // Check for optional label
        if self.peek().token_type == TokenType::Label {
            let label_token = self.advance();
            label = Some(label_token.value.trim_end_matches(':').to_string());

            // Skip newlines after label
            self.skip_newlines();
        }

        // If there's no instruction after the label, save the label for next instruction
        if self.peek().token_type != TokenType::Instruction {
            if label.is_some() {
                // We have a label but no instruction follows, save the label for next instruction
                self.pending_label = label;
                return Ok(None);
            } else {
                // Not a label and not an instruction, skip this token
                return Ok(None);
            }
        }

        let mnemonic = self.advance().value;

        // Parse parameters
        let mut parameters = Vec::new();

        while !self.is_at_end()
            && self.peek().token_type != TokenType::Newline
            && self.peek().token_type != TokenType::Comment
            && self.peek().token_type != TokenType::Eof
        {
            // Skip commas
            if self.peek().token_type == TokenType::Comma {
                self.advance();
                continue;
            }

            let parameter = self.parse_parameter()?;
            parameters.push(parameter);
        }

        Ok(Some(InstructionNode {
            label,
            mnemonic,
            parameters,
            line_number,
        }))
    }

    /// Parse a single parameter
    fn parse_parameter(&mut self) -> Result<ParameterNode> {
        let token = self.advance();

        let (param_type, value) = match token.token_type {
            TokenType::Register => ("register".to_string(), token.value),
            TokenType::Direct => (
                "direct".to_string(),
                token.value.trim_start_matches('%').to_string(),
            ),
            TokenType::DirectLabel => (
                "label".to_string(),
                token.value.trim_start_matches("%:").to_string(),
            ),
            TokenType::Indirect => ("indirect".to_string(), token.value),
            TokenType::LabelRef => (
                "label".to_string(),
                token.value.trim_start_matches(':').to_string(),
            ),
            _ => {
                return Err(CoreWarError::assembler(format!(
                    "Invalid parameter type '{}' at line {}",
                    token.value, token.line
                )));
            }
        };

        Ok(ParameterNode { param_type, value })
    }

    /// Skip newline tokens
    fn skip_newlines(&mut self) {
        while !self.is_at_end() && self.peek().token_type == TokenType::Newline {
            self.advance();
        }
    }

    /// Skip newline and comment tokens
    fn skip_newlines_and_comments(&mut self) {
        while !self.is_at_end()
            && (self.peek().token_type == TokenType::Newline
                || self.peek().token_type == TokenType::Comment)
        {
            self.advance();
        }
    }

    /// Check if we've reached the end of the token stream
    fn is_at_end(&self) -> bool {
        self.current >= self.tokens.len() || self.peek().token_type == TokenType::Eof
    }

    /// Get the current token without advancing
    fn peek(&self) -> &Token {
        if self.current >= self.tokens.len() {
            &self.tokens[self.tokens.len() - 1] // Return EOF token
        } else {
            &self.tokens[self.current]
        }
    }

    /// Get the current token and advance the position
    fn advance(&mut self) -> Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.tokens[self.current - 1].clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::assembler::lexer::Lexer;

    #[test]
    fn test_header_parsing() {
        let source = r#"
            .name "test"
            .comment "A test program"
        "#;

        let mut lexer = Lexer::new(source);
        let tokens = lexer.tokenize().unwrap();
        let mut parser = Parser::new(tokens);

        let header = parser.parse_header().unwrap();
        assert_eq!(header.name, "test");
        assert_eq!(header.comment, "A test program");
    }

    #[test]
    fn test_instruction_parsing() {
        let source = r#"
            .name "test"
            
            loop:   live %1
                    ld %0, r1
                    st r1, :loop
        "#;

        let mut lexer = Lexer::new(source);
        let tokens = lexer.tokenize().unwrap();
        let mut parser = Parser::new(tokens);

        let ast = parser.parse().unwrap();
        assert_eq!(ast.instructions.len(), 3);

        // Check first instruction (with label)
        assert_eq!(ast.instructions[0].label, Some("loop".to_string()));
        assert_eq!(ast.instructions[0].mnemonic, "live");
        assert_eq!(ast.instructions[0].parameters.len(), 1);
        assert_eq!(ast.instructions[0].parameters[0].param_type, "direct");
        assert_eq!(ast.instructions[0].parameters[0].value, "1");

        // Check second instruction
        assert_eq!(ast.instructions[1].mnemonic, "ld");
        assert_eq!(ast.instructions[1].parameters.len(), 2);

        // Check third instruction
        assert_eq!(ast.instructions[2].mnemonic, "st");
        assert_eq!(ast.instructions[2].parameters[1].param_type, "label");
        assert_eq!(ast.instructions[2].parameters[1].value, "loop");
    }

    #[test]
    fn test_missing_name_directive() {
        let source = "live %1";

        let mut lexer = Lexer::new(source);
        let tokens = lexer.tokenize().unwrap();
        let mut parser = Parser::new(tokens);

        // Should fail because .name is required
        assert!(parser.parse().is_err());
    }
}
