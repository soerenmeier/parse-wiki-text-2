// Copyright 2019 Fredrik Portström <https://portstrom.com>
// This is free software distributed under the terms specified in
// the file LICENSE at the top-level directory of this distribution.

pub struct OpenNode<'a> {
    pub nodes: Vec<crate::Node<'a>>,
    pub start: usize,
    pub type_: OpenNodeType<'a>,
}

pub enum OpenNodeType<'a> {
    DefinitionList {
        items: Vec<crate::DefinitionListItem<'a>>,
    },
    ExternalLink,
    Heading {
        level: u8,
    },
    Link {
        namespace: Option<crate::Namespace>,
        target: &'a str,
    },
    OrderedList {
        items: Vec<crate::ListItem<'a>>,
    },
    Parameter {
        default: Option<Vec<crate::Node<'a>>>,
        name: Option<Vec<crate::Node<'a>>>,
    },
    Preformatted,
    Table(Table<'a>),
    Tag {
        name: crate::Cow<'a, str>,
    },
    Template {
        name: Option<Vec<crate::Node<'a>>>,
        parameters: Vec<crate::Parameter<'a>>,
    },
    UnorderedList {
        items: Vec<crate::ListItem<'a>>,
    },
}

pub struct State<'a> {
    pub flushed_position: usize,
    pub nodes: Vec<crate::Node<'a>>,
    pub scan_position: usize,
    pub stack: Vec<OpenNode<'a>>,
    pub warnings: Vec<crate::Warning>,
    pub wiki_text: &'a str,
}

pub struct Table<'a> {
    pub attributes: Vec<crate::Node<'a>>,
    pub before: Vec<crate::Node<'a>>,
    pub captions: Vec<crate::TableCaption<'a>>,
    pub child_element_attributes: Option<Vec<crate::Node<'a>>>,
    pub rows: Vec<crate::TableRow<'a>>,
    pub start: usize,
    pub state: TableState,
}

pub enum TableState {
    Before,
    CaptionFirstLine,
    CaptionRemainder,
    CellFirstLine,
    CellRemainder,
    HeadingFirstLine,
    HeadingRemainder,
    Row,
    TableAttributes,
}

impl<'a> State<'a> {
    pub fn flush(&mut self, end_position: usize) {
        flush(
            &mut self.nodes,
            self.flushed_position,
            end_position,
            self.wiki_text,
        );
    }

    pub fn get_byte(&self, position: usize) -> Option<u8> {
        self.wiki_text.as_bytes().get(position).cloned()
    }

    pub fn push_open_node(&mut self, type_: OpenNodeType<'a>, inner_start_position: usize) {
        let scan_position = self.scan_position;
        self.flush(scan_position);
        self.stack.push(OpenNode {
            nodes: std::mem::replace(&mut self.nodes, vec![]),
            start: scan_position,
            type_,
        });
        self.scan_position = inner_start_position;
        self.flushed_position = inner_start_position;
    }

    pub fn rewind(&mut self, nodes: Vec<crate::Node<'a>>, position: usize) {
        self.scan_position = position + 1;
        self.nodes = nodes;
        if let Some(position_before_text) = match self.nodes.last() {
            Some(crate::Node::Text { start, .. }) => Some(*start),
            _ => None,
        } {
            self.nodes.pop();
            self.flushed_position = position_before_text;
        } else {
            self.flushed_position = position;
        }
    }

    pub fn skip_empty_lines(&mut self) {
        match self.stack.last() {
            Some(OpenNode {
                type_: OpenNodeType::Table { .. },
                ..
            }) => {
                self.scan_position -= 1;
                crate::table::parse_table_end_of_line(self, false);
            }
            _ => {
                crate::line::parse_beginning_of_line(self, None);
            }
        }
    }

    pub fn skip_whitespace_backwards(&self, position: usize) -> usize {
        skip_whitespace_backwards(self.wiki_text, position)
    }

    pub fn skip_whitespace_forwards(&self, position: usize) -> usize {
        skip_whitespace_forwards(self.wiki_text, position)
    }
}

pub fn flush<'a>(
    nodes: &mut Vec<crate::Node<'a>>,
    flushed_position: usize,
    end_position: usize,
    wiki_text: &'a str,
) {
    if end_position > flushed_position {
        nodes.push(crate::Node::Text {
            end: end_position,
            start: flushed_position,
            value: &wiki_text[flushed_position..end_position],
        });
    }
}

pub fn skip_whitespace_backwards(wiki_text: &str, mut position: usize) -> usize {
    while position > 0
        && match wiki_text.as_bytes()[position - 1] {
            b'\t' | b'\n' | b' ' => true,
            _ => false,
        }
    {
        position -= 1;
    }
    position
}

pub fn skip_whitespace_forwards(wiki_text: &str, mut position: usize) -> usize {
    while match wiki_text.as_bytes().get(position).cloned() {
        Some(b'\t') | Some(b'\n') | Some(b' ') => true,
        _ => false,
    } {
        position += 1;
    }
    position
}
