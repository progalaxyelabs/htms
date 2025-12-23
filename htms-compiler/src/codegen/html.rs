//! HTML generator
//!
//! Generates static HTML from HTMS templates.
//! v1: Empty context - all dynamic data is blank.

use crate::ast::*;
use crate::{CompileOptions, GeneratedFile};

/// Generate HTML output
pub fn generate(program: &Program, options: &CompileOptions) -> Vec<GeneratedFile> {
    let mut files = Vec::new();

    // Collect all pages
    let pages: Vec<&PageDecl> = program.body.iter()
        .filter_map(|decl| match decl {
            Declaration::Page(page) => Some(page),
            _ => None,
        })
        .collect();

    if pages.is_empty() {
        return files;
    }

    if options.split_templates {
        // Split mode: Generate separate template files + main HTML with lazy loading
        generate_split_templates(program, &pages, options, &mut files);
    } else {
        // Inline mode: Generate single HTML file with all templates inline
        generate_inline_templates(program, &pages, options, &mut files);
    }

    files
}

/// Generate inline templates (all templates in single HTML file)
fn generate_inline_templates(
    program: &Program,
    pages: &[&PageDecl],
    options: &CompileOptions,
    files: &mut Vec<GeneratedFile>,
) {
    // Detect common components used in all pages (for hoisting)
    let common_components = find_common_components(pages);

    let mut templates_content = String::new();
    let mut routes = Vec::new();
    let mut layout_content = String::new();

    // Generate common components (layout) once
    if !common_components.is_empty() {
        for component_name in &common_components {
            if let Some(component) = find_component(program, component_name) {
                generate_component_html(component, &mut layout_content, 2, program);
            }
        }
    }

    for page in pages {
        let page_id = format!("page-{}", page.name.to_lowercase());
        routes.push((page.route.clone(), page_id.clone()));

        // Generate template tag
        templates_content.push_str(&format!("  <template id=\"{}\">\n", page_id));

        // Generate page content (excluding common components)
        let mut page_content = String::new();
        for node in &page.body {
            if !is_common_component_ref(node, &common_components) {
                generate_node(node, &mut page_content, 2, program);
            }
        }
        templates_content.push_str(&page_content);

        templates_content.push_str("  </template>\n\n");
    }

    // Generate routing script
    let router_script = generate_router_script(&routes, false);

    // Generate final HTML
    let html = if let Some(template) = options.template_html.as_deref() {
        // Inject layout, templates, app container, and script into body
        let mut combined = String::new();
        if !layout_content.is_empty() {
            combined.push_str("  <div id=\"layout\">\n");
            combined.push_str(&layout_content);
            combined.push_str("  </div>\n\n");
        }
        combined.push_str("  <div id=\"app\"></div>\n\n");
        combined.push_str(&templates_content);
        combined.push_str(&router_script);
        inject_into_body(template, &combined)
    } else {
        // Generate standalone HTML5 document
        let mut html = String::new();
        html.push_str("<!DOCTYPE html>\n");
        html.push_str("<html lang=\"en\">\n");
        html.push_str("<head>\n");
        html.push_str("  <meta charset=\"UTF-8\">\n");
        html.push_str("  <meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0\">\n");
        html.push_str(&format!("  <title>{}</title>\n", title_case(&pages[0].name)));
        html.push_str("</head>\n");
        html.push_str("<body>\n");
        if !layout_content.is_empty() {
            html.push_str("  <div id=\"layout\">\n");
            html.push_str(&layout_content);
            html.push_str("  </div>\n\n");
        }
        html.push_str("  <div id=\"app\"></div>\n\n");
        html.push_str(&templates_content);
        html.push_str(&router_script);
        html.push_str("</body>\n");
        html.push_str("</html>\n");
        html
    };

    // Determine output filename
    let output_filename = if let Some(filename) = options.source_filename.as_deref() {
        // Use source filename (e.g., "app.htms" -> "app.html")
        filename.to_string()
    } else {
        // Fall back to first page name
        format!("{}.html", pages[0].name.to_lowercase())
    };

    files.push(GeneratedFile {
        path: output_filename,
        content: html,
    });
}

