use wasm_bindgen::JsValue;
use web_sys::Node;

use super::{utils, Html};
use crate::{
    component::{Component, ComponentExt},
    props::PropFor,
    Props,
};

pub use super::{
    event::bind, text::*, Class, ContentEditable, HtmlMount, HtmlProps, OnBeforeInput, OnClick,
    Style,
};

macro_rules! attr {
    ($el:expr, $x:expr, $eval:expr, $where:expr) => {
        let prop = &$where;
        if !prop.is_empty() {
            let value = prop.observe($eval);
            $el.set_attribute(
                $x,
                &(*value)
                    .as_ref()
                    .map(|v| v.to_string())
                    .unwrap_or(String::from("")),
            )?
        }
    };
}

#[derive(Debug)]
pub struct Div {}

pub fn div() -> Div {
    Div {}
}

impl PropFor<Div> for Style {}
impl PropFor<Div> for Class {}
impl PropFor<Div> for ContentEditable {}
impl PropFor<Div> for OnClick {}
impl PropFor<Div> for OnBeforeInput {}

impl Component for Div {
    type Props = Props<Self>;
    type Target = Html;

    fn mount(&self, ctx: &mut HtmlMount) -> Result<Node, JsValue> {
        let el = ctx.doc.create_element("div")?;
        ctx.add_node(&el);

        let props = Self::props(ctx);
        for prop in &props.props {
            if let Some((_, ev)) = prop.downcast::<OnClick>() {
                ctx.add_handler(Box::new(bind(ev, &el, "click")?));
            } else if let Some((_, ev)) = prop.downcast::<OnBeforeInput>() {
                ctx.add_handler(Box::new(bind(ev, &el, "beforeinput")?));
            }
        }

        utils::mount_children(ctx, &el)?;

        ctx.reaction(Self::reaction({
            let el = el.clone();
            move |_, ctx| {
                for prop in &props.props {
                    if let Some((_, style)) = prop.downcast::<Style>() {
                        attr!(el, "style", &mut ctx.eval, style);
                    } else if let Some((_, class)) = prop.downcast::<Class>() {
                        attr!(el, "class", &mut ctx.eval, class);
                    } else if let Some((_, contenteditable)) = prop.downcast::<ContentEditable>() {
                        attr!(el, "contenteditable", &mut ctx.eval, contenteditable);
                    }
                }
                Ok(())
            }
        }));

        Ok(el.into())
    }
}
