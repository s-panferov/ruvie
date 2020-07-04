use markup5ever_rcdom::Node;
use markup5ever_rcdom::NodeData;
use observe::Observable;
use std::{
	cell::{Cell, RefCell},
	rc::Rc,
};

use crate::{context::Mount, error::RuvieError, html::Text};

use super::StaticContext;

impl Text {
	pub(crate) fn mount_static(
		&mut self,
		_ctx: &mut Mount,
		impl_ctx: &mut StaticContext,
	) -> Result<(), RuvieError> {
		let node = Node {
			children: RefCell::new(Vec::new()),
			parent: Cell::new(None),
			data: NodeData::Text {
				contents: RefCell::new(self.props.once().clone().into()),
			},
		};

		impl_ctx.fragment.push(Rc::new(node));

		Ok(())
	}
}
