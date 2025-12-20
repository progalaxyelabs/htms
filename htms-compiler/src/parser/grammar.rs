//! Recursive descent parser for HTMS

use crate::ast::*;
use crate::error::ParseError;
use crate::lexer::{Token, TokenKind};
use crate::Location;

/// Parser state
pub struct Parser<'a> {
    tokens: &'a [Token],
    current: usize,
    errors: Vec<ParseError>,
}

impl<'a> Parser<'a> {
    pub fn new(tokens: &'a [Token]) -> Self {
        Self {
            tokens,
            current: 0,
            errors: Vec::new(),
        }
    }

    /// Parse the entire program
    pub fn parse(mut self) -> Result<Program, Vec<ParseError>> {
        let start_loc = self.current_location();
        let mut body = Vec::new();

        while !self.is_at_end() {
            match self.declaration() {
                Ok(decl) => body.push(decl),
                Err(e) => {
                    self.errors.push(e);
                    self.synchronize();
                }
            }
        }

        if self.errors.is_empty() {
            Ok(Program {
                body,
                loc: Location {
                    line: start_loc.line,
                    column: start_loc.column,
                    start: start_loc.start,
                    end: self.current_location().end,
                },
            })
        } else {
            Err(self.errors)
        }
    }

    // =========================================================================
    // Declarations
    // =========================================================================

    fn declaration(&mut self) -> Result<Declaration, ParseError> {
        if self.check(TokenKind::Component) {
            self.component_decl().map(Declaration::Component)
        } else if self.check(TokenKind::Section) {
            self.section_decl().map(Declaration::Section)
        } else if self.check(TokenKind::Page) {
            self.page_decl().map(Declaration::Page)
        } else {
            Err(self.error("Expected 'component', 'section', or 'page'"))
        }
    }

    fn component_decl(&mut self) -> Result<ComponentDecl, ParseError> {
        let start = self.current_location();
        self.consume(TokenKind::Component, "Expected 'component'")?;

        let name = self.consume(TokenKind::ComponentName, "Expected component name")?;
        let name = name.value.clone();

        // Optional parameters: (item: user)
        let parameters = if self.check(TokenKind::LParen) {
            self.parameter_list()?
        } else {
            Vec::new()
        };

        // Optional attributes: [class: "foo"]
        let attributes = if self.check(TokenKind::LBracket) {
            self.attribute_list()?
        } else {
            Vec::new()
        };

        // Body
        let body = self.block()?;

        Ok(ComponentDecl {
            name,
            parameters,
            attributes,
            body,
            loc: self.location_from(start),
        })
    }

    fn section_decl(&mut self) -> Result<SectionDecl, ParseError> {
        let start = self.current_location();
        self.consume(TokenKind::Section, "Expected 'section'")?;

        let name = self.consume(TokenKind::ComponentName, "Expected section name")?;
        let name = name.value.clone();

        let body = self.block()?;

        Ok(SectionDecl {
            name,
            body,
            loc: self.location_from(start),
        })
    }

    fn page_decl(&mut self) -> Result<PageDecl, ParseError> {
        let start = self.current_location();
        self.consume(TokenKind::Page, "Expected 'page'")?;

        let name = self.consume(TokenKind::Identifier, "Expected page name")?;
        let name = name.value.clone();

        let route = self.consume(TokenKind::String, "Expected route string")?;
        let route = route.value.clone();

        let body = self.block()?;

        Ok(PageDecl {
            name,
            route,
            body,
            loc: self.location_from(start),
        })
    }

    // =========================================================================
    // Parameters and Attributes
    // =========================================================================

    fn parameter_list(&mut self) -> Result<Vec<Parameter>, ParseError> {
        self.consume(TokenKind::LParen, "Expected '('")?;
        let mut params = Vec::new();

        if !self.check(TokenKind::RParen) {
            loop {
                params.push(self.parameter()?);
                if !self.match_token(TokenKind::Comma) {
                    break;
                }
            }
        }

        self.consume(TokenKind::RParen, "Expected ')'")?;
        Ok(params)
    }

    fn parameter(&mut self) -> Result<Parameter, ParseError> {
        let start = self.current_location();
        let name = self.consume(TokenKind::Identifier, "Expected parameter name")?;
        let name = name.value.clone();

        self.consume(TokenKind::Colon, "Expected ':'")?;

        let binding = self.consume(TokenKind::Identifier, "Expected binding name")?;
        let binding = binding.value.clone();

        Ok(Parameter {
            name,
            binding,
            loc: self.location_from(start),
        })
    }

