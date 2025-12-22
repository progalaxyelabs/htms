use htms_compiler::*;

#[test]
fn test_times_symbol_end_to_end() {
    // Test that × character doesn't get duplicated in the generated code
    let source = r#"
component TodoItem {
  button [class: "delete-btn"] {{ × }}
}
"#;

    let tokens = lexer::tokenize(source).unwrap();
    let ast = parser::parse(&tokens).unwrap();
    let (symbols, _warnings) = analyzer::analyze(&ast);

    let options = CompileOptions {
        generate_router: false,
        generate_events: false,
    };

    let files = codegen::generate(&ast, &symbols, &options);
    let templates = &files[0];

    // Print generated code for debugging
    eprintln!("Generated code:\n{}", templates.content);

    // Verify the × character appears exactly once in the generated code
    let times_count = templates.content.matches("×").count();
    assert_eq!(times_count, 1, "× character should appear exactly once, but found {} occurrences", times_count);

    // Verify the specific line contains the correct text
    assert!(templates.content.contains("createTextNode('×')"),
            "Generated code should contain createTextNode('×'), not duplicated");

    // Make sure it doesn't contain '× ×'
    assert!(!templates.content.contains("× ×"),
            "Generated code should not contain duplicated times symbol");
}

#[test]
fn test_add_btn_text_end_to_end() {
    // Test that regular button text works correctly
    let source = r#"
page home "/" {
  button [id: "addBtn"] {{ Add Todo }}
}
"#;

    let tokens = lexer::tokenize(source).unwrap();
    let ast = parser::parse(&tokens).unwrap();
    let (symbols, _warnings) = analyzer::analyze(&ast);

    let options = CompileOptions {
        generate_router: false,
        generate_events: false,
    };

    let files = codegen::generate(&ast, &symbols, &options);
    let templates = &files[0];

    // Verify the button text appears correctly
    assert!(templates.content.contains("createTextNode('Add Todo')"),
            "Generated code should contain createTextNode('Add Todo')");

    // Make sure it's not duplicated
    assert!(!templates.content.contains("Add Todo Add Todo"),
            "Button text should not be duplicated");
}

#[test]
fn test_user_app_htms_reproduction() {
    // Reproduce the exact issue from the user's app.htms file
    let source = r#"component TodoItem {
  div [class: "todo-item"] {
    input [type: "checkbox"]
    span [class: "todo-title"] {{ Todo title }}
    span [class: "todo-date"] {{ Date }}
    button [class: "delete-btn"] {{ × }}
  }
}"#;

    let tokens = lexer::tokenize(source).unwrap();

    // Debug: print tokens
    eprintln!("Tokens:");
    for (i, token) in tokens.iter().enumerate() {
        eprintln!("  {}: {:?} = {:?}", i, token.kind, token.value);
    }

    let ast = parser::parse(&tokens).unwrap();
    let (symbols, _warnings) = analyzer::analyze(&ast);

    let options = CompileOptions {
        generate_router: false,
        generate_events: false,
    };

    let files = codegen::generate(&ast, &symbols, &options);
    let templates = &files[0];

    eprintln!("\nGenerated code:\n{}", templates.content);

    // Verify the × character appears exactly once
    let times_count = templates.content.matches("×").count();
    assert_eq!(times_count, 1, "× character should appear exactly once, but found {} occurrences", times_count);

    // Make sure it doesn't contain '× ×'
    assert!(!templates.content.contains("× ×"),
            "Generated code should not contain duplicated times symbol '× ×'");
}
