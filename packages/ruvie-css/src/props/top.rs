use crate::global::Auto;
use crate::length::Length;
use crate::percentage::Percentage;
use crate::rule::{Attribute, ValueFor};

pub struct Top;

impl ValueFor<Top> for Length {}
impl ValueFor<Top> for Percentage {}
impl ValueFor<Top> for Auto {}

impl Attribute for Top {
    const NAME: &'static str = "top";
}