    fn attribute_list(&mut self) -> Result<Vec<Attribute>, ParseError> {
        self.consume(TokenKind::LBracket, "Expected '['")?;
        let mut attrs = Vec::new();

        if !self.check(TokenKind::RBracket) {
            loop {
                attrs.push(self.attribute()?);
                if !self.match_token(TokenKind::Comma) {
                    break;
                }
            }
        }

        self.consume(TokenKind::RBracket, "Expected ']'")?;
        Ok(attrs)
    }

    fn attribute(&mut self) -> Result<Attribute, ParseError> {
        let start = self.current_location();
        let name = self.consume(TokenKind::Identifier, "Expected attribute name")?;
        let name = name.value.clone();

        self.consume(TokenKind::Colon, "Expected ':'")?;

        let value = self.expression()?;

        Ok(Attribute {
            name,
            value,
            loc: self.location_from(start),
        })
    }

    // =========================================================================
    // Block and Nodes
    // =========================================================================

    fn block(&mut self) -> Result<Vec<Node>, ParseError> {
        self.consume(TokenKind::LBrace, "Expected '{'")?;
        let mut nodes = Vec::new();

        while !self.check(TokenKind::RBrace) && !self.is_at_end() {
            nodes.push(self.node()?);
        }

        self.consume(TokenKind::RBrace, "Expected '}'")?;
        Ok(nodes)
    }

    fn node(&mut self) -> Result<Node, ParseError> {
        if self.check(TokenKind::If) {
            self.if_statement().map(Node::If)
        } else if self.check(TokenKind::Each) {
            self.each_statement().map(Node::Each)
        } else if self.check(TokenKind::Slot) {
            self.slot().map(Node::Slot)
        } else if self.check(TokenKind::TextOpen) {
            self.text_node().map(Node::Text)
        } else if self.check(TokenKind::ContextPath) {
            self.dynamic_text().map(Node::Text)
        } else if self.check(TokenKind::ComponentName) {
            self.component_ref().map(Node::ComponentRef)
        } else if self.check(TokenKind::Identifier) {
            // Check if this is dynamic text (identifier followed by . for member access)
            // or if it's an HTML element tag
            let lookahead_pos = self.current + 1;
            if lookahead_pos < self.tokens.len() && self.tokens[lookahead_pos].kind == TokenKind::Dot {
                // This is member access like item.name - treat as dynamic text
                self.dynamic_expression_text().map(Node::Text)
            } else {
                // This is an HTML element tag
                self.element().map(Node::Element)
            }
        } else {
            Err(self.error("Expected element, component, or directive"))
        }
    }

    fn element(&mut self) -> Result<Element, ParseError> {
        let start = self.current_location();
        let tag = self.consume(TokenKind::Identifier, "Expected tag name")?;
        let tag = tag.value.clone();

        let attributes = if self.check(TokenKind::LBracket) {
            self.attribute_list()?
        } else {
            Vec::new()
        };

        // Check for shorthand text: button [onClick: submit] {{ Send }}
        let children = if self.check(TokenKind::TextOpen) {
            // Shorthand: text directly after element/attributes
            vec![Node::Text(self.text_node()?)]
        } else if self.check(TokenKind::LBrace) {
            self.block()?
        } else {
            Vec::new()
        };

        Ok(Element {
            tag,
            attributes,
            children,
            loc: self.location_from(start),
        })
    }

    fn component_ref(&mut self) -> Result<ComponentRef, ParseError> {
        let start = self.current_location();
        let name = self.consume(TokenKind::ComponentName, "Expected component name")?;
        let name = name.value.clone();

        let parameters = if self.check(TokenKind::LParen) {
            self.parameter_binding_list()?
        } else {
            Vec::new()
        };

        let children = if self.check(TokenKind::LBrace) {
            self.block()?
        } else {
            Vec::new()
        };

        Ok(ComponentRef {
            name,
            parameters,
            children,
            loc: self.location_from(start),
        })
    }

    fn parameter_binding_list(&mut self) -> Result<Vec<ParameterBinding>, ParseError> {
        self.consume(TokenKind::LParen, "Expected '('")?;
        let mut bindings = Vec::new();

        if !self.check(TokenKind::RParen) {
            loop {
                bindings.push(self.parameter_binding()?);
                if !self.match_token(TokenKind::Comma) {
                    break;
                }
            }
        }

        self.consume(TokenKind::RParen, "Expected ')'")?;
        Ok(bindings)
    }

    fn parameter_binding(&mut self) -> Result<ParameterBinding, ParseError> {
        let start = self.current_location();
        let name = self.consume(TokenKind::Identifier, "Expected parameter name")?;
        let name = name.value.clone();

        self.consume(TokenKind::Colon, "Expected ':'")?;

        let value = self.expression()?;

        Ok(ParameterBinding {
            name,
            value,
            loc: self.location_from(start),
        })
    }

