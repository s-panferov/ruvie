use js_sys::Object;
use std::rc::Rc;
use wasm_bindgen::JsCast;
use web_sys::{Comment, Document, DocumentFragment, Element, Node};

pub enum Child {
	Node(Node),
	Fragment(Rc<PersistedFragment>),
}

pub struct PersistedFragment {
	kind: FragmentKind,
	children: Vec<Child>,
	name: Option<String>,
}

enum FragmentKind {
	Empty(Comment),
	Multi(Comment, Comment),
	Proxy,
}

fn fragment_name(name: &Option<String>) -> &str {
	name.map(|v| v.as_ref()).unwrap_or("Fragment")
}

impl Child {
	fn is_document_fragment(&self) -> bool {
		match self {
			Child::Fragment(frag) => false,
			Child::Node(node) => node.has_type::<DocumentFragment>(),
		}
	}

	fn left(&self) -> &Node {
		match self {
			Child::Fragment(frag) => frag.left(),
			Child::Node(node) => node,
		}
	}

	fn right(&self) -> &Node {
		match self {
			Child::Fragment(frag) => frag.right(),
			Child::Node(node) => node,
		}
	}

	fn insert_before(&self, parent: &Node, before: Option<&Node>) {
		match self {
			Child::Node(node) => {
				parent.insert_before(&node, before).unwrap();
			}
			Child::Fragment(frag) => frag.insert_self_before(parent, before),
		}
	}

	fn replace_with(&self, child: Child) {
		match self {
			Child::Node(node) => match child {
				Child::Node(child) => {
					let parent = node.parent_node().unwrap();
					parent.replace_child(&node, &child);
				}
				Child::Fragment(child) => {
					let parent = node.parent_node().unwrap();
					child.insert_self_before(&parent, Some(&node));
					parent.remove_child(&node);
				}
			},
			Child::Fragment(frag) => frag.replace_self_with(child),
		}
	}

	fn remove(&self, parent: Option<&Node>) {
		match self {
			Child::Node(node) => {
				parent
					.unwrap_or_else(|| &node.parent_node().unwrap())
					.remove_child(&node);
			}
			Child::Fragment(frag) => frag.remove(),
		}
	}
}

type RenderFragment = Node;

impl PersistedFragment {
	fn new(
		doc: &Document,
		parent: &DocumentFragment,
		name: Option<String>,
		children: Vec<Child>,
	) -> PersistedFragment {
		if children.len() == 0 {
			let comment = doc.create_comment(&format!("<{}/>", fragment_name(&name)));
			PersistedFragment {
				kind: FragmentKind::Empty(comment),
				children: vec![],
				name,
			}
		} else if children.len() == 1 && !children[0].is_document_fragment() {
			PersistedFragment {
				kind: FragmentKind::Proxy,
				children,
				name,
			}
		} else {
			let comment_start = doc.create_comment(&format!("<{}>", fragment_name(&name)));
			let comment_end = doc.create_comment(&format!("</{}>", fragment_name(&name)));
			PersistedFragment {
				kind: FragmentKind::Multi(comment_start, comment_end),
				children,
				name,
			}
		}
	}

	pub fn insert_self_before(&self, parent: &Node, before: Option<&Node>) {
		match self.kind {
			FragmentKind::Multi(left, right) => {
				parent.insert_before(&left, before);
				for node in self.children {
					node.insert_before(parent, before)
				}
				parent.insert_before(&right, before);
			}
			FragmentKind::Empty(comment) => {
				parent.insert_before(&comment, before);
			}
			FragmentKind::Proxy => {
				self.children[0].insert_before(parent, before);
			}
		}
	}

	pub fn first_node(&self) -> Option<&Node> {
		todo!()
	}

	pub fn left(&self) -> &Node {
		match self.kind {
			FragmentKind::Multi(left, _) => left.unchecked_ref::<Node>(),
			FragmentKind::Empty(left) => left.unchecked_ref::<Node>(),
			FragmentKind::Proxy => self.children[0].left(),
		}
	}

