use crate::{
	context::Mount,
	error::RuvieError,
	html::{props, HtmlElement},
};

use observe::Observable;

pub use super::{text::*, Static, StaticContext};
use html5ever::{Attribute, LocalName, Namespace, QualName};
use markup5ever_rcdom::{Node, NodeData};
use std::{
	cell::{Cell, RefCell},
	rc::Rc,
};

impl HtmlElement {
	pub(crate) fn mount_static(
		&mut self,
		ctx: &mut Mount,
		target: &mut StaticContext,
	) -> Result<(), RuvieError> {
		let mut attrs = Vec::with_capacity(self.props.len());
		for prop in self.props.values() {
			if let Some(style) = prop.downcast_ref::<props::Style>() {
				match style {
					props::Style::String(style) => {
						let style = style.once();
						attrs.push(Attribute {
							name: QualName::new(
								None,
								Namespace::from("http://www.w3.org/1999/xhtml"),
								LocalName::from("style"),
							),
							value: style.to_string().into(),
						});
					}
					props::Style::StyleSheet(style) => {
						let style = style.once();
						attrs.push(Attribute {
							name: QualName::new(
								None,
								Namespace::from("http://www.w3.org/1999/xhtml"),
								LocalName::from("style"),
							),
							value: style.to_string().into(),
						});
					}
				}
			} else if let Some(props::Class(classlist)) = prop.downcast_ref() {
				let classlist = classlist.once();
				let rt = ctx.view.def.runtime.clone();
				let platform = rt.platform.downcast_ref::<Static>().unwrap();
				let list = classlist.format(&platform.styles);
				attrs.push(Attribute {
					name: QualName::new(
						None,
						Namespace::from("http://www.w3.org/1999/xhtml"),
						LocalName::from("class"),
					),
					value: list.into(),
				});
			} else if let Some(props::ContentEditable(contenteditable)) = prop.downcast_ref() {
				let contenteditable = contenteditable.once();
				attrs.push(Attribute {
					name: QualName::new(
						None,
						Namespace::from("http://www.w3.org/1999/xhtml"),
						LocalName::from("contenteditable"),
					),
					value: contenteditable.to_string().into(),
				});
			} else if let Some(props::Id(id)) = prop.downcast_ref() {
				attrs.push(Attribute {
					name: QualName::new(
						None,
						Namespace::from("http://www.w3.org/1999/xhtml"),
						LocalName::from("id"),
					),
					value: id.clone().into(),
				});
			}
		}

		let node = Node {
			parent: Cell::new(None),
			children: RefCell::new(vec![]),
			data: NodeData::Element {
				name: QualName::new(
					None,
					Namespace::from("http://www.w3.org/1999/xhtml"),
					LocalName::from(self.kind.clone()),
				),
				attrs: RefCell::new(attrs),
				template_contents: None,
				mathml_annotation_xml_integration_point: false,
			},
		};

		super::utils::mount_children(ctx, target, Some(&node))?;
		target.fragment.push(Rc::new(node));

		Ok(())
	}
}
