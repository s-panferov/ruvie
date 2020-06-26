use crate::target::{Html, Target};
use crate::{
	component::ComponentExt, context::Render, scope::Scope, web::elem::Div, Children, Component,
};

struct Button {}

impl<T: Target<Realm = Html>> Component<T> for Button {
	type Props = ();

	fn create(_props: Self::Props, _scope: Scope<Self, T>) -> Self {
		Button {}
	}

	fn render(&mut self, _ctx: &Render<T>) -> Children<T> {
		Div::default().into()
	}
}
