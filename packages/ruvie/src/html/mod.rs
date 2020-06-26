use crate::{
	context::Mount,
	target::{Html, Target},
	Runtime, View,
};

#[derive(Clone)]
pub struct StaticHtml {}

impl Target for StaticHtml {
	type Realm = Html;

	type Arg = ();
	type Mount = ();
	type Result = ();
	type Error = ();
	type State = ();

	fn mount(
		_view: &View<Self>,
		ctx: Mount<Self>,
		_arg: Option<()>,
	) -> Result<Mount<Self>, Self::Error> {
		Ok(ctx)
	}

	fn mount_component(_ctx: &mut Self::Mount) -> Result<(), Self::Error> {
		Ok(())
	}

	fn schedule_tick(_scheduler: &Runtime<Self>) {}
}
