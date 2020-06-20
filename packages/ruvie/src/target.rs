use crate::{context::Mount, runtime::Runtime, View};
use downcast_rs::{impl_downcast, Downcast};

pub trait TargetRealm: Downcast {}

pub struct Html {}
pub struct Pdf {}

impl TargetRealm for Html {}
impl TargetRealm for Pdf {}

pub trait Target: Downcast + Clone {
	type Realm: TargetRealm;

	type Arg;
	type Mount;
	type Result;
	type Error;
	type State;

	fn mount_component(ctx: &mut Self::Mount) -> Result<(), Self::Error>;
	fn mount(
		view: &View<Self>,
		ctx: Mount<Self>,
		arg: Option<Self::Arg>,
	) -> Result<Mount<Self>, Self::Error>;
	fn schedule_tick(scheduler: &Runtime<Self>);
}
