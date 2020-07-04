use std::{any::Any, rc::Rc};

use super::props;

use crate::{
	component::{Component, Constructor},
	context::Mount,
	error::RuvieError,
	props::PropFor,
	scope::Scope,
	Props,
};

pub struct HtmlElement {
	pub kind: String,
	pub props: Rc<Props<Self>>,
}

impl Component for HtmlElement {
	type Props = Rc<Props<Self>>;

	fn mount(&mut self, ctx: &mut Mount, target: &mut dyn Any) -> Result<(), RuvieError> {
		#[cfg(feature = "web")]
		if target.is::<crate::web::WebContext>() {
			let target = target.downcast_mut::<crate::web::WebContext>().unwrap();
			self.mount_web(ctx, target)?
		}

		#[cfg(feature = "ssr")]
		if target.is::<crate::ssr::StaticContext>() {
			let target = target.downcast_mut::<crate::ssr::StaticContext>().unwrap();
			self.mount_static(ctx, target)?
		}

		Ok(())
	}
}

impl PropFor<HtmlElement> for props::Style {}
impl PropFor<HtmlElement> for props::Class {}
impl PropFor<HtmlElement> for props::ContentEditable {}
impl PropFor<HtmlElement> for props::Id {}

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
