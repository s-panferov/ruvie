use std::collections::HashSet;
use std::iter::FromIterator;

use wasm_bindgen::{prelude::*, JsCast, JsValue};
use web_sys::Document;
use web_sys::Node;

use crate::component::{Component, Target, UpdateContext, UpdateContextTyped};
use crate::instance::Instance;
use crate::instance::InstanceRef;
use crate::instance::WeakInstance;
use crate::layout::Children;
use crate::scheduler::{Scheduler, WeakScheduler};

use super::utils;
use observe::{EvalContext, Tracker};

#[derive(Clone, Debug)]
pub struct Html;

pub struct Runtime {
    pub nodes: Vec<Node>,
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

pub struct MountContext {
    pub doc: Document,
    instance: InstanceRef<Html>,
    pub(crate) scheduler: Scheduler<Html>,
    pub(crate) parent: Option<WeakInstance<Html>>,
    pub(crate) nodes: Vec<Node>,
    pub(crate) children: Vec<InstanceRef<Html>>,
    pub(crate) reactions: Vec<Tracker>,
}

impl MountContext {
    pub(crate) fn add_node(&mut self, node: &Node) {
        self.nodes.push(node.clone());
    }

    pub(crate) fn add_child(&mut self, child: InstanceRef<Html>) {
        self.children.push(child);
    }

    pub(crate) fn reaction<F, C: Component<Target = Html>>(&mut self, _: &C, handler: F)
    where
        F: Fn(UpdateContextTyped<C>) -> Result<(), JsValue>,
        F: 'static,
    {
        let rx = Tracker::new("Update Reaction".to_owned());

        let mut rx_mut = rx.get_mut();
        let instance_1 = self.instance.weak();
        let instance_2 = self.instance.weak();

        rx_mut.set_is_observer();
        rx_mut.on_reaction({
            let rx = rx.weak();
            move || {
                if let Some(instance) = instance_1.upgrade() {
                    // FIXME move to instance
                    instance.get_mut().updated_rx.insert(rx.clone());
                    // FIXME triggers several times, need to optimize in observe
                    instance.schedule_update()
                } else {
                    unreachable!()
                }
            }
        });

        rx_mut.set_computation({
            move |eval: &mut EvalContext| {
                if let Some(inst) = instance_2.upgrade() {
                    let i = inst.get();
                    let ctx = UpdateContext { eval, instance: &i }.typed::<C>();
                    let res = handler(ctx);
                    std::mem::drop(i);
                    inst.get_mut().update_res = res;
                } else {
                    unreachable!()
                }
                0
            }
        });

        std::mem::drop(rx_mut);
        self.reactions.push(rx)
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

        utils::mount_children(ctx, children, &fragment)?;

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
            instance: instance.strong_ref(),
            parent: Some(instance.weak_ref().clone()),
            scheduler: instance.opts.scheduler.clone(),
            nodes: vec![],
            children: vec![],
            reactions: vec![],
        };

        let el = instance.opts.layout.mount(&mut ctx, instance.tree.take())?;

        let MountContext {
            children,
            nodes,
            reactions,
            ..
        } = ctx;

        instance.children = children;

        // mark all reactions as updated
        instance.updated_rx = HashSet::from_iter(reactions.iter().map(|c| c.weak()));
        instance.update_rx = reactions;

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
