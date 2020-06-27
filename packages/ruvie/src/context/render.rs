use observe::EvalContext;

use crate::children::Children;

pub struct Render<'a> {
	pub eval: &'a EvalContext,
	pub children: &'a Children,
}
