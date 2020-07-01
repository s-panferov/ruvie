use wasm_bindgen::JsCast;
use web_sys::Node;

use super::utils;
use crate::{
	context::Mount,
	error::RuvieError,
	html::{props, HtmlElement},
};

use observe::Observable;

pub use super::{
	event::bind,
	fragment::{FragmentChild, SharedPersistedFragment},
	text::*,
	Web, WebContext,
};

macro_rules! dynattr {
	($el:expr, $x:expr, $eval:expr, $where:expr) => {
		let prop = &$where;
		let value = prop.get($eval);
		$el.set_attribute($x, &value.to_string())?
	};
}

macro_rules! attr {
	($el:expr, $x:expr, $where:expr) => {
		let prop = &$where;
		$el.set_attribute($x, &prop)?
	};
}

impl HtmlElement {
	pub(crate) fn mount_web(
		&mut self,
		ctx: &mut Mount,
		target: &mut WebContext,
	) -> Result<(), RuvieError> {
		let el = target.doc.create_element(&self.kind)?;

		target.fragment.child(el.clone().unchecked_into::<Node>());
		utils::mount_children(ctx, target, Some(&el))?;

		for prop in self.props.iter() {
			let el = el.clone();
			if let Some((_, ev)) = prop.downcast::<props::OnClick>() {
				target.handler(Box::new(bind(ev, &el, "click")?));
			} else if let Some((_, ev)) = prop.downcast::<props::OnBeforeInput>() {
				target.handler(Box::new(bind(ev, &el, "beforeinput")?));
			} else if let Some((_, style)) = prop.downcast::<props::Style>() {
				let style = style.clone();
				ctx.reaction(Box::new(move |ctx| {
					dynattr!(el, "style", &mut ctx.eval, style);
					Ok(())
				}))
			} else if let Some((_, classlist)) = prop.downcast::<props::Class>() {
				let classlist = classlist.clone();
				ctx.reaction({
					let rt = ctx.view.def.runtime.clone();
					Box::new(move |ctx| {
						let platform = rt.platform.downcast_ref::<Web>().unwrap();
						let list = classlist.get(ctx.eval).format(&platform.styles);
						el.set_attribute("class", &list)?;
						Ok(())
					})
				})
			} else if let Some((_, contenteditable)) = prop.downcast::<props::ContentEditable>() {
				let contenteditable = contenteditable.clone();
				ctx.reaction(Box::new(move |ctx| {
					dynattr!(el, "contenteditable", &mut ctx.eval, contenteditable);
					Ok(())
				}))
			} else if let Some((_, id)) = prop.downcast::<props::Id>() {
				attr!(el, "id", id);
			}
		}

		Ok(())
	}
}
