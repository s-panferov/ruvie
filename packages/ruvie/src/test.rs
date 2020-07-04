use crate::{
	component::ComponentExt,
	context::Render,
	html::{el::div, props::Class},
	prelude::*,
	Children,
};
use observe::Value;

fn button(ctx: &Render) -> Children {
	div::prop(Class(Value::from(crate::cx!("test")))).into()
}

fn app(ctx: &Render) -> Children {
	button.default().into()
}

#[cfg(test)]
mod test {
	use super::*;
	use crate::{ssr::Static, Runtime};

	#[test]
	fn test() {
		let rt = Runtime::new(Static::new());
		let view = rt.render(app.default(), Box::new(())).unwrap();
		rt.tick().unwrap();
		assert_eq!(crate::ssr::stringify(&view), "test")
	}
}
