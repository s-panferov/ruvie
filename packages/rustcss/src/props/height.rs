use crate::global::Auto;
use crate::length::Length;
use crate::percentage::Percentage;
use crate::rule::{Attribute, ValueFor};

pub struct Height;

impl ValueFor<Height> for Length {}
impl ValueFor<Height> for Percentage {}
impl ValueFor<Height> for Auto {}

impl Attribute for Height {
    const NAME: &'static str = "height";
}