	pub fn right(&self) -> &Node {
		match self.kind {
			FragmentKind::Multi(_, right) => right.unchecked_ref::<Node>(),
			FragmentKind::Empty(right) => right.unchecked_ref::<Node>(),
			FragmentKind::Proxy => self.children[0].right(),
		}
	}

	pub fn remove_between(&self) {
		if let FragmentKind::Multi(left, right) = self.kind {
			remove_between(&left, &right)
		}
	}

	pub fn remove(&self) {
		match self.kind {
			FragmentKind::Multi(left, right) => {
				remove_between(&left, &right);
				left.remove();
				right.remove();
			}
			FragmentKind::Empty(comment) => comment.remove(),
			FragmentKind::Proxy => {
				assert!(self.children.len() == 1);
				self.children[0].remove(None);
			}
		}
	}

	pub fn replace_self_with(&mut self, child: Child) {
		match self.kind {
			FragmentKind::Multi(left, _) => {
				let parent = left.parent_node().unwrap();
				child.insert_before(&parent, Some(&left));
				self.remove()
			}
			FragmentKind::Empty(comment) => {
				let parent = comment.parent_node().unwrap();
				child.insert_before(&parent, Some(&comment));
				self.remove()
			}
			FragmentKind::Proxy => self.children[0].replace_with(child),
		}
	}

	pub fn replace_children(&mut self, doc: &Document, children: Vec<Child>) {
		if children.len() == 0 {
			match self.kind {
				FragmentKind::Multi(comment1, comment2) => {
					for child in self.children {
						child.remove(None);
					}
					comment1.set_data(&format!("<{}/>", fragment_name(&self.name)));
					comment2.remove();
				}
				FragmentKind::Empty(_) => assert!(self.children.len() == 0),
				FragmentKind::Proxy => {
					assert!(self.children.len() == 1);
					let child = self.children[0];
					let comment = doc.create_comment(&format!("<{}/>", fragment_name(&self.name)));
					child.replace_with(Child::Node(comment.clone().unchecked_into::<Node>()));
					self.kind = FragmentKind::Empty(comment);
				}
			}
		} else if children.len() == 1 && !children[0].is_document_fragment() {
			// DocumentFragment can contain any amount of nodes, so we treat it like multi-node
			match self.kind {
				FragmentKind::Multi(comment1, comment2) => {
					for child in self.children {
						child.remove(None)
					}

					let parent = comment1.parent_node().unwrap();
					children[0].insert_before(&parent, Some(&comment1));

					comment1.remove();
					comment2.remove();
					self.kind = FragmentKind::Proxy;
				}
				FragmentKind::Empty(comment) => {
					let parent = comment.parent_node().unwrap();
					children[0].insert_before(&parent, Some(&comment));
					comment.remove();
					self.kind = FragmentKind::Proxy;
				}
				FragmentKind::Proxy => {}
			}
		} else {
			match self.kind {
				FragmentKind::Multi(comment1, comment2) => {
					for child in self.children {
						child.remove(None)
					}
					let parent = comment1.parent_node().unwrap();
					for child in children {
						child.insert_before(&parent, Some(&comment2));
					}
				}
				FragmentKind::Empty(comment) => {
					let parent = comment.parent_node().unwrap();

					let comment_start =
						doc.create_comment(&format!("<{}>", fragment_name(&self.name)));

					let comment_end =
						doc.create_comment(&format!("</{}>", fragment_name(&self.name)));

					parent
						.insert_before(&comment_start, Some(&comment))
						.unwrap();

					for child in children {
						child.insert_before(&parent, Some(&comment));
					}

					parent.insert_before(&comment_end, Some(&comment)).unwrap();
					comment.remove();
					self.kind = FragmentKind::Multi(comment_start, comment_end);
				}
				FragmentKind::Proxy => {
					assert!(self.children.len() == 1, "Should have a Empty node");

					let child = self.children[0];

					let comment_start =
						doc.create_comment(&format!("<{}>", fragment_name(&self.name)));
					let comment_end =
						doc.create_comment(&format!("</{}>", fragment_name(&self.name)));

					child.replace_with(Child::Node(comment_end.clone().unchecked_into::<Node>()));
					let parent = comment_end.parent_node().unwrap();

					parent
						.insert_before(&comment_start, Some(&comment_end))
						.unwrap();

					for child in children {
						child.insert_before(&parent, Some(&comment_end));
					}

					self.kind = FragmentKind::Multi(comment_start, comment_end);
				}
			}
		}

		self.children = children
	}