    fn text_node(&mut self) -> Result<TextNode, ParseError> {
        let start = self.current_location();
        self.consume(TokenKind::TextOpen, "Expected '{{'")?;

        let content = if self.check(TokenKind::TextContent) {
            let token = self.advance();
            token.value.clone()
        } else {
            String::new()
        };

        self.consume(TokenKind::TextClose, "Expected '}}'")?;

        Ok(TextNode {
            content: content.trim().to_string(),
            is_dynamic: false,
            loc: self.location_from(start),
        })
    }

    fn dynamic_text(&mut self) -> Result<TextNode, ParseError> {
        let start = self.current_location();
        let token = self.consume(TokenKind::ContextPath, "Expected context path")?;

        Ok(TextNode {
            content: token.value.clone(),
            is_dynamic: true,
            loc: self.location_from(start),
        })
    }

    fn dynamic_expression_text(&mut self) -> Result<TextNode, ParseError> {
        let start = self.current_location();
        let expr = self.expression()?;

        // Convert expression to string representation for the content
        let content = self.expr_to_string(&expr);

        Ok(TextNode {
            content,
            is_dynamic: true,
            loc: self.location_from(start),
        })
    }

    fn expr_to_string(&self, expr: &Expression) -> String {
        match expr {
            Expression::ContextPath(p) => p.path.clone(),
            Expression::Identifier(id) => id.name.clone(),
            Expression::MemberAccess(m) => {
                let obj = self.expr_to_string(&m.object);
                format!("{}.{}", obj, m.property)
            }
            _ => String::new(),
        }
    }

    fn slot(&mut self) -> Result<Slot, ParseError> {
        let start = self.current_location();
        self.consume(TokenKind::Slot, "Expected '@slot'")?;

        Ok(Slot {
            loc: self.location_from(start),
        })
    }

    // =========================================================================
    // Control Flow
    // =========================================================================

    fn if_statement(&mut self) -> Result<IfStatement, ParseError> {
        let start = self.current_location();
        self.consume(TokenKind::If, "Expected '@if'")?;

        let condition = self.expression()?;
        let consequent = self.block()?;

        let alternate = if self.match_token(TokenKind::Else) {
            if self.check(TokenKind::If) {
                Some(Alternate::ElseIf(Box::new(self.if_statement()?)))
            } else {
                Some(Alternate::Block(self.block()?))
            }
        } else {
            None
        };

        Ok(IfStatement {
            condition,
            consequent,
            alternate,
            loc: self.location_from(start),
        })
    }

    fn each_statement(&mut self) -> Result<EachStatement, ParseError> {
        let start = self.current_location();
        self.consume(TokenKind::Each, "Expected '@each'")?;

        let iterable = self.expression()?;

        self.consume(TokenKind::As, "Expected 'as'")?;

        let item_name = self.consume(TokenKind::Identifier, "Expected item name")?;
        let item_name = item_name.value.clone();

        let index_name = if self.match_token(TokenKind::Comma) {
            let name = self.consume(TokenKind::Identifier, "Expected index name")?;
            Some(name.value.clone())
        } else {
            None
        };

        let body = self.block()?;

        Ok(EachStatement {
            iterable,
            item_name,
            index_name,
            body,
            loc: self.location_from(start),
        })
    }

    // =========================================================================
    // Expressions
    // =========================================================================

    fn expression(&mut self) -> Result<Expression, ParseError> {
        self.ternary()
    }

    fn ternary(&mut self) -> Result<Expression, ParseError> {
        let start = self.current_location();
        let mut expr = self.or()?;

        if self.match_token(TokenKind::Question) {
            let consequent = self.expression()?;
            self.consume(TokenKind::Colon, "Expected ':' in ternary")?;
            let alternate = self.expression()?;

            expr = Expression::Ternary(TernaryExpr {
                condition: Box::new(expr),
                consequent: Box::new(consequent),
                alternate: Box::new(alternate),
                loc: self.location_from(start),
            });
        }

        Ok(expr)
    }

    fn or(&mut self) -> Result<Expression, ParseError> {
        let start = self.current_location();
        let mut left = self.and()?;

        while self.match_token(TokenKind::Or) {
            let right = self.and()?;
            left = Expression::Binary(BinaryExpr {
                operator: BinaryOp::Or,
                left: Box::new(left),
                right: Box::new(right),
                loc: self.location_from(start),
            });
        }

        Ok(left)
    }

