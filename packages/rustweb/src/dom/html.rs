use std::rc::Rc;

use wasm_bindgen::{prelude::*, JsCast, JsValue};
use web_sys::Node;

use crate::runtime::Runtime;
use crate::{context::Mount, target::Target};

use super::{event::BoxedHandler, utils, HtmlMount};

#[derive(Clone, Debug)]
pub struct Html;

pub struct HtmlInstance {
    pub nodes: Vec<Node>,
    pub handlers: Vec<Box<dyn BoxedHandler>>,
}

impl Drop for HtmlInstance {
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
    type Mount = HtmlMount;
    type Error = JsValue;
    type Instance = HtmlInstance;
    type Result = Node;

    fn mount_component(ctx: &mut HtmlMount) -> Result<Node, JsValue> {
        let fragment = ctx.doc.create_document_fragment();

        let name = ctx.instance.spec.layout.name();

        let comment = ctx.doc.create_comment(name);
        fragment.append_child(&comment)?;
        ctx.add_node(&comment);

        utils::mount_children(ctx, &fragment)?;

        let comment = ctx.doc.create_comment(&format!("/{}", name));
        ctx.add_node(&comment);
        fragment.append_child(&comment)?;

        Ok(fragment.into())
    }

    fn mount(ctx: Mount<Html>) -> Result<(Self::Result, Mount<Html>), Self::Error> {
        let window = web_sys::window().expect("no global `window` exists");
        let document = window.document().expect("should have a document on window");

        let layout = ctx.instance.spec.layout.clone();
        let mut html = HtmlMount {
            doc: document,
            nodes: vec![],
            handlers: vec![],
            mount: ctx,
        };

        let el = layout.mount(&mut html)?;

        let HtmlMount {
            nodes, handlers, ..
        } = html;

        let mut state = html.mount.instance.state_mut();
        let platform = &mut state.platform;
        if platform.is_none() {
            *platform = Some(HtmlInstance { nodes, handlers })
        } else {
            let rt = platform.as_mut().unwrap();
            if rt.nodes.len() > 0 {
                // TODO do we need to remove nodes here if we re-create runtime anyway?
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
            *platform = Some(HtmlInstance { nodes, handlers })
        }

        std::mem::drop(platform);
        std::mem::drop(state);

        Ok((el, html.mount))
    }

    fn tick(scheduler: Rc<Runtime<Html>>) {
        let callback = Closure::once(move || scheduler.tick().unwrap());

        web_sys::window()
            .unwrap()
            .request_animation_frame(callback.as_ref().unchecked_ref())
            .unwrap();

        // FIXME
        callback.forget();
    }
}
