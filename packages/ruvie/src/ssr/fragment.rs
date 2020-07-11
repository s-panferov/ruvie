use markup5ever_rcdom::{Node, NodeData};
use std::{
	cell::{Cell, RefCell},
	rc::Rc,
};

pub fn wrap_fragment(fragment: &mut Vec<Rc<Node>>, name: &'static str) {
	if fragment.len() == 0 {
		fragment.push(Rc::new(Node {
			parent: Cell::new(None),
			children: RefCell::new(vec![]),
			data: NodeData::Comment {
				contents: format!("<{}/>", name).into(),
			},
		}))
	} else if fragment.len() > 1 {
		fragment.insert(
			0,
			Rc::new(Node {
				parent: Cell::new(None),
				children: RefCell::new(vec![]),
				data: NodeData::Comment {
					contents: format!("<{}>", name).into(),
				},
			}),
		);
		fragment.push(Rc::new(Node {
			parent: Cell::new(None),
			children: RefCell::new(vec![]),
			data: NodeData::Comment {
				contents: format!("</{}>", name).into(),
			},
		}));
	}
}
