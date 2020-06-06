use std::rc::Rc;
use wasm_bindgen::{JsCast, JsValue};
use web_sys::Node;

use super::{utils, Web};
use crate::{
	component::{Component, Lifecycle},
	props::PropFor,
	scope::Scope,
	target::{Html, Target},
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

pub struct Div<T: Target<Realm = Html>> {
	props: Rc<Props<Self>>,
	scope: Scope<Self, T>,
}

impl<T: Target<Realm = Html>> PropFor<Div<T>> for Style {}
impl<T: Target<Realm = Html>> PropFor<Div<T>> for Class {}
impl<T: Target<Realm = Html>> PropFor<Div<T>> for ContentEditable {}
impl<T: Target<Realm = Html>> PropFor<Div<T>> for OnClick {}
impl<T: Target<Realm = Html>> PropFor<Div<T>> for Id {}
impl<T: Target<Realm = Html>> PropFor<Div<T>> for OnBeforeInput {}

impl<T: Target<Realm = Html>> Component<T> for Div<T> {
	type Props = Rc<Props<Self>>;
	fn create(props: Self::Props, scope: Scope<Self, T>) -> Self {
		Div { props, scope }
	}
}

impl Lifecycle<Web> for Div<Web> {
	fn mount(&mut self, ctx: &mut WebContext) -> Result<(), JsValue> {
		let el = ctx.doc.create_element("div")?;

		for prop in &self.props.props {
			if let Some((_, ev)) = prop.downcast::<OnClick>() {
				ctx.handler(Box::new(bind(ev, &el, "click")?));
			} else if let Some((_, ev)) = prop.downcast::<OnBeforeInput>() {
				ctx.handler(Box::new(bind(ev, &el, "beforeinput")?));
			}
		}

		ctx.fragment.child(el.clone().unchecked_into::<Node>());
		utils::mount_children(ctx, Some(&el))?;

		ctx.reaction(self.scope.reaction({
			let el = el.clone();
			move |c: &mut Self, ctx| {
				for prop in &c.props.props {
					if let Some((_, style)) = prop.downcast::<Style>() {
						dynattr!(el, "style", &mut ctx.eval, style);
					} else if let Some((_, class)) = prop.downcast::<Class>() {
						dynattr!(el, "class", &mut ctx.eval, class);
					} else if let Some((_, contenteditable)) = prop.downcast::<ContentEditable>() {
						dynattr!(el, "contenteditable", &mut ctx.eval, contenteditable);
					} else if let Some((_, id)) = prop.downcast::<Id>() {
						attr!(el, "id", id);
					}
				}
				Ok(())
			}
		}));

		Ok(())
	}
}