/// Generate split templates (separate .template.html files for lazy loading)
fn generate_split_templates(
    program: &Program,
    pages: &[&PageDecl],
    options: &CompileOptions,
    files: &mut Vec<GeneratedFile>,
) {
    let mut routes = Vec::new();

    // Generate individual template files
    for page in pages {
        let page_name = page.name.to_lowercase();
        let template_filename = format!("{}.template.html", page_name);
        routes.push((page.route.clone(), template_filename.clone()));

        // Generate page content
        let mut page_content = String::new();
        for node in &page.body {
            generate_node(node, &mut page_content, 0, program);
        }

        files.push(GeneratedFile {
            path: template_filename,
            content: page_content,
        });
    }

    // Generate routing script for lazy loading
    let router_script = generate_router_script(&routes, true);

    // Generate main HTML file
    let html = if let Some(template) = options.template_html.as_deref() {
        // Inject app container and script into body
        let mut combined = String::new();
        combined.push_str("  <div id=\"app\">Loading...</div>\n\n");
        combined.push_str(&router_script);
        inject_into_body(template, &combined)
    } else {
        // Generate standalone HTML5 document
        let mut html = String::new();
        html.push_str("<!DOCTYPE html>\n");
        html.push_str("<html lang=\"en\">\n");
        html.push_str("<head>\n");
        html.push_str("  <meta charset=\"UTF-8\">\n");
        html.push_str("  <meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0\">\n");
        html.push_str(&format!("  <title>{}</title>\n", title_case(&pages[0].name)));
        html.push_str("</head>\n");
        html.push_str("<body>\n");
        html.push_str("  <div id=\"app\">Loading...</div>\n");
        html.push_str(&router_script);
        html.push_str("</body>\n");
        html.push_str("</html>\n");
        html
    };

    // Determine output filename
    let output_filename = if let Some(filename) = options.source_filename.as_deref() {
        filename.to_string()
    } else {
        format!("{}.html", pages[0].name.to_lowercase())
    };

    files.push(GeneratedFile {
        path: output_filename,
        content: html,
    });
}

/// Generate client-side routing script
fn generate_router_script(routes: &[(String, String)], lazy_load: bool) -> String {
    let mut script = String::new();
    script.push_str("  <script>\n");

    if lazy_load {
        // Lazy loading router with fetch
        script.push_str("    // HTMS Router v2 - Lazy-loading template routing\n");
        script.push_str("    const routes = {\n");
        for (path, template_file) in routes {
            script.push_str(&format!("      '{}': '{}',\n", path, template_file));
        }
        script.push_str("    };\n\n");

        script.push_str("    // Cache loaded templates\n");
        script.push_str("    const templateCache = new Map();\n\n");

        script.push_str("    async function loadTemplate(url) {\n");
        script.push_str("      if (templateCache.has(url)) {\n");
        script.push_str("        return templateCache.get(url);\n");
        script.push_str("      }\n\n");
        script.push_str("      const response = await fetch(url);\n");
        script.push_str("      const html = await response.text();\n");
        script.push_str("      templateCache.set(url, html);\n");
        script.push_str("      return html;\n");
        script.push_str("    }\n\n");

        script.push_str("    async function renderPage() {\n");
        script.push_str("      const path = window.location.pathname;\n");
        script.push_str("      const templateUrl = routes[path] || routes['/'];\n\n");
        script.push_str("      if (!templateUrl) {\n");
        script.push_str("        document.getElementById('app').innerHTML = '<h1>404 - Page Not Found</h1>';\n");
        script.push_str("        return;\n");
        script.push_str("      }\n\n");
        script.push_str("      try {\n");
        script.push_str("        const html = await loadTemplate(templateUrl);\n");
        script.push_str("        document.getElementById('app').innerHTML = html;\n");
        script.push_str("      } catch (error) {\n");
        script.push_str("        console.error('Failed to load template:', error);\n");
        script.push_str("        document.getElementById('app').innerHTML = '<h1>Error loading page</h1>';\n");
        script.push_str("      }\n");
        script.push_str("    }\n\n");
    } else {
        // Inline template router
        script.push_str("    // HTMS Router v2 - Template-based client-side routing\n");
        script.push_str("    const routes = {\n");
        for (path, template_id) in routes {
            script.push_str(&format!("      '{}': '{}',\n", path, template_id));
        }
        script.push_str("    };\n\n");

        script.push_str("    function renderPage() {\n");
        script.push_str("      const path = window.location.pathname;\n");
        script.push_str("      const templateId = routes[path] || routes['/'];\n");
        script.push_str("      \n");
        script.push_str("      // Get or create app container\n");
        script.push_str("      let appContainer = document.getElementById('app');\n");
        script.push_str("      if (!appContainer) {\n");
        script.push_str("        appContainer = document.createElement('div');\n");
        script.push_str("        appContainer.id = 'app';\n");
        script.push_str("        document.body.appendChild(appContainer);\n");
        script.push_str("      }\n");
        script.push_str("      \n");
        script.push_str("      if (!templateId) {\n");
        script.push_str("        appContainer.innerHTML = '<h1>404 - Page Not Found</h1>';\n");
        script.push_str("        return;\n");
        script.push_str("      }\n");
        script.push_str("      \n");
        script.push_str("      const template = document.getElementById(templateId);\n");
        script.push_str("      if (!template) {\n");
        script.push_str("        console.error('Template not found:', templateId);\n");
        script.push_str("        return;\n");
        script.push_str("      }\n");
        script.push_str("      \n");
        script.push_str("      // Clear container and render template content\n");
        script.push_str("      const content = template.content.cloneNode(true);\n");
        script.push_str("      appContainer.innerHTML = '';\n");
        script.push_str("      appContainer.appendChild(content);\n");
        script.push_str("    }\n\n");
    }

    // Client-side navigation (intercept link clicks)
    script.push_str("    // Client-side navigation - intercept internal link clicks\n");
    script.push_str("    document.addEventListener('click', (e) => {\n");
    script.push_str("      const link = e.target.closest('a');\n");
    script.push_str("      if (link && link.href && link.origin === window.location.origin) {\n");
    script.push_str("        e.preventDefault();\n");
    script.push_str("        window.history.pushState({}, '', link.pathname);\n");
    script.push_str("        renderPage();\n");
    script.push_str("      }\n");
    script.push_str("    });\n\n");

    script.push_str("    // Handle back/forward buttons\n");
    script.push_str("    window.addEventListener('popstate', renderPage);\n");
    script.push_str("    \n");
    script.push_str("    // Initial render on load\n");
    script.push_str("    window.addEventListener('load', renderPage);\n");
    script.push_str("  </script>\n");

    script
}

