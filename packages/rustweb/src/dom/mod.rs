use observe::Value;
use std::collections::BTreeMap;
use wasm_bindgen::{prelude::*, JsCast, JsValue};
use web_sys::Document;
use web_sys::Element;
use web_sys::HtmlElement;
use web_sys::Node;

use crate::component::UpdateContext;
use crate::component::{Component, Target};
use crate::instance::Instance;
use crate::instance::InstanceOptions;
use crate::instance::InstanceRef;
use crate::instance::WeakInstance;
use crate::layout::Children;
use crate::layout::Layout;
use crate::scheduler::{Scheduler, WeakScheduler};

pub use render::{node, render};

mod render;

#[derive(Clone, Debug)]
pub struct Html;

pub struct Runtime {
    nodes: Vec<Node>,
}

impl Drop for Runtime {
    fn drop(&mut self) {
        for n in &self.nodes {
            let parent = n.parent_node();
            if let Some(parent) = parent {
                let _ = parent.remove_child(n);
            }
        }
    }
}

impl Target for Html {
    type MountContext = MountContext;
    type Error = JsValue;
    type Runtime = Runtime;
    type Result = Node;

    fn component(ctx: &mut MountContext, children: Children<Html>) -> Result<Node, JsValue> {
        let fragment = ctx.doc.create_document_fragment();

        let comment = ctx.doc.create_comment("Component");
        fragment.append_child(&comment)?;
        ctx.add_node(&comment);

        if children.is_some() {
            for c in children.unwrap().into_iter() {
                let instance = InstanceRef::new(InstanceOptions {
                    parent: ctx.parent.clone(),
                    scheduler: ctx.scheduler.clone(),
                    layout: c,
                    level: ctx.parent.as_ref().map(|p| p.level).unwrap_or(0) + 1,
                });

                fragment.append_child(&instance.perform_render()?)?;
                ctx.add_child(instance);
            }
        }

        let comment = ctx.doc.create_comment("/Component");
        ctx.add_node(&comment);
        fragment.append_child(&comment)?;

        Ok(fragment.into())
    }

    fn mount(instance: &mut Instance<Html>) -> Result<Self::Result, Self::Error> {
        let window = web_sys::window().expect("no global `window` exists");
        let document = window.document().expect("should have a document on window");
        let mut ctx = MountContext {
            doc: document,
            parent: Some(instance.weak_ref().clone()),
            scheduler: instance.opts.scheduler.clone(),
            nodes: vec![],
            children: vec![],
        };

        let el = instance.opts.layout.mount(&mut ctx, instance.tree.take())?;

        let MountContext {
            children, nodes, ..
        } = ctx;

        instance.children = children;

        if instance.runtime.is_none() {
            instance.runtime = Some(Runtime { nodes })
        } else {
            let rt = instance.runtime.as_mut().unwrap();
            if rt.nodes.len() > 0 {
                // TODO do we need to remove nodes here is re-create runtime anyway?
                for (i, el) in rt.nodes.iter().enumerate() {
                    if i > 0 {
                        if let Some(parent) = el.parent_node() {
                            parent.remove_child(&el)?;
                        }
                    }
                }

                let first = &rt.nodes[0];
                first.parent_node().unwrap().replace_child(&el, &first)?;
            }
            instance.runtime = Some(Runtime { nodes })
        }

        Ok(el)
    }

    fn tick(scheduler: WeakScheduler<Html>) {
        let callback = Closure::once(move || {
            let scheduler = scheduler.upgrade();
            match scheduler {
                Some(scheduler) => scheduler.get_mut().tick().unwrap(),
                None => {}
            };
        });

        web_sys::window()
            .unwrap()
            .request_animation_frame(callback.as_ref().unchecked_ref())
            .unwrap();

        // FIXME
        callback.forget();
    }
}

impl<C: Component<Target = Html> + 'static> Layout<C> {}

// impl<P, C: Component<P, Html>> HtmlRenderer<P> for Instance<C, P, Html> {
//     fn create(&self, props: &P, ctx: &RenderContext) -> Result<Element, JsValue> {}
// }

#[derive(Debug)]
pub struct Div {}

#[derive(Default, Debug)]
pub struct HtmlProps {
    pub attributes: Value<BTreeMap<String, String>>,
}

impl Component for Div {
    type Props = HtmlProps;
    type Target = Html;

    fn mount(&self, ctx: &mut MountContext, children: Children<Html>) -> Result<Node, JsValue> {
        let el = ctx.doc.create_element("div")?;
        ctx.add_node(&el);

        if children.is_some() {
            for c in children.unwrap().into_iter() {
                let instance = InstanceRef::new(InstanceOptions {
                    parent: ctx.parent.clone(),
                    scheduler: ctx.scheduler.clone(),
                    layout: c,
                    level: ctx.parent.as_ref().map(|p| p.level).unwrap_or(0) + 1,
                });

                el.append_child(&instance.perform_render()?)?;
                ctx.add_child(instance);
            }
        }
        Ok(el.into())
    }

    fn update(&self, ctx: UpdateContext<HtmlProps>) -> Result<(), JsValue> {
        let UpdateContext { props, .. } = ctx;

        let el: Element = node(&ctx).unwrap().unchecked_into();
        for (k, v) in props.attributes.observe(ctx.eval).iter() {
            el.set_attribute(k, v)?
        }

        Ok(())
    }
}

pub struct MountContext {
    pub doc: Document,

    scheduler: Scheduler<Html>,
    parent: Option<WeakInstance<Html>>,

    nodes: Vec<Node>,
    children: Vec<InstanceRef<Html>>,
}

impl MountContext {
    fn add_node(&mut self, node: &Node) {
        self.nodes.push(node.clone());
    }

    fn add_child(&mut self, child: InstanceRef<Html>) {
        self.children.push(child);
    }
}

pub fn div() -> Div {
    Div {}
}

pub struct TextProps {
    value: Value<String>,
}

pub struct Text {}

impl Component for Text {
    type Props = TextProps;
    type Target = Html;

    fn mount(&self, ctx: &mut MountContext, _children: Children<Html>) -> Result<Node, JsValue> {
        let el = ctx.doc.create_text_node("");
        ctx.add_node(&el);
        Ok(el.into())
    }

    fn update(&self, ctx: UpdateContext<Self::Props>) -> Result<(), JsValue> {
        let UpdateContext { props, .. } = ctx;

        let el: HtmlElement = node(&ctx).unwrap().unchecked_into();
        el.set_inner_text(&props.value.observe(ctx.eval));

        Ok(())
    }

    fn name(&self) -> &'static str {
        return "Text";
    }
}

impl From<String> for Layout<Text> {
    fn from(value: String) -> Self {
        Layout {
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
            component: Text {},
            props: TextProps {
                value: value.to_owned().into(),
            },
            children: None.into(),
        }
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