	pub fn insert_child_at(&mut self, doc: &Document, child: Child, idx: usize) {
		self.children.insert(idx, child);

		match self.kind {
			FragmentKind::Multi(left, right) => {
				let before = self.children.get(idx + 1);
				let parent = left.parent_node().unwrap();
				child.insert_before(&parent, before.map(|b| b.left()))
			}
			FragmentKind::Empty(left) => {
				assert_eq!(idx, 0);

				let parent = left.parent_node().unwrap();
				self.children[0].insert_before(&parent, Some(&left));

				left.remove();
				self.kind = FragmentKind::Proxy
			}
			FragmentKind::Proxy => {
				assert_eq!(idx, 1);

				let comment_start = doc.create_comment(&format!("<{}>", fragment_name(&self.name)));
				let comment_end = doc.create_comment(&format!("</{}>", fragment_name(&self.name)));

				let left = self.children[0].left();
				let right = self.children[0].right();
				let parent = right.parent_node().unwrap();
				parent.insert_before(&comment_start, Some(&left)).unwrap();
				parent
					.insert_before(&comment_end, right.next_sibling().as_ref())
					.unwrap();

				child.insert_before(&parent, Some(&comment_end));
				self.kind = FragmentKind::Multi(comment_start, comment_end)
			}
		}
	}

	pub fn move_child(&mut self, from: usize, to: usize) {
		let child = self.children.remove(from);
		self.children.insert(to, child);

		match self.kind {
			FragmentKind::Multi(left, right) => {
				let before = self.children.get(to + 1);
				let parent = left.parent_node().unwrap();
				child.insert_before(&parent, before.map(|b| b.left()))
			}
			FragmentKind::Empty(left) => unreachable!(),
			FragmentKind::Proxy => unreachable!(),
		}
	}

	pub fn remove_child_at(&mut self, doc: &Document, idx: usize) {
		let child = self.children.remove(idx);

		match self.kind {
			FragmentKind::Multi(left, right) => {
				// FIXME should become proxy if the last one
				child.remove(None)
			}
			FragmentKind::Empty(left) => unreachable!(),
			FragmentKind::Proxy => {
				let comment = doc.create_comment(&format!("<{}/>", fragment_name(&self.name)));

				let left = self.children[0].left();
				let parent = left.parent_node().unwrap();
				parent.insert_before(&comment, Some(&left)).unwrap();

				self.children[0].remove(Some(&parent));
				self.kind = FragmentKind::Empty(comment);
			}
		}
	}
}

impl Drop for PersistedFragment {
	fn drop(&mut self) {
		for n in &self.children {
			n.remove(None /* FIXME Same parent? */)
		}
	}
}

fn remove_between(left: &Node, right: &Node) {
	let parent = left.parent_node().unwrap().unchecked_ref::<Element>();
	let children = parent.children();

	let iterator = js_sys::try_iter(&children)
		.unwrap()
		.ok_or_else(|| "need to pass iterable JS values!")
		.unwrap();

	let started = false;
	for node in iterator {
		let node = node.unwrap().unchecked_ref::<Node>();
		if Object::is(&node, &left) {
			started = true;
			continue;
		} else if Object::is(&node, &right) {
			return;
		} else if started {
			parent.remove_child(&node);
		}
	}
}
