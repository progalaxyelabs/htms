//! Scanner implementation using Logos

use logos::Logos;
use crate::Location;
use crate::error::LexerError;
use super::tokens::{Token, TokenKind};

/// Tokenize HTMS source code
pub fn tokenize(source: &str) -> Result<Vec<Token>, Vec<LexerError>> {
    let mut tokens = Vec::new();
    let mut errors = Vec::new();
    let mut lexer = TokenKind::lexer(source);

    let mut line = 1;
    let mut line_start = 0;
    let mut in_text_content = false;
    let mut text_start = 0;
    let mut text_content = String::new();

    while let Some(result) = lexer.next() {
        let span = lexer.span();
        let slice = lexer.slice();

        // Calculate column
        let column = span.start - line_start + 1;

        let location = Location {
            line,
            column,
            start: span.start,
            end: span.end,
        };

        match result {
            Ok(kind) => {
                // Handle text content mode
                if in_text_content {
                    if kind == TokenKind::TextClose {
                        // End text content
                        if !text_content.is_empty() {
                            tokens.push(Token {
                                kind: TokenKind::TextContent,
                                value: text_content.clone(),
                                location: Location {
                                    line,
                                    column: text_start - line_start + 1,
                                    start: text_start,
                                    end: span.start,
                                },
                            });
                            text_content.clear();
                        }
                        tokens.push(Token {
                            kind: TokenKind::TextClose,
                            value: "}}".to_string(),
                            location,
                        });
                        in_text_content = false;
                    } else {
                        // Text content already captured, skip tokens until }}
                        if kind == TokenKind::Newline {
                            line += 1;
                            line_start = span.end;
                        }
                    }
                    continue;
                }

                // Handle special tokens
                match kind {
                    TokenKind::TextOpen => {
                        tokens.push(Token {
                            kind: TokenKind::TextOpen,
                            value: "{{".to_string(),
                            location,
                        });
                        in_text_content = true;
                        text_start = span.end;

                        // Manually capture text content until we find }}
                        let mut content_end = text_start;
                        let source_bytes = source.as_bytes();
                        while content_end < source_bytes.len() - 1 {
                            if &source_bytes[content_end..content_end+2] == b"}}" {
                                break;
                            }
                            content_end += 1;
                        }

                        if content_end < source_bytes.len() - 1 {
                            text_content = source[text_start..content_end].to_string();
                        }
                    }
                    TokenKind::Newline => {
                        line += 1;
                        line_start = span.end;
                        // Don't add newline tokens
                    }
                    TokenKind::LineComment | TokenKind::BlockComment => {
                        // Count newlines in block comments
                        if kind == TokenKind::BlockComment {
                            for c in slice.chars() {
                                if c == '\n' {
                                    line += 1;
                                }
                            }
                            // Update line_start to after last newline
                            if let Some(last_newline) = slice.rfind('\n') {
                                line_start = span.start + last_newline + 1;
                            }
                        }
                        // Don't add comment tokens
                    }
                    TokenKind::String => {
                        // Remove quotes from string value
                        let value = &slice[1..slice.len() - 1];
                        tokens.push(Token {
                            kind,
                            value: value.to_string(),
                            location,
                        });
                    }
                    _ => {
                        tokens.push(Token {
                            kind,
                            value: slice.to_string(),
                            location,
                        });
                    }
                }
            }
            Err(()) => {
                // Handle text content mode for errors
                if in_text_content {
                    text_content.push_str(slice);
                    if slice == "\n" {
                        line += 1;
                        line_start = span.end;
                    }
                } else {
                    errors.push(LexerError::new(
                        format!("Unexpected character: '{}'", slice),
                        location,
                    ));
                }
            }
        }
    }

    // Check for unclosed text content
    if in_text_content {
        errors.push(LexerError::new(
            "Unterminated text content: missing '}}'",
            Location {
                line,
                column: text_start - line_start + 1,
                start: text_start,
                end: source.len(),
            },
        ));
    }

    // Add EOF token
    tokens.push(Token {
        kind: TokenKind::Eof,
        value: String::new(),
        location: Location {
            line,
            column: source.len() - line_start + 1,
            start: source.len(),
            end: source.len(),
        },
    });

    if errors.is_empty() {
        Ok(tokens)
    } else {
        Err(errors)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_tokens() {
        let source = "component NavBar { }";
        let tokens = tokenize(source).unwrap();

        assert_eq!(tokens[0].kind, TokenKind::Component);
        assert_eq!(tokens[1].kind, TokenKind::ComponentName);
        assert_eq!(tokens[1].value, "NavBar");
        assert_eq!(tokens[2].kind, TokenKind::LBrace);
        assert_eq!(tokens[3].kind, TokenKind::RBrace);
        assert_eq!(tokens[4].kind, TokenKind::Eof);
    }

    #[test]
    fn test_attributes() {
        let source = r#"div [class: "container", id: "main"]"#;
        let tokens = tokenize(source).unwrap();

        assert_eq!(tokens[0].kind, TokenKind::Identifier);
        assert_eq!(tokens[0].value, "div");
        assert_eq!(tokens[1].kind, TokenKind::LBracket);
        assert_eq!(tokens[2].kind, TokenKind::Identifier);
        assert_eq!(tokens[2].value, "class");
        assert_eq!(tokens[3].kind, TokenKind::Colon);
        assert_eq!(tokens[4].kind, TokenKind::String);
        assert_eq!(tokens[4].value, "container");
    }

    #[test]
    fn test_text_content() {
        let source = r#"{{ Hello "world" }}"#;
        let tokens = tokenize(source).unwrap();

        assert_eq!(tokens[0].kind, TokenKind::TextOpen);
        assert_eq!(tokens[1].kind, TokenKind::TextContent);
        assert_eq!(tokens[1].value, r#" Hello "world" "#);
        assert_eq!(tokens[2].kind, TokenKind::TextClose);
    }

    #[test]
    fn test_context_path() {
        let source = "ctx.user.name";
        let tokens = tokenize(source).unwrap();

        assert_eq!(tokens[0].kind, TokenKind::ContextPath);
        assert_eq!(tokens[0].value, "ctx.user.name");
    }

    #[test]
    fn test_directives() {
        let source = "@if @else @each @slot";
        let tokens = tokenize(source).unwrap();

        assert_eq!(tokens[0].kind, TokenKind::If);
        assert_eq!(tokens[1].kind, TokenKind::Else);
        assert_eq!(tokens[2].kind, TokenKind::Each);
        assert_eq!(tokens[3].kind, TokenKind::Slot);
    }

    #[test]
    fn test_line_numbers() {
        let source = "component\nNavBar\n{\n}";
        let tokens = tokenize(source).unwrap();

        assert_eq!(tokens[0].location.line, 1);
        assert_eq!(tokens[1].location.line, 2);
        assert_eq!(tokens[2].location.line, 3);
        assert_eq!(tokens[3].location.line, 4);
    }

    #[test]
    fn test_comments_ignored() {
        let source = "// comment\ncomponent /* inline */ NavBar";
        let tokens = tokenize(source).unwrap();

        assert_eq!(tokens[0].kind, TokenKind::Component);
        assert_eq!(tokens[1].kind, TokenKind::ComponentName);
    }
}
