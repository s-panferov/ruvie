use crate::global::Auto;
use crate::length::Length;
use crate::percentage::Percentage;
use crate::rule::{Attribute, ValueFor};

pub struct Width;

impl ValueFor<Width> for Length {}
impl ValueFor<Width> for Percentage {}
impl ValueFor<Width> for Auto {}

impl Attribute for Width {
    const NAME: &'static str = "width";
}
