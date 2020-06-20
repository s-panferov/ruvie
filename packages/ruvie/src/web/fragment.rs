use std::{borrow::Cow, cell::RefCell, mem::ManuallyDrop, rc::Rc};
use wasm_bindgen::{JsCast, JsValue};
use web_sys::{Comment, Document, DocumentFragment, Node, Range};

pub type SharedPersistedFragment = Rc<RefCell<PersistedFragment>>;

fn fragment_name(name: &Option<&'static str>) -> &'static str {
	name.map(|v| v.as_ref()).unwrap_or("Fragment")
}

pub enum FragmentKind {
	Empty(Comment),
	Range(Comment, Comment),
	Proxy(FragmentChild),
}

pub struct PersistedFragment {
	kind: FragmentKind,
	document: Document,
	attached: bool,
	name: Option<&'static str>,
}

pub enum FragmentChild {
	Node(Node),
	Fragment(SharedPersistedFragment),
}

impl FragmentChild {
	fn is_document_fragment(&self) -> bool {
		match self {
			FragmentChild::Fragment(_) => false,
			FragmentChild::Node(node) => node.has_type::<DocumentFragment>(),
		}
	}

	fn left(&self) -> Node {
		match self {
			FragmentChild::Fragment(frag) => frag.borrow().left(),
			FragmentChild::Node(node) => node.clone(),
		}
	}

	fn right(&self) -> Node {
		match self {
			FragmentChild::Fragment(frag) => frag.borrow().right(),
			FragmentChild::Node(node) => node.clone(),
		}
	}

	fn extract_contents(&self) -> Result<Node, JsValue> {
		match self {
			FragmentChild::Fragment(frag) => frag.borrow().extract(),
			FragmentChild::Node(node) => Ok(node.clone()),
		}
	}

	fn insert_self(&self, parent: &Node, pos: ChildPosition) -> Result<(), JsValue> {
		match self {
			FragmentChild::Node(node) => pos.insert(parent, &node)?,
			FragmentChild::Fragment(frag) => frag.borrow_mut().insert_self(parent, pos)?,
		}
		Ok(())
	}

	fn replace_with(&self, child: &FragmentChild) -> Result<(), JsValue> {
		match self {
			FragmentChild::Node(node) => match child {
				FragmentChild::Node(child) => {
					let parent = node.parent_node().unwrap();
					parent.replace_child(&node, &child)?;
				}
				FragmentChild::Fragment(child) => {
					let parent = node.parent_node().unwrap();
					child
						.borrow_mut()
						.insert_self(&parent, ChildPosition::Before(Cow::Borrowed(&node)))?;
					parent.remove_child(&node)?;
				}
			},
			FragmentChild::Fragment(frag) => frag.borrow_mut().replace_with(child)?,
		}

		Ok(())
	}

	fn remove(&self, parent: Option<Node>) -> Result<(), JsValue> {
		match self {
			FragmentChild::Node(node) => {
				let parent = parent.or_else(|| node.parent_node());
				if let Some(parent) = parent {
					parent.remove_child(&node)?;
				}
			}
			FragmentChild::Fragment(frag) => frag.borrow_mut().remove()?,
		}
		Ok(())
	}
}

fn self_closing(name: &Option<&'static str>) -> String {
	format!("<{}/>", fragment_name(&name))
}

fn open(name: &Option<&'static str>) -> String {
	format!("<{}>", fragment_name(&name))
}

fn close(name: &Option<&'static str>) -> String {
	format!("</{}>", fragment_name(&name))
}

const MISSING_PARENT: &'static str = "Should have a parent";

pub enum ChildPosition<'a> {
	Before(Cow<'a, Node>),
	After(Cow<'a, Node>),
	Append,
	Prepend,
}

impl<'a> ChildPosition<'a> {
	fn insert(&self, parent: &Node, node: &Node) -> Result<(), JsValue> {
		match self {
			ChildPosition::Append => parent.append_child(&node).map(|_| ()),
			ChildPosition::Prepend => parent
				.unchecked_ref::<web_sys::Element>()
				.prepend_with_node_1(&node),
			ChildPosition::Before(before) => parent.insert_before(&node, Some(&before)).map(|_| ()),
			ChildPosition::After(after) => parent
				.insert_before(&node, after.next_sibling().as_ref())
				.map(|_| ()),
		}
	}
}

impl PersistedFragment {
	pub fn new(
		document: Document,
		name: Option<&'static str>,
		mut children: Vec<FragmentChild>,
	) -> Result<PersistedFragment, JsValue> {
		let parent = document.create_document_fragment().unchecked_into::<Node>();
		let fragment = if children.len() == 0 {
			let comment = document.create_comment(&self_closing(&name));
			parent.append_child(&comment)?;
			PersistedFragment {
				document,
				attached: true,
				kind: FragmentKind::Empty(comment),
				name,
			}
		} else if children.len() == 1 && !children[0].is_document_fragment() {
			children[0].insert_self(&parent, ChildPosition::Append)?;
			PersistedFragment {
				document,
				attached: true,
				kind: FragmentKind::Proxy(children.remove(0)),
				name,
			}
		} else {
			let comment_start = document.create_comment(&open(&name));
			let comment_end = document.create_comment(&close(&name));
			parent.append_child(&comment_start)?;
			for child in &children {
				child.insert_self(&parent, ChildPosition::Append)?;
			}
			parent.append_child(&comment_end)?;
			PersistedFragment {
				document,
				attached: true,
				kind: FragmentKind::Range(comment_start, comment_end),
				name,
			}
		};

		Ok(fragment)
	}

	pub fn extract(&self) -> Result<Node, JsValue> {
		let range = self.range()?;
		Ok(range.extract_contents()?.unchecked_into())
	}

	pub fn insert_self(&mut self, parent: &Node, pos: ChildPosition) -> Result<(), JsValue> {
		pos.insert(parent, &self.extract()?)?;
		Ok(())
	}

	pub fn first_node(&self) -> Option<&Node> {
		todo!()
	}

	pub fn left(&self) -> Node {
		match &self.kind {
			FragmentKind::Range(left, _) => left.clone().unchecked_into(),
			FragmentKind::Empty(left) => left.clone().unchecked_into(),
			FragmentKind::Proxy(child) => child.left(),
		}
	}

	pub fn right(&self) -> Node {
		match &self.kind {
			FragmentKind::Range(_, right) => right.clone().unchecked_into(),
			FragmentKind::Empty(right) => right.clone().unchecked_into(),
			FragmentKind::Proxy(child) => child.right(),
		}
	}

	pub fn range(&self) -> Result<Range, JsValue> {
		assert!(self.attached);

		match &self.kind {
			FragmentKind::Range(left, right) => {
				let range = self.document.create_range().unwrap();
				range.set_start_before(&left)?;
				range.set_end_after(&right)?;
				Ok(range)
			}
			FragmentKind::Empty(comment) => {
				let range = self.document.create_range().unwrap();
				range.set_start_before(&comment)?;
				range.set_end_after(&comment)?;
				Ok(range)
			}
			FragmentKind::Proxy(child) => {
				let range = self.document.create_range().unwrap();
				range.set_start_before(&child.left())?;
				range.set_end_after(&child.right())?;
				Ok(range)
			}
		}
	}

	pub fn remove(&mut self) -> Result<(), JsValue> {
		match &self.kind {
			FragmentKind::Range(_left, _right) => self.range()?.delete_contents()?,
			FragmentKind::Empty(comment) => comment.remove(),
			FragmentKind::Proxy(child) => {
				child.remove(None)?;
			}
		}

		self.attached = false;

		Ok(())
	}

	pub fn replace_with(&mut self, child: &FragmentChild) -> Result<(), JsValue> {
		match &self.kind {
			FragmentKind::Range(left, _) => {
				let parent = left.parent_node().expect(MISSING_PARENT);
				child.insert_self(&parent, ChildPosition::Before(Cow::Borrowed(&left)))?;
				self.remove()?
			}
			FragmentKind::Empty(comment) => {
				let parent = comment.parent_node().expect(MISSING_PARENT);
				child.insert_self(&parent, ChildPosition::Before(Cow::Borrowed(&comment)))?;
				self.remove()?
			}
			FragmentKind::Proxy(child) => child.replace_with(child)?,
		}

		Ok(())
	}

	fn wrap_children(
		&self,
		children: &Vec<FragmentChild>,
	) -> Result<(DocumentFragment, Comment, Comment), JsValue> {
		let fragment = self.document.create_document_fragment();

		let left = self.document.create_comment(&open(&self.name));
		fragment.append_child(&left)?;

		for child in children {
			child.insert_self(&fragment, ChildPosition::Append)?;
		}

		let right = self.document.create_comment(&close(&self.name));
		fragment.append_child(&right)?;

		Ok((fragment, left, right))
	}

	pub fn replace_children(
		&mut self,
		doc: &Document,
		mut children: Vec<FragmentChild>,
	) -> Result<(), JsValue> {
		assert!(self.attached);

		let len = children.len();
		if len == 0 {
			match &self.kind {
				FragmentKind::Range(left, right) => {
					let range = self.document.create_range()?;
					range.set_start_after(&left)?;
					range.set_end_after(&right)?;
					range.delete_contents()?;
					left.set_data(&self_closing(&self.name));
					self.kind = FragmentKind::Empty(left.clone());
				}
				FragmentKind::Empty(_) => {}
				FragmentKind::Proxy(child) => {
					let comment = doc.create_comment(&self_closing(&self.name));
					child.replace_with(&FragmentChild::Node(comment.clone().unchecked_into()))?;
					self.kind = FragmentKind::Empty(comment);
				}
			}
		} else if len == 1 && !children[0].is_document_fragment() {
			// DocumentFragment can contain any amount of nodes, so we treat it like multi-node
			match &self.kind {
				FragmentKind::Range(left, _right) => {
					let parent = left.parent_node().expect(MISSING_PARENT);
					children[0]
						.insert_self(&parent, ChildPosition::Before(Cow::Borrowed(&left)))?;
					self.remove()?;
					self.kind = FragmentKind::Proxy(children.remove(0));
				}
				FragmentKind::Empty(comment) => {
					let parent = comment.parent_node().expect(MISSING_PARENT);
					children[0]
						.insert_self(&parent, ChildPosition::Before(Cow::Borrowed(&comment)))?;
					comment.remove();
					self.kind = FragmentKind::Proxy(children.remove(0));
				}
				FragmentKind::Proxy(child) => {
					let left = child.left();
					let parent = left.parent_node().expect(MISSING_PARENT);
					children[0]
						.insert_self(&parent, ChildPosition::Before(Cow::Borrowed(&left)))?;
					child.remove(Some(parent))?;
					self.kind = FragmentKind::Proxy(children.remove(0))
				}
			}
		} else {
			match &self.kind {
				FragmentKind::Range(left, right) => {
					let range = self.document.create_range()?;
					range.set_start_after(&left)?;
					range.set_end_before(&right)?;
					range.delete_contents()?;

					let fragment = self.document.create_document_fragment();
					// FIXME self.parent can be a fragment,
					//       so maybe it's pointless to crate an intermediate one
					for child in &children {
						child
							.insert_self(&fragment, ChildPosition::Before(Cow::Borrowed(&left)))?;
					}
					let parent = left.parent_node().expect(MISSING_PARENT);
					parent.insert_before(&parent, Some(right))?;
				}
				FragmentKind::Empty(comment) => {
					let parent = comment.parent_node().expect(MISSING_PARENT);
					let (fragment, left, right) = self.wrap_children(&children)?;
					parent.insert_before(&fragment, Some(&comment))?;
					comment.remove();
					self.kind = FragmentKind::Range(left, right);
				}
				FragmentKind::Proxy(child) => {
					let (fragment, left, right) = self.wrap_children(&children)?;
					child.replace_with(&FragmentChild::Node(
						fragment.clone().unchecked_into::<Node>(),
					))?;

					self.kind = FragmentKind::Range(left, right);
				}
			}
		}

		Ok(())
	}

	pub fn insert_child(
		&mut self,
		child: FragmentChild,
		pos: ChildPosition,
	) -> Result<(), JsValue> {
		match &self.kind {
			FragmentKind::Range(left, right) => {
				let parent = left.parent_node().expect(MISSING_PARENT);
				match &pos {
					ChildPosition::Before(_) => child.insert_self(&parent, pos),
					ChildPosition::After(_) => child.insert_self(&parent, pos),
					ChildPosition::Append => {
						child.insert_self(&parent, ChildPosition::Before(Cow::Borrowed(&right)))
					}
					ChildPosition::Prepend => {
						child.insert_self(&parent, ChildPosition::After(Cow::Borrowed(&left)))
					}
				}
			}
			FragmentKind::Empty(left) => {
				let parent = left.parent_node().expect(MISSING_PARENT);
				child.insert_self(&parent, ChildPosition::Before(Cow::Borrowed(&left)))?;
				left.remove();
				self.kind = FragmentKind::Proxy(child);
				Ok(())
			}
			FragmentKind::Proxy(current) => {
				let comment_start = self.document.create_comment(&open(&self.name));
				let comment_end = self.document.create_comment(&close(&self.name));

				let left = current.left();
				let right = current.right();

				let parent = left.parent_node().expect(MISSING_PARENT);
				parent.insert_before(&comment_start, Some(&left))?;
				parent.insert_before(&comment_end, right.next_sibling().as_ref())?;
				child.insert_self(&parent, ChildPosition::Before(Cow::Borrowed(&comment_end)))?;
				self.kind = FragmentKind::Range(comment_start, comment_end);
				Ok(())
			}
		}
	}

	pub fn move_child(
		&mut self,
		child: &SharedPersistedFragment,
		before: Option<Node>,
	) -> Result<(), JsValue> {
		match &self.kind {
			FragmentKind::Range(left, right) => {
				let parent = left.parent_node().expect(MISSING_PARENT);
				match before {
					Some(before) => child
						.borrow_mut()
						.insert_self(&parent, ChildPosition::Before(Cow::Borrowed(&before)))?,
					None => child
						.borrow_mut()
						.insert_self(&parent, ChildPosition::Before(Cow::Borrowed(&right)))?,
				}
			}
			FragmentKind::Empty(_left) => unreachable!(),
			FragmentKind::Proxy(_) => unreachable!(),
		}

		Ok(())
	}

	pub fn remove_child(&mut self, child: &SharedPersistedFragment) -> Result<(), JsValue> {
		match &self.kind {
			FragmentKind::Range(left, right) => {
				child.borrow_mut().remove()?;
				if left.next_sibling().as_ref() == Some(right) {
					right.remove();
					left.set_data(&self_closing(&self.name));
					self.kind = FragmentKind::Empty(left.clone())
				}
			}
			FragmentKind::Empty(_left) => unreachable!(),
			FragmentKind::Proxy(current) => {
				// TODO assert current == child

				let comment = self.document.create_comment(&self_closing(&self.name));

				let left = current.left();
				let parent = left.parent_node().expect(MISSING_PARENT);
				parent.insert_before(&comment, Some(&left))?;
				current.remove(Some(parent))?;
				self.kind = FragmentKind::Empty(comment);
			}
		}

		Ok(())
	}
}

pub struct FragmentBuilder {
	pub children: Vec<FragmentChild>,
}

impl FragmentBuilder {
	pub fn new() -> Self {
		FragmentBuilder { children: vec![] }
	}

	// fn node_ref(&mut self, node: &dyn AsRef<Node>) {
	// 	self.children.push(Child::Node(node.as_ref().clone()));
	// }

	pub fn child<C: Into<FragmentChild>>(&mut self, node: C) {
		self.children.push(node.into())
	}
}

impl From<Node> for FragmentChild {
	fn from(node: Node) -> Self {
		FragmentChild::Node(node)
	}
}

impl From<SharedPersistedFragment> for FragmentChild {
	fn from(frag: SharedPersistedFragment) -> Self {
		FragmentChild::Fragment(frag)
	}
}

impl Drop for PersistedFragment {
	fn drop(&mut self) {
		self.remove().unwrap()
	}
}
