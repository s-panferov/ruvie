#![allow(non_snake_case)]

use std::task::Poll;
use std::{marker::PhantomData, rc::Rc};

use wasm_bindgen::{prelude::*, JsCast, JsValue};

use observe::{local::Value, transaction, Computed, Var};

use rustweb::{
    context::Render,
    contrib::list::{List, ListProps, ListStore},
    dom::{el::div, Class, ClassList, Html, Style},
    prelude::*,
    Children,
};

use store::AppStore;
use store::{AppProps, Task, Theme};

mod api;
mod error;
mod store;

fn App(ctx: &mut Render<AppStore, Html>) -> Children<Html> {
    let store = ctx.props;
    let payload = store.data.clone();

    div()
        .prop(Style, store.style.clone())
        .prop(Class, ClassList::new(vec!["test".to_owned()]))
        .scope(move |_ctx| {
            let payload = payload.clone();
            Value::from(Computed::new(move |eval| match &*payload.observe(eval) {
                Poll::Ready(_v) => format!("DATA: {:?}", _v),
                Poll::Pending => String::from("Loading"),
            }))
        })
        .child(List::new().with_props(Rc::new(ListStore::new(ListProps {
            list: store.tasks.clone(),
            key: Box::new(|task: &Task| &task.id),
            hint: Default::default(),
            item: Box::new(move |task, refr| {
                Rc::new(
                    div()
                        .default()
                        .with_ref(refr)
                        .child(task.title.clone())
                        .build(),
                )
            }),
            _t: PhantomData,
        }))))
        // .child(Button.create().default().child("Test"))
        // .scope(move |_| "Test")
        .into()
}

#[wasm_bindgen(start)]
pub fn run() -> Result<(), JsValue> {
    console_error_panic_hook::set_once();

    let x: Value<_> = 0.into();
    let y: Value<_> = 0.into();
    let theme = Value::from(Var::new(Theme::Square));

    let store = AppStore::new(AppProps {
        theme: theme.clone(),
        x: x.clone(),
        y: y.clone(),
    });

    let store1 = store.clone();
    let app = App.create().with_props(store).build();

    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let body = document.body().expect("document should have a body");

    let mousemove = Closure::wrap(Box::new(move |event: web_sys::MouseEvent| {
        transaction(None, |tx| {
            x.set(tx, event.x());
            y.set(tx, event.y());
        });
    }) as Box<dyn FnMut(_)>);

    let click = Closure::wrap(Box::new(move |_ev: web_sys::MouseEvent| {
        transaction(None, |tx| {
            let mut tasks = (*store1.tasks.once()).clone();
            let uuid = uuid::Uuid::new_v4();
            let name = format!("Task {}", uuid);
            tasks.push(Task {
                id: uuid,
                title: name,
                completed: false,
            });
            let uuid = uuid::Uuid::new_v4();
            let name = format!("Task {}", uuid);
            tasks.push(Task {
                id: uuid,
                title: name,
                completed: false,
            });
            store1.tasks.set(tx, tasks)
        });
        transaction(None, |tx| {
            let current = theme.once();
            theme.set(
                tx,
                match *current {
                    Theme::Circle => Theme::Square,
                    Theme::Square => Theme::Circle,
                },
            );
        });
    }) as Box<dyn FnMut(_)>);

    document.add_event_listener_with_callback("mousemove", mousemove.as_ref().unchecked_ref())?;
    document.add_event_listener_with_callback("click", click.as_ref().unchecked_ref())?;

    mousemove.forget();
    click.forget();

    let rt = rustweb::Runtime::new();

    let (node, instance) = rt.render(Rc::new(app))?;
    body.append_child(&node)?;

    Box::leak(Box::new(instance));

    Ok(())
}