    fn and(&mut self) -> Result<Expression, ParseError> {
        let start = self.current_location();
        let mut left = self.equality()?;

        while self.match_token(TokenKind::And) {
            let right = self.equality()?;
            left = Expression::Binary(BinaryExpr {
                operator: BinaryOp::And,
                left: Box::new(left),
                right: Box::new(right),
                loc: self.location_from(start),
            });
        }

        Ok(left)
    }

    fn equality(&mut self) -> Result<Expression, ParseError> {
        let start = self.current_location();
        let mut left = self.comparison()?;

        loop {
            let op = if self.match_token(TokenKind::Eq) {
                BinaryOp::Eq
            } else if self.match_token(TokenKind::Ne) {
                BinaryOp::Ne
            } else {
                break;
            };

            let right = self.comparison()?;
            left = Expression::Binary(BinaryExpr {
                operator: op,
                left: Box::new(left),
                right: Box::new(right),
                loc: self.location_from(start),
            });
        }

        Ok(left)
    }

    fn comparison(&mut self) -> Result<Expression, ParseError> {
        let start = self.current_location();
        let mut left = self.additive()?;

        loop {
            let op = if self.match_token(TokenKind::Gt) {
                BinaryOp::Gt
            } else if self.match_token(TokenKind::Ge) {
                BinaryOp::Ge
            } else if self.match_token(TokenKind::Lt) {
                BinaryOp::Lt
            } else if self.match_token(TokenKind::Le) {
                BinaryOp::Le
            } else {
                break;
            };

            let right = self.additive()?;
            left = Expression::Binary(BinaryExpr {
                operator: op,
                left: Box::new(left),
                right: Box::new(right),
                loc: self.location_from(start),
            });
        }

        Ok(left)
    }

    fn additive(&mut self) -> Result<Expression, ParseError> {
        let start = self.current_location();
        let mut left = self.postfix()?;

        loop {
            let op = if self.match_token(TokenKind::Plus) {
                BinaryOp::Add
            } else if self.match_token(TokenKind::Minus) {
                BinaryOp::Sub
            } else {
                break;
            };

            let right = self.postfix()?;
            left = Expression::Binary(BinaryExpr {
                operator: op,
                left: Box::new(left),
                right: Box::new(right),
                loc: self.location_from(start),
            });
        }

        Ok(left)
    }

    fn postfix(&mut self) -> Result<Expression, ParseError> {
        let start = self.current_location();
        let mut expr = self.primary()?;

        loop {
            if self.match_token(TokenKind::Dot) {
                let property = self.consume(TokenKind::Identifier, "Expected property name after '.'")?;
                expr = Expression::MemberAccess(MemberAccessExpr {
                    object: Box::new(expr),
                    property: property.value.clone(),
                    loc: self.location_from(start),
                });
            } else if self.check(TokenKind::LParen) {
                // Handle function calls on the current expression
                // For now, only support simple identifier calls
                if let Expression::Identifier(ident) = &expr {
                    self.advance(); // consume '('
                    let mut arguments = Vec::new();

                    if !self.check(TokenKind::RParen) {
                        loop {
                            arguments.push(self.expression()?);
                            if !self.match_token(TokenKind::Comma) {
                                break;
                            }
                        }
                    }

                    self.consume(TokenKind::RParen, "Expected ')'")?;

                    expr = Expression::Call(CallExpr {
                        callee: ident.name.clone(),
                        arguments,
                        loc: self.location_from(start),
                    });
                } else {
                    break;
                }
            } else {
                break;
            }
        }

        Ok(expr)
    }

    fn primary(&mut self) -> Result<Expression, ParseError> {
        let start = self.current_location();

        if self.check(TokenKind::String) {
            let token = self.advance();
            return Ok(Expression::String(StringLiteral {
                value: token.value.clone(),
                loc: self.location_from(start),
            }));
        }

        if self.check(TokenKind::Number) {
            let token = self.advance();
            let value: f64 = token.value.parse().unwrap_or(0.0);
            return Ok(Expression::Number(NumberLiteral {
                value,
                loc: self.location_from(start),
            }));
        }

        if self.check(TokenKind::True) {
            self.advance();
            return Ok(Expression::Boolean(BooleanLiteral {
                value: true,
                loc: self.location_from(start),
            }));
        }

        if self.check(TokenKind::False) {
            self.advance();
            return Ok(Expression::Boolean(BooleanLiteral {
                value: false,
                loc: self.location_from(start),
            }));
        }

        if self.check(TokenKind::ContextPath) {
            let token = self.advance();
            return Ok(Expression::ContextPath(ContextPathExpr {
                path: token.value.clone(),
                loc: self.location_from(start),
            }));
        }

        if self.check(TokenKind::Identifier) {
            let token = self.advance();
            let name = token.value.clone();

            return Ok(Expression::Identifier(IdentifierExpr {
                name,
                loc: self.location_from(start),
            }));
        }

        if self.check(TokenKind::LParen) {
            self.advance();
            let expr = self.expression()?;
            self.consume(TokenKind::RParen, "Expected ')'")?;
            return Ok(expr);
        }

        Err(self.error("Expected expression"))
    }

