//! This library is a port of [nakai](https://github.com/nakaixo/nakai), an html builder
//! library written in Gleam.

use crate::attrs::Attr;
use paste::paste;

/// Top level type to represent a piece of HTML (a node).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Node {
    Doctype {
        content: String,
    },
    Html {
        attrs: Vec<Attr>,
        children: Vec<Node>,
    },
    Head {
        children: Vec<Node>,
    },
    Body {
        attrs: Vec<Attr>,
        children: Vec<Node>,
    },

    /// A transparent container that will render it's children.
    /// Equivalent to `<> ... some elements ... </>` in html macros.
    Fragment {
        children: Vec<Node>,
    },

    Element {
        tag: String,
        attrs: Vec<Attr>,
        children: Vec<Node>,
    },

    /// A self closing HTML element
    LeafElement {
        tag: String,
        attrs: Vec<Attr>,
    },

    Comment {
        content: String,
    },

    Text {
        content: String,
    },

    UnsafeInlineHtml {
        content: String,
    },

    Script {
        script: String,
    },

    Nothing,
}

macro_rules! t {
    ($e:expr) => {
        &[Node::Text { content: $e.into() }]
    };
}

/// Generates the html helpers
macro_rules! html {
    ($($i:ident),+) => {
        paste! {
            $(pub fn $i<const N: usize, const M: usize>(attrs: [Attr; N], children: [impl Into<Node> + Clone; M]) -> Node {
                [<$i _inner>](&attrs, &children)
            }

            pub fn [<$i _inner>](attrs: &[Attr], children: &[impl Into<Node> + Clone]) -> Node {
                Node::Element { tag: stringify!($i).into(), attrs: attrs.to_vec(), children: children.into_iter().cloned().map(Into::into).collect() }
            })+
        }
    };
}

macro_rules! html_self_closing {
    ($($i:ident),+) => {
        paste! {
            $(
                pub fn $i<const N: usize>(attrs: [Attr; N]) -> Node {
                    [<$i _inner>](&attrs)
                }

                pub fn [<$i _inner>](attrs: &[Attr]) -> Node {
                Node::LeafElement { tag: stringify!($i).into(), attrs: attrs.to_vec() }
            })+
        }
    };
}

html!(
    a, abbr, address, article, aside, audio, b, bdi, bdo, blockquote, button, canvas, caption,
    cite, code, col, colgroup, data, datalist, dd, del, details, dfn, dialog, div, dl, dt, em,
    embed, fieldset, figcaption, figure, footer, form, h1, h2, h3, h4, h5, h6, header, i, iframe,
    ins, kbd, label, legend, li, main, map, mark, math, menu, menuitem, meter, nav, noscript,
    object, ol, optgroup, option, output, p, param, picture, pre, progress, q, rp, rt, ruby, s,
    samp, section, select, small, span, strong, sub, summary, sup, svg, table, tbody, td, textarea,
    tfoot, th, thead, time, tr, u, ul, var, video, wbr
);

html_self_closing!(area, base, br, hr, img, input, link, meta, source, track);

impl Into<Node> for &'static str {
    fn into(self) -> Node {
        Node::Text {
            content: self.into(),
        }
    }
}

impl Node {
    pub fn render(&self) -> String {
        match self {
            Node::Doctype { content } => todo!(),
            Node::Html { attrs, children } => todo!(),
            Node::Head { children } => todo!(),
            Node::Body { attrs, children } => todo!(),
            Node::Fragment { children } => todo!(),
            Node::Element {
                tag,
                attrs,
                children,
            } => todo!(),
            Node::LeafElement { tag, attrs } => todo!(),
            Node::Comment { content } => todo!(),
            Node::Text { content } => todo!(),
            Node::UnsafeInlineHtml { content } => todo!(),
            Node::Script { script } => todo!(),
            Node::Nothing => todo!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::attrs::{alt, src};

    use super::*;

    fn my_list() -> Node {
        ol(
            [],
            [
                li([], ["Hello, Mercury!"]),
                li([], ["Hello, Venus!"]),
                li([], ["Hello, World!"]),
                li([], ["Hello, Mars!"]),
            ],
        )
    }

    fn my_img() -> Node {
        img([src("https://some_img.jpg"), alt("It's an image")])
    }

    #[test]
    fn example() {
        assert_eq!(
            my_list(),
            Node::Element {
                tag: "ol".into(),
                attrs: vec![],
                children: vec![
                    Node::Element {
                        tag: "li".into(),
                        attrs: vec![],
                        children: vec![Node::Text {
                            content: "Hello, Mercury!".into()
                        }]
                    },
                    Node::Element {
                        tag: "li".into(),
                        attrs: vec![],
                        children: vec![Node::Text {
                            content: "Hello, Venus!".into()
                        }]
                    },
                    Node::Element {
                        tag: "li".into(),
                        attrs: vec![],
                        children: vec![Node::Text {
                            content: "Hello, World!".into()
                        }]
                    },
                    Node::Element {
                        tag: "li".into(),
                        attrs: vec![],
                        children: vec![Node::Text {
                            content: "Hello, Mars!".into()
                        }]
                    }
                ]
            }
        );
    }
}
