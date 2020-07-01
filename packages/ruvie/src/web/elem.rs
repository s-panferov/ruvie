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
		$el.set_attribute($x, &value.to_string())?
	};
}

macro_rules! attr {
	($el:expr, $x:expr, $where:expr) => {
		let prop = &$where;
		$el.set_attribute($x, &prop)?
	};
}

pub struct HtmlElementProps {}

pub struct HtmlElement {
	kind: String,
	props: Rc<Props<Self>>,
}

impl PropFor<HtmlElement> for Style {}
impl PropFor<HtmlElement> for Class {}
impl PropFor<HtmlElement> for ContentEditable {}
impl PropFor<HtmlElement> for OnClick {}
impl PropFor<HtmlElement> for Id {}
impl PropFor<HtmlElement> for OnBeforeInput {}

impl Component for HtmlElement {
	type Props = Rc<Props<Self>>;

	fn mount(&mut self, ctx: &mut Mount, target: &mut dyn Any) -> Result<(), RuvieError> {
		if target.is::<WebContext>() {
			let target = target.downcast_mut::<WebContext>().unwrap();
			let el = target.doc.create_element(&self.kind)?;

			target.fragment.child(el.clone().unchecked_into::<Node>());
			utils::mount_children(ctx, target, Some(&el))?;

			for prop in self.props.iter() {
				let el = el.clone();
				if let Some((_, ev)) = prop.downcast::<OnClick>() {
					target.handler(Box::new(bind(ev, &el, "click")?));
				} else if let Some((_, ev)) = prop.downcast::<OnBeforeInput>() {
					target.handler(Box::new(bind(ev, &el, "beforeinput")?));
				} else if let Some((_, style)) = prop.downcast::<Style>() {
					let style = style.clone();
					ctx.reaction(Box::new(move |ctx| {
						dynattr!(el, "style", &mut ctx.eval, style);
						Ok(())
					}))
				} else if let Some((_, classlist)) = prop.downcast::<Class>() {
					let classlist = classlist.clone();
					ctx.reaction(Box::new(move |ctx| {
						let list = classlist.get(ctx.eval).to_string();
						el.set_attribute("class", &list)?;
						Ok(())
					}))
				} else if let Some((_, contenteditable)) = prop.downcast::<ContentEditable>() {
					let contenteditable = contenteditable.clone();
					ctx.reaction(Box::new(move |ctx| {
						dynattr!(el, "contenteditable", &mut ctx.eval, contenteditable);
						Ok(())
					}))
				} else if let Some((_, id)) = prop.downcast::<Id>() {
					attr!(el, "id", id);
				}
			}
		}

		Ok(())
	}
}

macro_rules! elem {
	($el:ident) => {
		#[allow(non_camel_case_types)]
		pub struct $el {
			el: HtmlElement,
		}

		impl Constructor for $el {
			fn create(props: Self::Props, _scope: Scope<Self>) -> Self {
				$el {
					el: HtmlElement {
						kind: String::from(std::stringify!($el)),
						props,
					},
				}
			}
		}

		impl Component for $el {
			type Props = Rc<Props<HtmlElement>>;
			fn mount(&mut self, ctx: &mut Mount, target: &mut dyn Any) -> Result<(), RuvieError> {
				self.el.mount(ctx, target)
			}
		}
	};
}

elem!(a);
elem!(abbr);
elem!(address);
elem!(area);
elem!(article);
elem!(aside);
elem!(audio);
elem!(b);
elem!(base);
elem!(bdi);
elem!(bdo);
elem!(big);
elem!(blockquote);
elem!(body);
elem!(br);
elem!(button);
elem!(canvas);
elem!(caption);
elem!(cite);
elem!(code);
elem!(col);
elem!(colgroup);
elem!(data);
elem!(datalist);
elem!(dd);
elem!(del);
elem!(details);
elem!(dfn);
elem!(dialog);
elem!(div);
elem!(dl);
elem!(dt);
elem!(em);
elem!(embed);
elem!(fieldset);
elem!(figcaption);
elem!(figure);
elem!(footer);
elem!(form);
elem!(h1);
elem!(h2);
elem!(h3);
elem!(h4);
elem!(h5);
elem!(h6);
elem!(head);
elem!(header);
elem!(hgroup);
elem!(hr);
elem!(html);
elem!(i);
elem!(iframe);
elem!(img);
elem!(input);
elem!(ins);
elem!(kbd);
elem!(keygen);
elem!(label);
elem!(legend);
elem!(li);
elem!(link);
elem!(main);
elem!(map);
elem!(mark);
elem!(menu);
elem!(menuitem);
elem!(meta);
elem!(meter);
elem!(nav);
elem!(noscript);
elem!(object);
elem!(ol);
elem!(optgroup);
elem!(option);
elem!(output);
elem!(p);
elem!(param);
elem!(picture);
elem!(pre);
elem!(progress);
elem!(q);
elem!(rp);
elem!(rt);
elem!(ruby);
elem!(s);
elem!(samp);
elem!(slot);
elem!(script);
elem!(section);
elem!(select);
elem!(small);
elem!(source);
elem!(span);
elem!(strong);
elem!(style);
elem!(sub);
elem!(summary);
elem!(sup);
elem!(table);
elem!(template);
elem!(tbody);
elem!(td);
elem!(textarea);
elem!(tfoot);
elem!(th);
elem!(thead);
elem!(time);
elem!(title);
elem!(tr);
elem!(track);
elem!(u);
elem!(ul);
elem!(var);
elem!(video);
elem!(wbr);
elem!(webview);
