use observe::EvalContext;

use crate::{target::Target, View};
use std::marker::PhantomData;

pub struct Update<'a, T: Target> {
	pub eval: &'a EvalContext,
	pub view: &'a View<T>,
	pub(crate) _t: PhantomData<T>,
}
