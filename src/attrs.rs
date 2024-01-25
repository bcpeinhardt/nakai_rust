use convert_case::{Case, Casing};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Attr {
    Attr { name: String, value: String },
}

macro_rules! attr {
    ($($i:ident),+) => {
        $(pub fn $i(value: impl Into<String>) -> Attr {
            Attr::Attr { name: stringify!($i).to_case(Case::Kebab), value: value.into() }
        })+
    };
}

macro_rules! bool_attr {
    ($($i:ident),+) => {
        $(pub fn $i() -> Attr {
            Attr::Attr { name: stringify!($i).to_case(Case::Kebab), value: "true".into() }
        })+
    };
}

attr!(
    accept,
    accept_charset,
    action,
    alt,
    autocapitalize,
    autocomplete,
    capture,
    charset,
    cite,
    class,
    content,
    formaction,
    height,
    href,
    http_equiv,
    id,
    integrity,
    lang,
    method,
    name,
    placeholder,
    property,
    rel,
    src,
    style,
    tabindex,
    target,
    title,
    value,
    width,
    hx_boost,
    hx_get,
    hx_post,
    hx_push_url,
    hx_select,
    hx_select_oob,
    hx_target,
    hx_trigger,
    hx_vals,
    hx_confirm,
    hx_delete,
    hx_disable,
    hx_disable_elt,
    hx_disinherit,
    hx_encoding,
    hx_ext,
    hx_headers,
    hx_history,
    hx_history_elt,
    hx_include,
    hx_indicator,
    hx_params,
    hx_patch,
    hx_preserve,
    hx_prompt,
    hx_put,
    hx_replace_url,
    hx_request,
    hx_sse,
    hx_sync,
    hx_validate,
    hx_vars,
    hx_ws
);

bool_attr!(
    autofocus,
    autoplay,
    checked,
    contenteditable,
    crossorigin,
    defer,
    disabled,
    draggable,
    preload,
    readonly,
    selected
);

pub fn for_(value: impl Into<String>) -> Attr {
    Attr::Attr {
        name: "for".into(),
        value: value.into(),
    }
}

pub fn type_(value: impl Into<String>) -> Attr {
    Attr::Attr {
        name: "type".into(),
        value: value.into(),
    }
}

pub fn async_() -> Attr {
    Attr::Attr {
        name: "type".into(),
        value: "true".into(),
    }
}

pub fn loop_() -> Attr {
    Attr::Attr {
        name: "type".into(),
        value: "true".into(),
    }
}

/// Create a data attribute with format `data-<name>="value"`
pub fn data_attr(name: impl AsRef<str>, value: impl Into<String>) -> Attr {
    Attr::Attr {
        name: ["data-", name.as_ref()].concat(),
        value: value.into(),
    }
}

pub fn hx_on(name: impl AsRef<str>, value: impl Into<String>) -> Attr {
    Attr::Attr {
        name: ["hx-on:", name.as_ref()].concat(),
        value: value.into(),
    }
}

impl Attr {
    pub fn render(self) -> String {
        let Attr::Attr { name, value } = self;

        // Sanitize the value
        let sanitized_value = value.replace("\"", "&quot;").replace(">", "&gt;");

        // Build
        let mut buf = String::new();
        buf.push_str(&name);
        buf.push_str("=\"");
        buf.push_str(&sanitized_value);
        buf.push('\"');
        buf
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn class_attr() {
        let class_attr = class("test-class another");

        // Constructs correctly
        assert_eq!(
            class_attr,
            Attr::Attr {
                name: "class".into(),
                value: "test-class another".into()
            }
        );

        // Stringies correctly
        assert_eq!(class_attr.render(), "class=\"test-class another\"");
    }
}
