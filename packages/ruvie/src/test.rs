use crate::target::{Html, Target};
use crate::{
	component::ComponentExt, context::Render, scope::Scope, web::elem::Div, Children, Component,
};

struct Button {}

impl<T: Target<Realm = Html>> Component<T> for Button {
	type Props = ();

	fn create(props: Self::Props, scope: Scope<Self, T>) -> Self {
		Button {}
	}

	fn render(&mut self, ctx: &Render<T>) -> Children<T> {
		Div::default().into()
	}
}
