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
		view: &View<Self>,
		ctx: Mount<Self>,
		arg: Option<()>,
	) -> Result<Mount<Self>, Self::Error> {
		Ok(ctx)
	}

	fn mount_component(ctx: &mut Self::Mount) -> Result<(), Self::Error> {
		Ok(())
	}

	fn schedule_tick(scheduler: &Runtime<Self>) {}
}
