use crate::global::Auto;
use crate::length::Length;
use crate::percentage::Percentage;
use crate::rule::{Attribute, ValueFor};

pub struct Left;

impl Attribute for Left {
    const NAME: &'static str = "left";
}

impl ValueFor<Left> for Length {}
impl ValueFor<Left> for Percentage {}
impl ValueFor<Left> for Auto {}
