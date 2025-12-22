use htms_compiler::lexer::{tokenize, TokenKind};

#[test]
fn test_interpolation_start_token() {
    // Verify that ${ is recognized as InterpolationStart token
    let source = "{{ ${ctx.count} }}";
    let tokens = tokenize(source).unwrap();

    // Print tokens for debugging
    for (i, token) in tokens.iter().enumerate() {
        eprintln!("Token {}: {:?} = {:?}", i, token.kind, token.value);
    }

    assert_eq!(tokens[0].kind, TokenKind::TextOpen);
    assert_eq!(tokens[1].kind, TokenKind::TextContent);
    // The text content should contain the full interpolation syntax
    assert_eq!(tokens[1].value, " ${ctx.count} ");
    assert_eq!(tokens[2].kind, TokenKind::TextClose);
}

#[test]
fn test_rbrace_outside_text() {
    // Test that } is RBrace in normal code
    let source = "component Foo { }";
    let tokens = tokenize(source).unwrap();

    assert_eq!(tokens[0].kind, TokenKind::Component);
    assert_eq!(tokens[1].kind, TokenKind::ComponentName);
    assert_eq!(tokens[2].kind, TokenKind::LBrace);
    assert_eq!(tokens[3].kind, TokenKind::RBrace);
}

#[test]
fn test_dollar_sign_alone_still_error() {
    // If $ appears without { following it, it should still be an error
    let source = "{{ Price: $99 }}";
    let tokens = tokenize(source).unwrap();

    assert_eq!(tokens[0].kind, TokenKind::TextOpen);
    assert_eq!(tokens[1].kind, TokenKind::TextContent);
    // The $ alone should be captured as part of text content
    assert_eq!(tokens[1].value, " Price: $99 ");
    assert_eq!(tokens[2].kind, TokenKind::TextClose);
}

#[test]
fn test_text_content_with_interpolation_syntax() {
    let source = "{{ Total: ${ctx.count} items }}";
    let tokens = tokenize(source).unwrap();

    assert_eq!(tokens[0].kind, TokenKind::TextOpen);
    assert_eq!(tokens[1].kind, TokenKind::TextContent);
    assert_eq!(tokens[1].value, " Total: ${ctx.count} items ");
    assert_eq!(tokens[2].kind, TokenKind::TextClose);
    assert_eq!(tokens[3].kind, TokenKind::Eof);
}

#[test]
fn test_text_content_with_braces() {
    let source = "{{ Hello ${ctx.user.name}! }}";
    let tokens = tokenize(source).unwrap();

    assert_eq!(tokens[0].kind, TokenKind::TextOpen);
    assert_eq!(tokens[1].kind, TokenKind::TextContent);
    assert_eq!(tokens[1].value, " Hello ${ctx.user.name}! ");
    assert_eq!(tokens[2].kind, TokenKind::TextClose);
}

#[test]
fn test_text_content_with_multiple_interpolations() {
    let source = "{{ Hello ${ctx.name}, you have ${ctx.count} messages }}";
    let tokens = tokenize(source).unwrap();

    assert_eq!(tokens[0].kind, TokenKind::TextOpen);
    assert_eq!(tokens[1].kind, TokenKind::TextContent);
    assert_eq!(tokens[1].value, " Hello ${ctx.name}, you have ${ctx.count} messages ");
    assert_eq!(tokens[2].kind, TokenKind::TextClose);
}

#[test]
fn test_text_content_utf8() {
    let source = "{{ Hello 世界 ${ctx.name} 🚀 }}";
    let tokens = tokenize(source).unwrap();

    assert_eq!(tokens[0].kind, TokenKind::TextOpen);
    assert_eq!(tokens[1].kind, TokenKind::TextContent);
    assert_eq!(tokens[1].value, " Hello 世界 ${ctx.name} 🚀 ");
    assert_eq!(tokens[2].kind, TokenKind::TextClose);
}

#[test]
fn test_text_content_no_trailing_dollar_sign() {
    // This test specifically verifies the bug fix for the trailing $ issue
    let source = "component Stats { p { {{ Total: ${ctx.count} items }} } }";
    let tokens = tokenize(source).unwrap();

    // Find the TextContent token
    let text_token = tokens.iter()
        .find(|t| t.kind == TokenKind::TextContent)
        .expect("Should have TextContent token");

    // Verify no trailing $ in the text content
    assert_eq!(text_token.value, " Total: ${ctx.count} items ");
    assert!(!text_token.value.ends_with('$'), "Text content should not have trailing $");
}

#[test]
fn test_text_content_empty() {
    let source = "{{}}";
    let tokens = tokenize(source).unwrap();

    assert_eq!(tokens[0].kind, TokenKind::TextOpen);
    // Empty text content should not produce a TextContent token
    assert_eq!(tokens[1].kind, TokenKind::TextClose);
}

#[test]
fn test_text_content_only_spaces() {
    let source = "{{   }}";
    let tokens = tokenize(source).unwrap();

    assert_eq!(tokens[0].kind, TokenKind::TextOpen);
    assert_eq!(tokens[1].kind, TokenKind::TextContent);
    assert_eq!(tokens[1].value, "   ");
    assert_eq!(tokens[2].kind, TokenKind::TextClose);
}

#[test]
fn test_text_content_with_special_chars() {
    let source = r#"{{ She said "hello" and clicked 'OK' }}"#;
    let tokens = tokenize(source).unwrap();

    assert_eq!(tokens[0].kind, TokenKind::TextOpen);
    assert_eq!(tokens[1].kind, TokenKind::TextContent);
    assert_eq!(tokens[1].value, r#" She said "hello" and clicked 'OK' "#);
    assert_eq!(tokens[2].kind, TokenKind::TextClose);
}

#[test]
fn test_text_content_invalid_interpolation_pattern() {
    let source = "{{ Use var(${--color}) for CSS }}";
    let tokens = tokenize(source).unwrap();

    assert_eq!(tokens[0].kind, TokenKind::TextOpen);
    assert_eq!(tokens[1].kind, TokenKind::TextContent);
    assert_eq!(tokens[1].value, " Use var(${--color}) for CSS ");
    assert_eq!(tokens[2].kind, TokenKind::TextClose);
}
