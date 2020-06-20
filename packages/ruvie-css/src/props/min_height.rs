use crate::global::Auto;
use crate::length::Length;
use crate::percentage::Percentage;
use crate::rule::{Attribute, ValueFor};

pub struct MinHeight;

impl Attribute for MinHeight {
    const NAME: &'static str = "min-height";
}

impl ValueFor<MinHeight> for Length {}
impl ValueFor<MinHeight> for Percentage {}
impl ValueFor<MinHeight> for Auto {}
