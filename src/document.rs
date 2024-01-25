pub struct Document {
    pub doctype: Option<String>,
    pub html_attrs: String,
    pub body_attrs: String,
    pub head: String,
    pub body: String,
    pub scripts: Vec<String>,
}

pub const ENCODING: &'static str = "
<meta charset=\"utf-8\" />
<meta http-equiv=\"content-type\" content=\"text/html; charset=utf-8\" />
";

impl Document {
    pub fn new() -> Self {
        Self {
            doctype: None,
            html_attrs: String::new(),
            body_attrs: String::new(),
            head: String::new(),
            body: String::new(),
            scripts: vec![],
        }
    }

    pub fn merge(self, rhs: Self) -> Self {
        Self {
            // The doctype of the rhs overrides the original.
            doctype: rhs.doctype.or(self.doctype),
            html_attrs: [self.html_attrs, rhs.html_attrs].concat(),
            body_attrs: [self.body_attrs, rhs.body_attrs].concat(),
            head: [self.head, rhs.head].concat(),
            body: [self.body, rhs.body].concat(),
            scripts: vec![self.scripts, rhs.scripts]
                .into_iter()
                .flatten()
                .collect(),
        }
    }

    pub fn concat(docs: Vec<Self>) -> Self {
        docs.into_iter().fold(Self::new(), Self::merge)
    }

    pub fn from_doctype(doctype: String) -> Self {
        Self {
            doctype: Some(doctype),
            ..Self::new()
        }
    }

    pub fn from_body(body: String) -> Self {
        Self {
            body,
            ..Self::new()
        }
    }

    pub fn from_script(script: String) -> Self {
        Self {
            scripts: vec![script],
            ..Self::new()
        }
    }

    pub fn append_html_attrs(&mut self, html_attrs: impl AsRef<str>) {
        self.html_attrs.push_str(html_attrs.as_ref());
    }

    pub fn append_body_attrs(&mut self, body_attrs: impl AsRef<str>) {
        self.body_attrs.push_str(body_attrs.as_ref());
    }

    /// Takes what's in the document body and moves it into the head.
    /// TODO: Document usage pattern
    pub fn into_head(mut self) -> Self {
        self.head.push_str(&self.body);
        self.body.clear();
        self
    }
}

impl Default for Document {
    fn default() -> Self {
        Self::new()
    }
}
