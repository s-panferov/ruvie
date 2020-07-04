use wasm_bindgen::JsCast;
use web_sys::Node;

use super::utils;
use crate::{
	context::Mount,
	error::RuvieError,
	html::{props, HtmlElement},
	props::PropFor,
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

impl PropFor<HtmlElement> for props::OnBeforeInput {}
impl PropFor<HtmlElement> for props::OnClick {}

impl HtmlElement {
	pub(crate) fn mount_web(
		&mut self,
		ctx: &mut Mount,
		target: &mut WebContext,
	) -> Result<(), RuvieError> {
		let el = target.doc.create_element(&self.kind)?;

		target.fragment.child(el.clone().unchecked_into::<Node>());
		utils::mount_children(ctx, target, Some(&el))?;

		for prop in self.props.values() {
			let el = el.clone();
			if let Some(ev) = prop.downcast_ref::<props::OnClick>() {
				match ev {
					props::OnClick::EventListener { handler, .. } => {
						target.handler(Box::new(bind(handler, &el, "click")?));
					}
				}
			} else if let Some(ev) = prop.downcast_ref::<props::OnBeforeInput>() {
				match ev {
					props::OnBeforeInput::EventListener { handler, .. } => {
						target.handler(Box::new(bind(handler, &el, "beforeinput")?));
					}
				}
			} else if let Some(style) = prop.downcast_ref::<props::Style>() {
				match style {
					props::Style::String(value) => {
						let value = value.clone();
						ctx.reaction(Box::new(move |ctx| {
							dynattr!(el, "style", &mut ctx.eval, value);
							Ok(())
						}))
					}
					props::Style::StyleSheet(sheet) => {
						let sheet = sheet.clone();
						ctx.reaction(Box::new(move |ctx| {
							dynattr!(el, "style", &mut ctx.eval, sheet);
							Ok(())
						}))
					}
				}
			} else if let Some(props::Class(classlist)) = prop.downcast_ref() {
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
			} else if let Some(props::ContentEditable(contenteditable)) = prop.downcast_ref() {
				let contenteditable = contenteditable.clone();
				ctx.reaction(Box::new(move |ctx| {
					dynattr!(el, "contenteditable", &mut ctx.eval, contenteditable);
					Ok(())
				}))
			} else if let Some(props::Id(id)) = prop.downcast_ref() {
				attr!(el, "id", id);
			}
		}

		Ok(())
	}
}
