mod to_test_str;

use to_test_str::ToTestStr;

use parse_wiki_text_2::Configuration;
use std::fs;

struct Case {
	case: String,
	nodes: String,
	warnings: String,
}

fn to_val(s: &str) -> String {
	s.replace('\t', "\\t")
		.replace('\n', "\\n")
		.replace('\0', "\\0")
		.replace('\r', "\\r")
		.replace('\x7f', "\\x7f")
}

fn from_val(s: &str) -> String {
	s.replace("\\t", "\t")
		.replace("\\n", "\n")
		.replace("\\0", "\0")
		.replace("\\r", "\r")
		.replace("\\x7f", "\x7f")
}

fn parse_cases(content: &str) -> Vec<Case> {
	content
		.split("\n\n")
		.filter(|case| !case.trim().is_empty())
		.map(|case| {
			let mut options = case.split('\n');
			let case = options.next().expect("expected case: ");
			let case = case.strip_prefix("case: ").expect("expected case: ");

			let nodes = options.next().expect("expected node:");
			let nodes =
				nodes.strip_prefix("node:").expect("expected node:").trim();

			let warnings = options.next().expect("expected warn:");
			let warnings = warnings
				.strip_prefix("warn:")
				.expect("expected warn:")
				.trim();

			Case {
				case: from_val(case),
				nodes: from_val(nodes),
				warnings: from_val(warnings),
			}
		})
		.collect()
}

fn write_cases(cases: &[Case]) -> String {
	cases
		.iter()
		.map(
			|Case {
			     case,
			     nodes,
			     warnings,
			 }| {
				format!(
					"case: {}\nnode: {}\nwarn: {}",
					to_val(case),
					to_val(nodes),
					to_val(warnings)
				)
			},
		)
		.collect::<Vec<_>>()
		.join("\n\n")
}

fn test_file(path: &str) {
	let content = fs::read_to_string(path).expect("could not read file");
	let mut cases = parse_cases(&content);
	let mut changed_cases = false;

	let cfg = Configuration::default();

	for case in &mut cases {
		let res = cfg.parse(&case.case).unwrap();
		let expected_nodes = res.nodes.to_test_str();
		let expected_warnings = res.warnings.to_test_str();

		if case.nodes.is_empty() {
			changed_cases = true;
			case.nodes = expected_nodes;
		} else {
			assert_eq!(case.nodes, expected_nodes);
		}

		if case.warnings.is_empty() {
			changed_cases = true;
			case.warnings = expected_warnings;
		} else {
			assert_eq!(case.warnings, expected_warnings);
		}
	}

	if changed_cases {
		// write file
		fs::write(path, write_cases(&cases)).expect("failed to write case");
		panic!("some cases where changed in {path} check manually")
	}
}

macro_rules! test_files {
	($($name:ident),*) => (
		$(
			#[test]
			fn $name() {
				test_file(concat!("./tests/cases/", stringify!($name), ".test"))
			}
		)*
	)
}

test_files![
	basic,
	bold_italic,
	character_entity,
	comment,
	external_link,
	heading,
	horizontal_divider,
	invalid_character,
	link,
	list,
	magic_word,
	mix,
	nowiki,
	paragraph_break,
	parameter,
	preformatted_block,
	redirect,
	table,
	tag,
	template
];

// #[test]
// fn write_test_cases() {
// 	let cfg = Configuration::default();

// 	for (title, test_cases) in TEST_CASES {
// 		let cases: Vec<_> = test_cases.iter().map(|case| {
// 			let res = cfg.parse(&case);
// 			let expected_nodes = res.nodes.to_test_str();
// 			let expected_warnings = res.warnings.to_test_str();

// 			Case {
// 				case: case.to_string(),
// 				nodes: expected_nodes,
// 				warnings: expected_warnings
// 			}
// 		}).collect();

// 		fs::write(format!("./tests/cases/{title}.test"), write_cases(&cases)).expect("failed to write");
// 	}
// }
