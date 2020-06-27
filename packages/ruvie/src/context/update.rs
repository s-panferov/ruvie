use observe::EvalContext;

use crate::View;

pub struct Update<'a> {
	pub eval: &'a EvalContext,
	pub view: &'a View,
}
