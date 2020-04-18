use crate::global::Auto;
use crate::length::Length;
use crate::percentage::Percentage;
use crate::rule::{Attribute, ValueFor};

pub struct MinWidth;

impl Attribute for MinWidth {
    const NAME: &'static str = "min-width";
}

impl ValueFor<MinWidth> for Length {}
impl ValueFor<MinWidth> for Percentage {}
impl ValueFor<MinWidth> for Auto {}
