use observe::local::Value;
use wasm_bindgen::JsValue;
use web_sys::Node;

use crate::component::Component;
use crate::layout::Layout;
use crate::{mount::Mount, Children};

use super::Html;

pub struct TextProps {
    value: Value<String>,
}

pub struct Text {}

impl Component for Text {
    type Props = TextProps;
    type Target = Html;

    fn mount(&self, ctx: &mut Mount<Html>) -> Result<Node, JsValue> {
        let el = ctx.doc.create_text_node("EMPTY");
        ctx.add_node(&el);
        ctx.reactions.add(self, {
            let el = el.clone();
            move |ctx| {
                let text = ctx.props.value.observe(ctx.eval);
                el.set_node_value(Some(&text));
                Ok(())
            }
        });

        Ok(el.into())
    }

    fn name(&self) -> &'static str {
        return "Text";
    }
}

impl From<String> for Layout<Text> {
    fn from(value: String) -> Self {
        Layout {
            reference: None,
            component: Text {},
            props: TextProps {
                value: value.into(),
            },
            children: None.into(),
        }
    }
}

impl From<&str> for Layout<Text> {
    fn from(value: &str) -> Self {
        Layout {
            reference: None,
            component: Text {},
            props: TextProps {
                value: value.to_owned().into(),
            },
            children: None.into(),
        }
    }
}

impl From<Value<String>> for Children<Html> {
    fn from(value: Value<String>) -> Self {
        Layout {
            reference: None,
            component: Text {},
            props: TextProps { value },
            children: None.into(),
        }
        .into()
    }
}

impl From<String> for Children<Html> {
    fn from(value: String) -> Self {
        Layout::from(value).into()
    }
}

impl From<&str> for Children<Html> {
    fn from(value: &str) -> Self {
        Layout::from(value).into()
    }
}
