use crate::{
    attrs::{self, Attr},
    document::{self, Document},
    elements::Node,
};

pub fn render_document(tree: Node) -> String {
    let result = render_document_node(tree);
    [
        render_doctype(result.doctype.unwrap_or("html".into())),
        "<html".into(),
        result.html_attrs,
        ">\n<head>".into(),
        document::ENCODING.into(),
        result.head,
        "</head>\n<body".into(),
        result.body_attrs,
        ">".into(),
        result.body,
        render_scripts(result.scripts),
        "</body>\n</html>\n".into(),
    ]
    .concat()
}

pub fn render_inline(tree: Node) -> String {
    render_inline_node(tree)
}

/// The HtmlBuilder type holds our two functions for constructing Html.
/// `T` is the type we want our HtmlBuilder to construct
/// `map` is the function that specifies how we should render a single `Node`
/// `fold` is the function that specifies how we want to combine pieces of `T`
struct HtmlBuilder<T> {
    map: fn(Node) -> T,
    combine: fn(Vec<T>) -> T,
}

/// Builds complete Html documents, and merges them using the appropriate strategy.
const DOCUMENT_BUILDER: HtmlBuilder<Document> = HtmlBuilder {
    map: render_document_node,
    combine: Document::concat,
};

/// Builds snippets of html, and merges them by simply concatenating them
const INLINE_BUILDER: HtmlBuilder<String> = HtmlBuilder {
    map: render_inline_node,
    combine: |v| v.concat(),
};

fn render_script(script: String) -> String {
    ["<script>".into(), script, "</script>\n".into()].concat()
}

fn render_scripts(scripts: Vec<String>) -> String {
    scripts.into_iter().map(render_script).collect()
}

fn render_doctype(doctype: String) -> String {
    ["<!DOCTYPE ".into(), doctype, ">\n".into()].concat()
}

fn render_children<T: Send>(children: Vec<Node>, builder: HtmlBuilder<T>) -> T {
    (builder.combine)(children.into_iter().map(builder.map).collect())
}

fn render_attrs(attrs: Vec<Attr>) -> String {
    attrs
        .into_iter()
        .map(Attr::render)
        .fold(String::new(), |mut acc, s| {
            acc.push(' ');
            acc.push_str(&s);
            acc
        })
}

/// Render a `Node` as an Html Document
fn render_document_node(tree: Node) -> Document {
    match tree {
        Node::Doctype { content } => Document::from_doctype(content),
        Node::Html { attrs, children } => {
            let mut document = render_children(children, DOCUMENT_BUILDER);
            document.append_html_attrs(render_attrs(attrs));
            document
        }
        Node::Head { children } => render_children(children, DOCUMENT_BUILDER).into_head(),
        Node::Body { attrs, children } => {
            let mut document = render_children(children, DOCUMENT_BUILDER);
            document.append_body_attrs(render_attrs(attrs));
            document
        }
        Node::Fragment { children } => render_children(children, DOCUMENT_BUILDER),
        Node::Element {
            tag,
            attrs,
            children,
        } => {
            let mut child_document = render_children(children, DOCUMENT_BUILDER);
            child_document.body = [
                "<".into(),
                tag.clone(),
                render_attrs(attrs),
                ">".into(),
                child_document.body,
                "</".into(),
                tag,
                ">".into(),
            ]
            .concat();
            child_document
        }
        Node::LeafElement { tag, attrs } => {
            Document::from_body(["<".into(), tag, render_attrs(attrs), " />".into()].concat())
        }
        Node::Comment { content } => {
            let content = content.replace("-->", "");
            Document::from_body(["<!-- ".into(), content, " -->".into()].concat())
        }
        Node::Text { content } => {
            let content = content
                .replace("&", "&amp;")
                .replace("<", "&lt;")
                .replace(">", "&gt;");
            Document::from_body(content)
        }
        Node::UnsafeInlineHtml { content } => Document::from_body(content),
        Node::Script { script } => Document::from_script(script),
        Node::Nothing => Document::new(),
    }
}

fn render_inline_node(tree: Node) -> String {
    match tree {
        Node::Doctype { content } => render_doctype(content),
        Node::Html { attrs, children } => render_inline_node(Node::Element {
            tag: "html".into(),
            attrs,
            children,
        }),
        Node::Head { children } => render_inline_node(Node::Element {
            tag: "head".into(),
            attrs: vec![],
            children,
        }),
        Node::Body { attrs, children } => render_inline_node(Node::Element {
            tag: "body".into(),
            attrs,
            children,
        }),
        Node::Fragment { children } => render_children(children, INLINE_BUILDER),
        Node::Element {
            tag,
            attrs,
            children,
        } => {
            let child_txt = render_children(children, INLINE_BUILDER);
            [
                "<".into(),
                tag.clone(),
                render_attrs(attrs),
                ">".into(),
                child_txt,
                "</".into(),
                tag,
                ">".into(),
            ]
            .concat()
        }
        Node::LeafElement { tag, attrs } => {
            ["<".into(), tag, render_attrs(attrs), " />".into()].concat()
        }
        Node::Comment { content } => {
            let content = content.replace("-->", "");
            ["<!-- ".into(), content, " -->".into()].concat()
        }
        Node::Text { content } => content
            .replace("&", "&amp;")
            .replace("<", "&lt;")
            .replace(">", "&gt;"),
        Node::UnsafeInlineHtml { content } => content,
        Node::Script { script } => render_inline_node(Node::Element {
            tag: "script".into(),
            attrs: vec![attrs::type_("module")],
            children: vec![Node::Text { content: script }],
        }),
        Node::Nothing => String::new(),
    }
}
