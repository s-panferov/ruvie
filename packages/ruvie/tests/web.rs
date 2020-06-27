#![cfg(feature = "web")]

use wasm_bindgen_test::{wasm_bindgen_test, wasm_bindgen_test_configure};

use observe::{transaction, MutObservable, Observable, Value, Var};
use ruvie::{
	contrib::list::{IndexList, List, ListProps},
	prelude::*,
	web::{elem::Div, Cursor, Id, Web},
};

use std::{cell::Cell, rc::Rc, sync::Arc};

use rand::seq::SliceRandom;
use rand::{thread_rng, Rng};

wasm_bindgen_test_configure!(run_in_browser);

#[derive(Hash, Clone)]
struct Task {
	id: usize,
	name: String,
}

#[wasm_bindgen_test]
fn list() {
	let tasks = IndexList::from(indexmap::indexmap! {
		0 => Task {
			id: 0,
			name: "0".to_owned(),
		}
	});

	let tasks = Var::new(Arc::new(tasks));
	let list = List::with_props(ListProps {
		hint: Default::default(),
		list: Value::from(tasks.clone()),
		item: Arc::new(move |_, task| {
			Div::prop(Id, task.id.to_string())
				.child(task.name.clone())
				.build()
		}),
	})
	.build();

	let rt = ruvie::Runtime::manual(Arc::new(Web));

	let window = web_sys::window().expect("no global `window` exists");
	let document = window.document().expect("should have a document on window");
	let body = document.body().expect("document should have a body");

	let div = document.create_element("div").unwrap();
	body.append_child(&div).unwrap();

	let view = rt
		.render(list, Box::new(Cursor::beginning_of(&div).unwrap()))
		.unwrap();

	let children = div.children();

	assert_eq!(children.length(), 1);

	let mut idx: usize = 1;

	transaction(None, {
		let tasks = tasks.clone();
		move |tx| {
			tasks.modify(Some(tx), |vec| {
				let mut new_vec = (**vec).clone();
				for _ in 0..5 {
					new_vec.insert(
						idx,
						Task {
							id: idx,
							name: idx.to_string(),
						},
					);
					idx += 1;
				}
				*vec = Arc::new(new_vec);
			});
		}
	});

	assert_eq!(
		children.length(),
		1,
		"Transaction should not change anything before tick"
	);

	web_sys::console::log_1(&format!("Tick").into());

	rt.tick().unwrap();

	assert_eq!(children.length(), 6);

	assert_eq!(children.item(0).unwrap().get_attribute("id").unwrap(), "0");
	assert_eq!(children.item(1).unwrap().get_attribute("id").unwrap(), "1");
	assert_eq!(children.item(2).unwrap().get_attribute("id").unwrap(), "2");
	assert_eq!(children.item(3).unwrap().get_attribute("id").unwrap(), "3");
	assert_eq!(children.item(4).unwrap().get_attribute("id").unwrap(), "4");
	assert_eq!(children.item(5).unwrap().get_attribute("id").unwrap(), "5");

	let counter = Rc::new(Cell::new(idx));

	for _ in 0..100 {
		transaction(None, {
			let tasks = tasks.clone();
			let counter = counter.clone();
			move |tx| {
				tasks.modify(Some(tx), |vec| {
					let mut idx = counter.get().clone();

					let mut new_vec = (**vec).clone();
					let mut rng = thread_rng();

					for _ in 0..3 {
						new_vec.insert(
							idx,
							Task {
								id: idx,
								name: idx.to_string(),
							},
						);
						idx += 1;
					}

					for _ in 0..2 {
						let ridx = rng.gen_range(0, new_vec.len() - 1);
						new_vec.shift_remove_index(ridx);
					}

					let mut keys: Vec<_> = new_vec.keys().cloned().collect();
					keys.shuffle(&mut rng);
					let mut map = indexmap::IndexMap::new();

					for key in keys {
						map.insert(key, new_vec.get(&key).unwrap().clone());
					}

					*vec = Arc::new(map.into());

					counter.set(idx)
				});
			}
		});

		web_sys::console::log_1(&format!("Tick").into());

		let list = tasks.once();
		for (_, task) in list.iter() {
			web_sys::console::log_1(&format!("Expected {} {}", task.name, task.id).into())
		}

		rt.tick().unwrap();

		for (idx, (_, task)) in list.iter().enumerate() {
			assert_eq!(
				children
					.item(idx as u32)
					.unwrap()
					.get_attribute("id")
					.unwrap(),
				task.id.to_string()
			);
		}
	}

	std::mem::drop(view);
	assert_eq!(children.length(), 0);
}
