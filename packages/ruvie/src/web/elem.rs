use std::{any::Any, rc::Rc};
use wasm_bindgen::JsCast;
use web_sys::Node;

use super::utils;
use crate::{
	component::{Component, Constructor},
	context::Mount,
	error::RuvieError,
	props::PropFor,
	scope::Scope,
	Props,
};

use observe::Observable;

pub use super::{
	event::bind,
	fragment::{FragmentChild, SharedPersistedFragment},
	text::*,
	Class, ContentEditable, Id, OnBeforeInput, OnClick, Style, WebContext,
};

macro_rules! dynattr {
	($el:expr, $x:expr, $eval:expr, $where:expr) => {
		let prop = &$where;
		let value = prop.get($eval);
		$el.set_attribute(
			$x,
			&value
				.as_ref()
				.map(|v| v.to_string())
				.unwrap_or(String::from("")),
			)?
	};
}

macro_rules! attr {
	($el:expr, $x:expr, $where:expr) => {
		let prop = &$where;
		$el.set_attribute($x, &prop)?
	};
}

pub struct Div {
	props: Rc<Props<Self>>,
	scope: Scope<Self>,
}

impl PropFor<Div> for Style {}
impl PropFor<Div> for Class {}
impl PropFor<Div> for ContentEditable {}
impl PropFor<Div> for OnClick {}
impl PropFor<Div> for Id {}
impl PropFor<Div> for OnBeforeInput {}

impl Constructor for Div {
	fn create(props: Self::Props, scope: Scope<Self>) -> Self {
		Div { props, scope }
	}
}

impl Component for Div {
	type Props = Rc<Props<Self>>;

	fn mount(&mut self, ctx: &mut Mount, target: &mut dyn Any) -> Result<(), RuvieError> {
		if target.is::<WebContext>() {
			let target = target.downcast_mut::<WebContext>().unwrap();
			let el = target.doc.create_element("div")?;

			for prop in &self.props.props {
				if let Some((_, ev)) = prop.downcast::<OnClick>() {
					target.handler(Box::new(bind(ev, &el, "click")?));
				} else if let Some((_, ev)) = prop.downcast::<OnBeforeInput>() {
					target.handler(Box::new(bind(ev, &el, "beforeinput")?));
				}
			}

			target.fragment.child(el.clone().unchecked_into::<Node>());
			utils::mount_children(ctx, target, Some(&el))?;

			ctx.reaction(self.scope.reaction({
				let el = el.clone();
				move |c: &mut Self, ctx| {
					for prop in &c.props.props {
						if let Some((_, style)) = prop.downcast::<Style>() {
							dynattr!(el, "style", &mut ctx.eval, style);
						} else if let Some((_, classlist)) = prop.downcast::<Class>() {
							let list = classlist.get(ctx.eval).to_string();
							el.set_attribute("class", &list)?;
						} else if let Some((_, contenteditable)) =
							prop.downcast::<ContentEditable>()
						{
							dynattr!(el, "contenteditable", &mut ctx.eval, contenteditable);
						} else if let Some((_, id)) = prop.downcast::<Id>() {
							attr!(el, "id", id);
						}
					}
					Ok(())
				}
			}))
		}

		Ok(())
	}
}
