use observe::local::EvalContext;

use crate::children::Children;
use crate::dom::Html;
use crate::target::Target;

pub struct Render<'a, P = (), T: Target = Html> {
    pub props: &'a P,
    pub eval: &'a mut EvalContext,
    pub children: &'a Children<T>,
}

pub trait Component: 'static {
    type Props;
    type Target: Target;

    fn name(&self) -> &'static str {
        return "Component";
    }

    fn render(&self, _ctx: Render<Self::Props, Self::Target>) -> Children<Self::Target> {
        _ctx.children.clone()
    }

    fn mount(
        &self,
        ctx: &mut <Self::Target as Target>::Mount,
        tree: Children<Self::Target>,
    ) -> Result<<Self::Target as Target>::Result, <Self::Target as Target>::Error> {
        Self::Target::component(ctx, tree)
    }
}
