use std::collections::HashSet;
use std::{iter::FromIterator, rc::Rc};

use wasm_bindgen::{prelude::*, JsCast, JsValue};
use web_sys::Node;

use crate::instance::Instance;
use crate::runtime::Runtime;
use crate::{
    mount::{Mount, Reactions},
    target::Target,
    AfterRender,
};

use super::{utils, HtmlMount};

#[derive(Clone, Debug)]
pub struct Html;

pub struct HtmlRuntime {
    pub nodes: Vec<Node>,
}

impl Drop for HtmlRuntime {
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
    type Runtime = HtmlRuntime;
    type Result = Node;

    fn mount_component(ctx: &mut Mount<Html>) -> Result<Node, JsValue> {
        let fragment = ctx.doc.create_document_fragment();

        let comment = ctx.doc.create_comment("Component");
        fragment.append_child(&comment)?;
        ctx.add_node(&comment);

        utils::mount_children(ctx, &fragment)?;

        let comment = ctx.doc.create_comment("/Component");
        ctx.add_node(&comment);
        fragment.append_child(&comment)?;

        Ok(fragment.into())
    }

    fn mount(instance: Rc<Instance<Html>>) -> Result<Self::Result, Self::Error> {
        let window = web_sys::window().expect("no global `window` exists");
        let document = window.document().expect("should have a document on window");

        let tree = instance.state_mut().rendered_tree.take();
        let layout = instance.spec.layout.clone();
        let mut ctx = Mount {
            children: vec![],
            reactions: Reactions::new(Rc::downgrade(&instance)),
            instance: instance.clone(),
            tree,
            platform: HtmlMount {
                doc: document,
                nodes: vec![],
            },
        };

        let el = layout.mount(&mut ctx)?;

        let Mount {
            children,
            reactions,
            platform,
            ..
        } = ctx;

        let HtmlMount { nodes, .. } = platform;

        let mut state = instance.state_mut();
        if state.runtime.is_none() {
            state.runtime = Some(HtmlRuntime { nodes })
        } else {
            let rt = state.runtime.as_mut().unwrap();
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
            state.runtime = Some(HtmlRuntime { nodes })
        }

        state.children = children;

        let Reactions { reactions, .. } = reactions;

        // mark all reactions as updated
        state.invalidated_updates = HashSet::from_iter(reactions.iter().map(|c| c.weak()));
        state.all_update_reactions = reactions;
        std::mem::drop(state);

        if let Some(refr) = instance.spec.layout.get_ref() {
            refr.apply(Rc::downgrade(&instance))
        }

        let ctx = instance.after_render();

        let AfterRender { reactions, .. } = ctx;
        let Reactions { reactions, .. } = reactions;

        let mut state = instance.state_mut();

        // mark all reactions as updated
        state
            .invalidated_updates
            .extend(reactions.iter().map(|c| c.weak()));

        state.all_update_reactions.extend(reactions);

        Ok(el)
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
