use parse_wiki_text_2::{
	DefinitionListItem, ListItem, Node, Parameter, TableCaption, TableCell,
	TableRow, Warning,
};

pub trait ToTestStr {
	fn to_test_str(&self) -> String;
}

impl<T> ToTestStr for Vec<T>
where
	T: ToTestStr,
{
	fn to_test_str(&self) -> String {
		let mut s = format!("[");
		for (i, v) in self.iter().enumerate() {
			if i != 0 {
				s.push_str(", ");
			}
			s.push_str(&v.to_test_str());
		}

		s.push(']');

		s
	}
}

impl<T> ToTestStr for Option<T>
where
	T: ToTestStr,
{
	fn to_test_str(&self) -> String {
		self.as_ref()
			.map(|t| t.to_test_str())
			.unwrap_or(String::new())
	}
}

impl ToTestStr for Node<'_> {
	fn to_test_str(&self) -> String {
		use Node::*;

		match self {
			Bold { .. } => "Bold".into(),
			BoldItalic { .. } => "BoldItalic".into(),
			Category {
				ordinal, target, ..
			} => {
				format!("Category({target}, {})", ordinal.to_test_str())
			}
			CharacterEntity { character, .. } => {
				format!("CharacterEntity({character})")
			}
			Comment { .. } => "Comment".into(),
			DefinitionList { items, .. } => {
				format!("DefinitionList({})", items.to_test_str())
			}
			EndTag { name, .. } => format!("EndTag({name})"),
			ExternalLink { nodes, .. } => {
				format!("ExternalLink({})", nodes.to_test_str())
			}
			Heading { level, nodes, .. } => {
				format!("Heading({level}, {})", nodes.to_test_str())
			}
			HorizontalDivider { .. } => "HorizontalDivider".into(),
			Image { target, text, .. } => {
				format!("Image({target}, {})", text.to_test_str())
			}
			Italic { .. } => "Italic".into(),
			Link { target, text, .. } => {
				format!("Link({target}, {})", text.to_test_str())
			}
			MagicWord { .. } => "MagicWord".into(),
			OrderedList { items, .. } => {
				format!("OrderedList({})", items.to_test_str())
			}
			ParagraphBreak { .. } => "ParagraphBreak".into(),
			Parameter { default, name, .. } => format!(
				"Parameter({}, {})",
				default.to_test_str(),
				name.to_test_str()
			),
			Preformatted { nodes, .. } => {
				format!("Preformatted({})", nodes.to_test_str())
			}
			Redirect { target, .. } => format!("Redirect({target})"),
			StartTag { name, .. } => format!("StartTag({name})"),
			Table {
				attributes,
				captions,
				rows,
				..
			} => {
				format!(
					"Table {{attributes: {}, captions: {}, rows: {}}}",
					attributes.to_test_str(),
					captions.to_test_str(),
					rows.to_test_str()
				)
			}
			Tag { name, nodes, .. } => {
				format!("Tag({name}, {})", nodes.to_test_str())
			}
			Template {
				name, parameters, ..
			} => format!(
				"Template({}, {})",
				name.to_test_str(),
				parameters.to_test_str()
			),
			Text { value, .. } => format!("Text({value})"),
			UnorderedList { items, .. } => {
				format!("UnorderedList({})", items.to_test_str())
			}
		}
	}
}

impl ToTestStr for ListItem<'_> {
	fn to_test_str(&self) -> String {
		format!("ListItem({})", self.nodes.to_test_str())
	}
}

impl ToTestStr for DefinitionListItem<'_> {
	fn to_test_str(&self) -> String {
		format!(
			"DefinitionListItem({:?}, {})",
			self.type_,
			self.nodes.to_test_str()
		)
	}
}

impl ToTestStr for Warning {
	fn to_test_str(&self) -> String {
		format!("Warning({:?})", self.message)
	}
}

impl ToTestStr for TableCaption<'_> {
	fn to_test_str(&self) -> String {
		format!(
			"TableCaption({}, {})",
			self.attributes.to_test_str(),
			self.content.to_test_str()
		)
	}
}

impl ToTestStr for TableRow<'_> {
	fn to_test_str(&self) -> String {
		format!(
			"TableRow({}, {})",
			self.attributes.to_test_str(),
			self.cells.to_test_str()
		)
	}
}

impl ToTestStr for TableCell<'_> {
	fn to_test_str(&self) -> String {
		format!(
			"TableCell({:?}, {}, {})",
			self.type_,
			self.attributes.to_test_str(),
			self.content.to_test_str()
		)
	}
}

impl ToTestStr for Parameter<'_> {
	fn to_test_str(&self) -> String {
		format!(
			"Parameter({}, {})",
			self.name.to_test_str(),
			self.value.to_test_str()
		)
	}
}