/// Inject generated content into template's <body> tag
fn inject_into_body(template: &str, content: &str) -> String {
    // Find <body> tag and inject content
    // We look for <body> or <body ...attributes...>
    use regex::Regex;

    let re = Regex::new(r"(?i)(<body[^>]*>)").unwrap();

    if let Some(captures) = re.captures(template) {
        let body_tag = captures.get(1).unwrap();
        let insert_pos = body_tag.end();

        let mut result = String::new();
        result.push_str(&template[..insert_pos]);
        result.push('\n');
        result.push_str(content);
        result.push_str(&template[insert_pos..]);
        result
    } else {
        // No <body> tag found, return template as-is
        template.to_string()
    }
}

fn generate_node(node: &Node, html: &mut String, indent: usize, program: &Program) {
    match node {
        Node::Element(el) => generate_element(el, html, indent, program),
        Node::ComponentRef(comp_ref) => {
            // Resolve and inline component body
            if let Some(component) = find_component(program, &comp_ref.name) {
                for child in &component.body {
                    generate_node(child, html, indent, program);
                }
            }
        }
        Node::Text(t) => generate_text(t, html),
        Node::If(_) => {
            // v1: @if assumes false, renders nothing
        }
        Node::Each(_) => {
            // v1: @each assumes empty array, renders nothing
        }
        Node::Slot(_) => {
            // v1: Slots are not rendered
        }
    }
}

/// Find a component by name in the program
fn find_component<'a>(program: &'a Program, name: &str) -> Option<&'a ComponentDecl> {
    program.body.iter().find_map(|decl| match decl {
        Declaration::Component(comp) if comp.name == name => Some(comp),
        _ => None,
    })
}

fn generate_element(el: &Element, html: &mut String, indent: usize, program: &Program) {
    let indent_str = "  ".repeat(indent);

    // v1: @if directive - skip rendering (assume false)
    if el.if_directive.is_some() {
        return;
    }

    // v1: @for directive - render empty container
    if el.for_directive.is_some() {
        html.push_str(&indent_str);
        html.push('<');
        html.push_str(&el.tag);

        // Attributes
        for attr in &el.attributes {
            generate_attribute(attr, html);
        }

        html.push_str("></");
        html.push_str(&el.tag);
        html.push_str(">\n");
        return;
    }

    // Check if self-closing tag
    let self_closing = is_self_closing(&el.tag);

    // Opening tag
    html.push_str(&indent_str);
    html.push('<');
    html.push_str(&el.tag);

    // Attributes
    for attr in &el.attributes {
        generate_attribute(attr, html);
    }

    if self_closing {
        html.push_str(">\n");
        return;
    }

    html.push('>');

    // Children
    if el.children.is_empty() {
        // Empty element - close on same line
        html.push_str("</");
        html.push_str(&el.tag);
        html.push_str(">\n");
    } else if el.children.len() == 1 && matches!(el.children[0], Node::Text(_)) {
        // Single text child - inline
        generate_node(&el.children[0], html, 0, program);
        html.push_str("</");
        html.push_str(&el.tag);
        html.push_str(">\n");
    } else {
        // Multiple children or complex content
        html.push('\n');
        for child in &el.children {
            generate_node(child, html, indent + 1, program);
        }
        html.push_str(&indent_str);
        html.push_str("</");
        html.push_str(&el.tag);
        html.push_str(">\n");
    }
}

