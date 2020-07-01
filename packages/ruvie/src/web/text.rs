use observe::Observable;
use wasm_bindgen::JsCast;
use web_sys::Node;

use crate::web::WebContext;

use crate::{context::Mount, error::RuvieError, html::Text};

impl Text {
	pub(crate) fn mount_web(
		&mut self,
		ctx: &mut Mount,
		target: &mut WebContext,
	) -> Result<(), RuvieError> {
		let el = target.doc.create_text_node("EMPTY");
		target.fragment.child(el.clone().unchecked_into::<Node>());
		let text = self.props.clone();
		ctx.reaction(Box::new(move |ctx| {
			el.set_data(&text.get(ctx.eval));
			Ok(())
		}));

		Ok(())
	}
}