    // =========================================================================
    // Helpers
    // =========================================================================

    fn check(&self, kind: TokenKind) -> bool {
        if self.is_at_end() {
            false
        } else {
            self.peek().kind == kind
        }
    }

    fn match_token(&mut self, kind: TokenKind) -> bool {
        if self.check(kind) {
            self.advance();
            true
        } else {
            false
        }
    }

    fn consume(&mut self, kind: TokenKind, message: &str) -> Result<&Token, ParseError> {
        if self.check(kind) {
            Ok(self.advance())
        } else {
            Err(self.error(message))
        }
    }

    fn advance(&mut self) -> &Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.previous()
    }

    fn peek(&self) -> &Token {
        &self.tokens[self.current]
    }

    fn previous(&self) -> &Token {
        &self.tokens[self.current - 1]
    }

    fn is_at_end(&self) -> bool {
        self.peek().kind == TokenKind::Eof
    }

    fn current_location(&self) -> Location {
        self.peek().location
    }

    fn location_from(&self, start: Location) -> Location {
        Location {
            line: start.line,
            column: start.column,
            start: start.start,
            end: self.previous().location.end,
        }
    }

    fn error(&self, message: &str) -> ParseError {
        let token = self.peek();
        ParseError::new(
            format!("{} (got {})", message, token.kind.name()),
            token.location,
        )
    }

    fn synchronize(&mut self) {
        self.advance();

        while !self.is_at_end() {
            if self.previous().kind == TokenKind::RBrace {
                return;
            }

            match self.peek().kind {
                TokenKind::Component | TokenKind::Section | TokenKind::Page => return,
                _ => {}
            }

            self.advance();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::tokenize;

    fn parse_source(source: &str) -> Result<Program, Vec<ParseError>> {
        let tokens = tokenize(source).unwrap();
        Parser::new(&tokens).parse()
    }

    #[test]
    fn test_component_decl() {
        let ast = parse_source("component NavBar { }").unwrap();
        assert_eq!(ast.body.len(), 1);
        match &ast.body[0] {
            Declaration::Component(c) => assert_eq!(c.name, "NavBar"),
            _ => panic!("Expected component"),
        }
    }

    #[test]
    fn test_page_decl() {
        let ast = parse_source(r#"page home "/" { }"#).unwrap();
        assert_eq!(ast.body.len(), 1);
        match &ast.body[0] {
            Declaration::Page(p) => {
                assert_eq!(p.name, "home");
                assert_eq!(p.route, "/");
            }
            _ => panic!("Expected page"),
        }
    }

    #[test]
    fn test_element_with_attrs() {
        let ast = parse_source(r#"component Test { div [class: "foo"] { } }"#).unwrap();
        match &ast.body[0] {
            Declaration::Component(c) => {
                assert_eq!(c.body.len(), 1);
                match &c.body[0] {
                    Node::Element(e) => {
                        assert_eq!(e.tag, "div");
                        assert_eq!(e.attributes.len(), 1);
                        assert_eq!(e.attributes[0].name, "class");
                    }
                    _ => panic!("Expected element"),
                }
            }
            _ => panic!("Expected component"),
        }
    }

    #[test]
    fn test_if_statement() {
        let ast = parse_source(r#"component Test { @if ctx.show { div { } } }"#).unwrap();
        match &ast.body[0] {
            Declaration::Component(c) => {
                match &c.body[0] {
                    Node::If(stmt) => {
                        assert!(stmt.alternate.is_none());
                    }
                    _ => panic!("Expected if statement"),
                }
            }
            _ => panic!("Expected component"),
        }
    }

    #[test]
    fn test_each_statement() {
        let ast = parse_source(r#"component Test { @each ctx.items as item { div { } } }"#).unwrap();
        match &ast.body[0] {
            Declaration::Component(c) => {
                match &c.body[0] {
                    Node::Each(stmt) => {
                        assert_eq!(stmt.item_name, "item");
                        assert!(stmt.index_name.is_none());
                    }
                    _ => panic!("Expected each statement"),
                }
            }
            _ => panic!("Expected component"),
        }
    }
}