fn generate_attribute(attr: &Attribute, html: &mut String) {
    html.push(' ');
    html.push_str(&attr.name);
    html.push_str("=\"");

    match &attr.value {
        Expression::String(s) => {
            html.push_str(&escape_html(&s.value));
        }
        Expression::Number(n) => {
            html.push_str(&n.value.to_string());
        }
        Expression::Boolean(b) => {
            if b.value {
                // Boolean true - render attribute name only (HTML5 style)
                html.pop(); // Remove ="
                html.pop(); // Remove "
                return;
            } else {
                // Boolean false - skip attribute entirely
                html.pop(); // Remove ="
                html.pop(); // Remove "
                html.pop(); // Remove space
                html.pop(); // Remove name
                return;
            }
        }
        Expression::ContextPath(_) => {
            // v1: Context data is empty
        }
        Expression::Identifier(_) => {
            // v1: Identifiers (like event handlers) are skipped
        }
        Expression::MemberAccess(_) => {
            // v1: Member access (ctx.foo.bar) is empty
        }
        Expression::Binary(_) => {
            // v1: Binary expressions can't be evaluated without context
        }
        Expression::Ternary(_) => {
            // v1: Ternary expressions can't be evaluated without context
        }
        Expression::Call(_) => {
            // v1: Function calls are skipped
        }
        Expression::Event(_) => {
            // v1: Event handlers are skipped
            html.pop(); // Remove ="
            html.pop(); // Remove "
            html.pop(); // Remove space
            // Remove attribute name
            for _ in 0..attr.name.len() {
                html.pop();
            }
            return;
        }
    }

    html.push('"');
}

fn generate_text(text: &TextNode, html: &mut String) {
    // v1: Always remove ${...} interpolations (empty context)
    // This handles both is_dynamic=true and any missed dynamic content
    let static_text = remove_interpolations(&text.content);

    // Only output if there's static text remaining
    if !static_text.trim().is_empty() {
        html.push_str(&escape_html(&static_text));
    }
}

fn remove_interpolations(text: &str) -> String {
    // Remove ${...} patterns
    use regex::Regex;
    let re = Regex::new(r"\$\{[^}]+\}").unwrap();
    re.replace_all(text, "").to_string()
}

fn escape_html(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#39;")
}

fn is_self_closing(tag: &str) -> bool {
    matches!(
        tag,
        "area" | "base" | "br" | "col" | "embed" | "hr" | "img" | "input" |
        "link" | "meta" | "param" | "source" | "track" | "wbr"
    )
}

fn title_case(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}

/// Find components that appear in ALL pages (candidates for hoisting)
fn find_common_components(pages: &[&PageDecl]) -> Vec<String> {
    if pages.is_empty() {
        return Vec::new();
    }

    // Get components from first page
    let mut common = get_component_refs(&pages[0].body);

    // Keep only components that appear in ALL pages
    for page in pages.iter().skip(1) {
        let page_components = get_component_refs(&page.body);
        common.retain(|comp| page_components.contains(comp));
    }

    common
}

/// Extract component reference names from nodes
fn get_component_refs(nodes: &[Node]) -> Vec<String> {
    let mut components = Vec::new();
    for node in nodes {
        if let Node::ComponentRef(comp_ref) = node {
            components.push(comp_ref.name.clone());
        }
    }
    components
}

/// Check if a node is a reference to a common component
fn is_common_component_ref(node: &Node, common_components: &[String]) -> bool {
    if let Node::ComponentRef(comp_ref) = node {
        common_components.contains(&comp_ref.name)
    } else {
        false
    }
}

/// Generate HTML for a component (used for layout rendering)
fn generate_component_html(component: &ComponentDecl, html: &mut String, indent: usize, program: &Program) {
    for node in &component.body {
        generate_node(node, html, indent, program);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_escape_html() {
        assert_eq!(escape_html("<script>"), "&lt;script&gt;");
        assert_eq!(escape_html("a & b"), "a &amp; b");
    }

    #[test]
    fn test_remove_interpolations() {
        assert_eq!(remove_interpolations("Hello ${ctx.name}!"), "Hello !");
        assert_eq!(remove_interpolations("${ctx.title}"), "");
        assert_eq!(remove_interpolations("Static text"), "Static text");
    }
}
