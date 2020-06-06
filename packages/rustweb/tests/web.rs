#![cfg(feature = "web")]

use wasm_bindgen_test::{wasm_bindgen_test, wasm_bindgen_test_configure};

use observe::{transaction, MutObservable, Observable, Value, Var};
use rustweb::{
	contrib::list::{IndexList, List, ListProps},
	prelude::*,
	web::{elem::Div, Cursor, Id, Web},
};

use std::sync::Arc;

use rand::seq::SliceRandom;
use rand::thread_rng;

wasm_bindgen_test_configure!(run_in_browser);

#[derive(Hash, Clone)]
struct Task {
	id: u32,
	name: String,
}

#[wasm_bindgen_test]
fn list() {
	let tasks = IndexList::from(indexmap::indexmap! {
		0 => Task {
			id: 0,
			name: "A".to_owned(),
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

	let rt = rustweb::Runtime::<Web>::manual();

	let window = web_sys::window().expect("no global `window` exists");
	let document = window.document().expect("should have a document on window");
	let body = document.body().expect("document should have a body");

	let div = document.create_element("div").unwrap();
	body.append_child(&div).unwrap();

	let view = rt
		.render(list, Cursor::beginning_of(&div).unwrap())
		.unwrap();

	let children = div.children();

	assert_eq!(children.length(), 1);

	transaction(None, {
		let tasks = tasks.clone();
		move |tx| {
			tasks.modify(Some(tx), |vec| {
				let mut new_vec = (**vec).clone();
				new_vec.insert(
					1,
					Task {
						id: 1,
						name: "B".to_owned(),
					},
				);
				new_vec.insert(
					2,
					Task {
						id: 2,
						name: "C".to_owned(),
					},
				);
				new_vec.insert(
					3,
					Task {
						id: 3,
						name: "D".to_owned(),
					},
				);
				new_vec.insert(
					4,
					Task {
						id: 4,
						name: "E".to_owned(),
					},
				);
				new_vec.insert(
					5,
					Task {
						id: 5,
						name: "F".to_owned(),
					},
				);
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

	// transaction(None, {
	// 	let tasks = tasks.clone();
	// 	move |tx| {
	// 		tasks.modify(Some(tx), |vec| {
	// 			let mut new_vec = (**vec).clone();
	// 			let item = new_vec.remove(0);
	// 			new_vec.push(item);
	// 			*vec = Arc::new(new_vec);
	// 		});
	// 	}
	// });

	// web_sys::console::log_1(&format!("Tick").into());
	// rt.tick().unwrap();

	// let list = tasks.once();
	// for task in list.iter() {
	// 	web_sys::console::log_1(&format!("Expected {}", task.id).into())
	// }

	// for (idx, task) in list.iter().enumerate() {
	// 	assert_eq!(
	// 		children
	// 			.item(idx as u32)
	// 			.unwrap()
	// 			.get_attribute("id")
	// 			.unwrap(),
	// 		task.id.to_string()
	// 	);
	// }

	// std::mem::drop(list);

	// transaction(None, {
	// 	let tasks = tasks.clone();
	// 	move |tx| {
	// 		tasks.modify(Some(tx), |vec| {
	// 			let new_vec = IndexList::from(indexmap::indexmap! {
	// 				3 => Task {
	// 					id: 3,
	// 					name: "D".to_owned(),
	// 				},
	// 				1 => Task {
	// 					id: 1,
	// 					name: "B".to_owned(),
	// 				},
	// 				4 => Task {
	// 					id: 4,
	// 					name: "E".to_owned(),
	// 				},
	// 				5 => Task {
	// 					id: 5,
	// 					name: "F".to_owned(),
	// 				},
	// 				2 => Task {
	// 					id: 2,
	// 					name: "C".to_owned(),
	// 				},
	// 				0 => Task {
	// 					id: 0,
	// 					name: "A".to_owned(),
	// 				}
	// 			});
	// 			*vec = Arc::new(new_vec);
	// 		});
	// 	}
	// });

	transaction(None, {
		let tasks = tasks.clone();
		move |tx| {
			tasks.modify(Some(tx), |vec| {
				let mut keys: Vec<_> = vec.keys().cloned().collect();
				keys.shuffle(&mut thread_rng());
				let mut map = indexmap::IndexMap::new();
				for key in keys {
					map.insert(key, vec.get(&key).unwrap().clone());
				}

				*vec = Arc::new(map.into());
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

	std::mem::drop(view);

	assert_eq!(children.length(), 0);
}
