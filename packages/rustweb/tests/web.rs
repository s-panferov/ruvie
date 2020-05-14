#![cfg(feature = "dom")]

use wasm_bindgen_test::{wasm_bindgen_test, wasm_bindgen_test_configure};

use observe::{local::Value, transaction, Var};
use rustweb::{
    contrib::list::{List, ListProps, ListStore},
    dom::el::div,
    prelude::*,
};

use std::marker::PhantomData;
use std::rc::Rc;

wasm_bindgen_test_configure!(run_in_browser);

#[derive(Hash, Clone)]
struct Task {
    id: u32,
    name: String,
}

#[wasm_bindgen_test]
fn list() {
    let tasks = vec![Task {
        id: 0,
        name: "Task 0".to_owned(),
    }];

    let tasks = Value::from(Var::new(tasks));

    let list = List::new()
        .with_props(Rc::new(ListStore::new(ListProps {
            list: tasks.clone(),
            key: Box::new(|task: &Task| &task.id),
            hint: Default::default(),
            item: Box::new(move |task, refr| {
                Rc::new(
                    div()
                        .default()
                        .with_ref(refr)
                        .child(task.name.clone())
                        .build(),
                )
            }),
            _t: PhantomData,
        })))
        .build();

    let rt = rustweb::Runtime::new();

    let (node, _instance) = rt.render(Rc::new(list)).unwrap();
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let body = document.body().expect("document should have a body");

    body.append_child(&node).unwrap();

    transaction(None, move |tx| {
        let mut vec = (*tasks.once()).clone();
        vec.push(Task {
            id: 1,
            name: "Task 1".to_owned(),
        });
        vec.push(Task {
            id: 2,
            name: "Task 2".to_owned(),
        });
        tasks.set(tx, vec);
    });

    rt.tick().unwrap();
}
