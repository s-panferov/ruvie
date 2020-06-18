use observe::EvalContext;

use crate::children::Children;
use crate::target::Target;

pub struct Render<'a, T>
where
	T: Target,
{
	pub eval: &'a EvalContext,
	pub children: &'a Children<T>,
}

// impl<'a, T> Render<'a, T> where T: Target {}
