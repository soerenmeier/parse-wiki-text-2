// Copyright 2019 Fredrik Portström <https://portstrom.com>
// This is free software distributed under the terms specified in
// the file LICENSE at the top-level directory of this distribution.

mod to_test_str;

use to_test_str::ToTestStr;

use parse_wiki_text_2::{Configuration, Node::*, Warning, WarningMessage::*};

// single line test
macro_rules! sl_test {
	// if nothing is specified just check that there are no warnigs
	($cfg:expr, $test_case:expr) => {{
		let res = $cfg.parse($test_case);
		assert!(
			res.warnings.is_empty(),
			"{:?} had warnings {:?}",
			$test_case,
			res.warnings.to_test_str()
		);
	}};
	($cfg:expr, $test_case:expr, dbg) => {{
		let res = $cfg.parse($test_case);
		eprintln!(
			"\n{:?} : nodes:\n{}\nwarnings:\n{}",
			$test_case,
			res.nodes.to_test_str(),
			res.warnings.to_test_str()
		);
	}};
	($cfg:expr, $test_case:expr, $nodes:expr) => {{
		let res = $cfg.parse($test_case);

		let nodes_test_str = res.nodes.to_test_str();
		assert_eq!(nodes_test_str, $nodes, "at {:?}", $test_case);

		assert!(
			res.warnings.is_empty(),
			"{:?} had warnings {:?}",
			$test_case,
			res.warnings.to_test_str()
		);
	}};
	($cfg:expr, $test_case:expr, $nodes:expr, $warnings:expr) => {{
		let res = $cfg.parse($test_case);

		let nodes_test_str = res.nodes.to_test_str();
		assert_eq!(nodes_test_str, $nodes);
		let warnings_test_str = res.warnings.to_test_str();
		assert_eq!(warnings_test_str, $warnings);
	}};
}

macro_rules! sl_tests {
	(
		$cfg:expr, $($test_case:expr),*
	) => ({
		$(
			sl_test!($cfg, $test_case);
		)*
	});
	(
		$cfg:expr, $($test_case:expr),*;
		dbg
	) => ({
		$(
			sl_test!($cfg, $test_case, dbg);
		)*
	});
	(
		$cfg:expr, $($test_case:expr),*;
		$nodes:expr
	) => ({
		$(
			sl_test!($cfg, $test_case, $nodes);
		)*
	});
	(
		$cfg:expr, $($test_case:expr),*;
		$nodes:expr,
		$warnings:expr
	) => ({
		$(
			sl_test!($cfg, $test_case, $nodes, $warnings);
		)*
	})
}

#[test]
fn test_cases() {
	// let cfg = Configuration::default();

	// sl_test!(cfg, "");

	// for (title, test_cases) in TEST_CASES {
	// 	eprintln!("test {title}");
	// 	for test_case in *test_cases {
	// 		let res = cfg.parse(test_case);
	// 		assert!(res.warnings.is_empty(), "{:?} {:?}", test_case, res);
	// 		eprintln!("test_case {test_case:?} res {res:?}");
	// 	}
	// }
}

#[test]
fn basic() {
	let cfg = Configuration::default();

	sl_tests!(cfg, "", "\t", "\t\n", "\n", "\n\t", " ", "  ");
	sl_tests!(cfg,
		"\t alpha",
		"\talpha",
		" \nalpha",
		"alpha",
		"\nalpha",
		"\nalpha\n",
		"alpha\t",
		"alpha\n",
		"alpha\n\t",
		"alpha\n\n",
		"alpha\n\n ",
		"alpha\n ",
		"alpha\n \n",
		"alpha ",
		"alpha \n";
		"[Text(alpha)]"
	);
	sl_tests!(cfg,
		"\n\n\nalpha";
		"[Text(alpha)]",
		"[Warning(RepeatedEmptyLine), Warning(RepeatedEmptyLine)]"
	);
	sl_tests!(cfg,
		"\n\nalpha",
		"\n\nalpha\n\n",
		"\n \nalpha",
		" \n\nalpha",
		"alpha\n\n\n";
		"[Text(alpha)]",
		"[Warning(RepeatedEmptyLine)]"
	);
	sl_tests!(cfg,
		"!!";
		"[Text(!!)]"
	);
	sl_tests!(cfg,
		"alpha\nbeta";
		"[Text(alpha\nbeta)]"
	);
}

#[test]
fn bold_italic() {
	let cfg = Configuration::default();

	sl_test!(cfg, "'", "[Text(')]");
	sl_test!(cfg, "''", "[Italic]");
	sl_test!(cfg, "'''", "[Bold]");
	sl_test!(cfg, "''''", "[Bold, Text(')]");
	sl_test!(cfg, "'''''", "[BoldItalic]");
	sl_test!(cfg, "''''''", "[BoldItalic, Text(')]");
	sl_test!(cfg, "'''''''", "[BoldItalic, Text('')]");
	sl_test!(cfg, "''''''''", "[BoldItalic, Text(''')]");
	sl_test!(cfg, "'''alpha", "[Bold, Text(alpha)]");
	sl_test!(cfg, "'''alpha''", "[Bold, Text(alpha), Italic]");
	sl_test!(cfg, "'''alpha'''", "[Bold, Text(alpha), Bold]");
	sl_test!(cfg, "''alpha", "[Italic, Text(alpha)]");
	sl_test!(cfg, "''alpha''", "[Italic, Text(alpha), Italic]");
	sl_test!(cfg, "''alpha'''", "[Italic, Text(alpha), Bold]");
	sl_test!(cfg, "alpha''", "[Text(alpha), Italic]");
	sl_test!(cfg, "alpha'''", "[Text(alpha), Bold]");
	sl_test!(cfg, "alpha'''beta", "[Text(alpha), Bold, Text(beta)]");
	sl_test!(
		cfg,
		"alpha'''beta'''gamma",
		"[Text(alpha), Bold, Text(beta), Bold, Text(gamma)]"
	);
	sl_test!(
		cfg,
		"alpha'''beta''gamma",
		"[Text(alpha), Bold, Text(beta), Italic, Text(gamma)]"
	);
	sl_test!(cfg, "alpha''beta", "[Text(alpha), Italic, Text(beta)]");
	sl_test!(
		cfg,
		"alpha''beta'''gamma",
		"[Text(alpha), Italic, Text(beta), Bold, Text(gamma)]"
	);
	sl_test!(
		cfg,
		"alpha''beta''gamma",
		"[Text(alpha), Italic, Text(beta), Italic, Text(gamma)]"
	);
}

#[test]
fn character_entity() {
	let cfg = Configuration::default();

	sl_test!(cfg, "&Lt;", "[Text(&Lt;)]");
	sl_test!(cfg, "&Ouml;", "[CharacterEntity(Ö)]");
	sl_test!(cfg, "&lt", "[Text(&lt)]");
	sl_test!(cfg, "&lt&ouml;", "[Text(&lt), CharacterEntity(ö)]");
	sl_test!(cfg, "&lt;", "[CharacterEntity(<)]");
	sl_test!(cfg, "&lt; alpha", "[CharacterEntity(<), Text( alpha)]");
	sl_test!(
		cfg,
		"&lt;&ouml;",
		"[CharacterEntity(<), CharacterEntity(ö)]"
	);
	sl_test!(cfg, "&lt;alpha", "[CharacterEntity(<), Text(alpha)]");
	sl_test!(cfg, "&ouml;", "[CharacterEntity(ö)]");
	sl_test!(cfg, "alpha &lt;", "[Text(alpha ), CharacterEntity(<)]");
	sl_test!(
		cfg,
		"alpha &lt; beta",
		"[Text(alpha ), CharacterEntity(<), Text( beta)]"
	);
	sl_test!(cfg, "alpha&lt;", "[Text(alpha), CharacterEntity(<)]");
	sl_test!(
		cfg,
		"alpha&lt;beta",
		"[Text(alpha), CharacterEntity(<), Text(beta)]"
	);
}

#[test]
fn comment() {
	let cfg = Configuration::default();

	sl_test!(cfg, "<!--", "[Comment]");
	sl_test!(cfg, "<!---->", "[Comment]");
	sl_test!(cfg, "<!--->beta", "[Comment]");
	sl_test!(cfg, "<!--alpha-->", "[Comment]");
	sl_test!(cfg, "<!--alpha--><!--beta", "[Comment, Comment]");
	sl_test!(cfg, "<!--alpha--><!--beta-->", "[Comment, Comment]");
	sl_test!(cfg, "<!---->beta", "[Comment, Text(beta)]");
	sl_test!(cfg, "<!--<!--alpha-->-->beta", "[Comment, Text(-->beta)]");
	sl_test!(cfg, "<!--alpha--> beta", "[Comment, Text( beta)]");
	sl_test!(cfg, "<!--alpha-->beta", "[Comment, Text(beta)]");
	sl_test!(
		cfg,
		"<!-<!--alpha-->beta",
		"[Text(<!-), Comment, Text(beta)]",
		"[Warning(UnrecognizedTagName)]"
	);
	sl_test!(cfg, "alpha <!--beta", "[Text(alpha ), Comment]");
	sl_test!(cfg, "alpha<!--beta", "[Text(alpha), Comment]");
}

#[test]
fn external_links() {
	let cfg = Configuration::default();

	sl_test!(
		cfg,
		"[//alpha",
		"[Text([//alpha)]",
		"[Warning(InvalidLinkSyntax)]"
	);
	sl_test!(
		cfg,
		"[//alpha beta\ngamma]",
		"[Text([//alpha beta\ngamma])]",
		"[Warning(InvalidLinkSyntax)]"
	);
	sl_test!(
		cfg,
		"[//alpha beta]",
		"[ExternalLink([Text(//alpha beta)])]"
	);
	sl_test!(cfg, "[//alpha]", "[ExternalLink([Text(//alpha)])]");
	sl_test!(
		cfg,
		"[//alpha] beta",
		"[ExternalLink([Text(//alpha)]), Text( beta)]"
	);
	sl_test!(
		cfg,
		"[//alpha]beta",
		"[ExternalLink([Text(//alpha)]), Text(beta)]"
	);
	sl_test!(
		cfg,
		"[HTTP://alpha]",
		"[ExternalLink([Text(HTTP://alpha)])]"
	);
	sl_test!(
		cfg,
		"[Http://alpha]",
		"[ExternalLink([Text(Http://alpha)])]"
	);
	sl_test!(cfg, "[alpha://beta]", "[Text([alpha://beta])]");
	sl_test!(
		cfg,
		"[hTtP://alpha]",
		"[ExternalLink([Text(hTtP://alpha)])]"
	);
	sl_test!(
		cfg,
		"[http://alpha]",
		"[ExternalLink([Text(http://alpha)])]"
	);
	sl_test!(cfg, "[http:/alpha]", "[Text([http:/alpha])]");
	sl_test!(cfg, "[http:alpha]", "[Text([http:alpha])]");
	sl_test!(
		cfg,
		"[https://alpha]",
		"[ExternalLink([Text(https://alpha)])]"
	);
	sl_test!(cfg, "[sip:alpha]", "[ExternalLink([Text(sip:alpha)])]");
	sl_test!(
		cfg,
		"alpha [//beta]",
		"[Text(alpha ), ExternalLink([Text(//beta)])]"
	);
	sl_test!(
		cfg,
		"alpha [//beta] gamma",
		"[Text(alpha ), ExternalLink([Text(//beta)]), Text( gamma)]"
	);
	sl_test!(
		cfg,
		"alpha[//beta]",
		"[Text(alpha), ExternalLink([Text(//beta)])]"
	);
	sl_test!(
		cfg,
		"alpha[//beta]gamma",
		"[Text(alpha), ExternalLink([Text(//beta)]), Text(gamma)]"
	);
}

#[test]
fn heading() {
	let cfg = Configuration::default();

	sl_test!(
		cfg,
		"=",
		"[Text(=)]",
		"[Warning(InvalidHeadingSyntaxRewinding)]"
	);
	sl_test!(cfg, "= =", "[Heading(1, [])]");
	sl_test!(cfg, "= alpha =", "[Heading(1, [Text(alpha)])]");
	sl_test!(cfg, "=''=", "[Heading(1, [Italic])]");
	sl_test!(
		cfg,
		"==",
		"[Text(==)]",
		"[Warning(InvalidHeadingSyntaxRewinding)]"
	);
	sl_test!(
		cfg,
		"== ''=",
		"[Heading(1, [Text(= ), Italic])]",
		"[Warning(UnexpectedHeadingLevelCorrecting)]"
	);
	sl_test!(
		cfg,
		"== alpha''=",
		"[Heading(1, [Text(= alpha), Italic])]",
		"[Warning(UnexpectedHeadingLevelCorrecting)]"
	);
	sl_test!(
		cfg,
		"==''=",
		"[Heading(1, [Text(=), Italic])]",
		"[Warning(UnexpectedHeadingLevelCorrecting)]"
	);
	sl_test!(
		cfg,
		"===",
		"[Heading(1, [Text(=)])]",
		"[Warning(UnexpectedHeadingLevelCorrecting)]"
	);
	sl_test!(
		cfg,
		"====",
		"[Heading(1, [Text(==)])]",
		"[Warning(UnexpectedHeadingLevelCorrecting)]"
	);
	sl_test!(
		cfg,
		"=====",
		"[Heading(2, [Text(=)])]",
		"[Warning(UnexpectedHeadingLevelCorrecting)]"
	);
	sl_test!(
		cfg,
		"======",
		"[Heading(2, [Text(==)])]",
		"[Warning(UnexpectedHeadingLevelCorrecting)]"
	);
	sl_test!(
		cfg,
		"========alpha========",
		"[Heading(6, [Text(==alpha==)])]"
	);
	sl_test!(cfg, "=======alpha======", "[Heading(6, [Text(=alpha)])]");
	sl_test!(cfg, "=======alpha=======", "[Heading(6, [Text(=alpha=)])]");
	sl_test!(cfg, "======alpha======", "[Heading(6, [Text(alpha)])]");
	sl_test!(cfg, "=====alpha=====", "[Heading(5, [Text(alpha)])]");
	sl_test!(cfg, "====alpha====", "[Heading(4, [Text(alpha)])]");
	sl_test!(cfg, "===alpha===", "[Heading(3, [Text(alpha)])]");
	sl_test!(
		cfg,
		"==alpha''=",
		"[Heading(1, [Text(=alpha), Italic])]",
		"[Warning(UnexpectedHeadingLevelCorrecting)]"
	);
	sl_test!(
		cfg,
		"==alpha=",
		"[Heading(1, [Text(=alpha)])]",
		"[Warning(UnexpectedHeadingLevelCorrecting)]"
	);
	sl_test!(cfg, "==alpha==", "[Heading(2, [Text(alpha)])]");
	sl_test!(
		cfg,
		"=alpha",
		"[Text(=alpha)]",
		"[Warning(InvalidHeadingSyntaxRewinding)]"
	);
	sl_test!(
		cfg,
		"=alpha\nbeta=",
		"[Text(=alpha\nbeta=)]",
		"[Warning(InvalidHeadingSyntaxRewinding)]"
	);
	sl_test!(cfg, "=alpha=", "[Heading(1, [Text(alpha)])]");
	sl_test!(
		cfg,
		"=alpha=\n\n\nbeta",
		"[Heading(1, [Text(alpha)]), Text(beta)]",
		"[Warning(RepeatedEmptyLine)]"
	);
	sl_test!(
		cfg,
		"=alpha=\n\n=beta=",
		"[Heading(1, [Text(alpha)]), Heading(1, [Text(beta)])]"
	);
	sl_test!(
		cfg,
		"=alpha=\n\nbeta",
		"[Heading(1, [Text(alpha)]), Text(beta)]"
	);
	sl_test!(
		cfg,
		"=alpha=\n=beta=",
		"[Heading(1, [Text(alpha)]), Heading(1, [Text(beta)])]"
	);
	sl_test!(
		cfg,
		"=alpha=\nbeta",
		"[Heading(1, [Text(alpha)]), Text(beta)]"
	);
	sl_test!(
		cfg,
		"=alpha= \nbeta",
		"[Heading(1, [Text(alpha)]), Text(beta)]"
	);
	sl_test!(cfg, "=alpha==", "[Heading(1, [Text(alpha=)])]");
	sl_test!(
		cfg,
		"alpha\t\n=beta=",
		"[Text(alpha), Heading(1, [Text(beta)])]"
	);
	sl_test!(
		cfg,
		"alpha\n\n=beta=",
		"[Text(alpha), Heading(1, [Text(beta)])]"
	);
	sl_test!(
		cfg,
		"alpha\n\n=beta=\n\ngamma",
		"[Text(alpha), Heading(1, [Text(beta)]), Text(gamma)]"
	);
	sl_test!(
		cfg,
		"alpha\n=beta=",
		"[Text(alpha), Heading(1, [Text(beta)])]"
	);
	sl_test!(
		cfg,
		"alpha\n=beta=\ngamma",
		"[Text(alpha), Heading(1, [Text(beta)]), Text(gamma)]"
	);
	sl_test!(
		cfg,
		"alpha \n=beta=",
		"[Text(alpha), Heading(1, [Text(beta)])]"
	);
}

#[test]
fn horizontal_divider() {
	let cfg = Configuration::default();

	sl_test!(cfg, "----", "[HorizontalDivider]");
	sl_test!(cfg, "----\t\nalpha", "[HorizontalDivider, Text(alpha)]");
	sl_test!(
		cfg,
		"----\n\n\n----",
		"[HorizontalDivider, HorizontalDivider]",
		"[Warning(RepeatedEmptyLine)]"
	);
	sl_test!(
		cfg,
		"----\n\n\nalpha",
		"[HorizontalDivider, Text(alpha)]",
		"[Warning(RepeatedEmptyLine)]"
	);
	sl_test!(
		cfg,
		"----\n\n----",
		"[HorizontalDivider, HorizontalDivider]"
	);
	sl_test!(cfg, "----\n\nalpha", "[HorizontalDivider, Text(alpha)]");
	sl_test!(cfg, "----\n----", "[HorizontalDivider, HorizontalDivider]");
	sl_test!(cfg, "----\nalpha", "[HorizontalDivider, Text(alpha)]");
	sl_test!(cfg, "---- \nalpha", "[HorizontalDivider, Text(alpha)]");
	sl_test!(cfg, "-----", "[HorizontalDivider]");
	sl_test!(cfg, "------", "[HorizontalDivider]");
	sl_test!(cfg, "----alpha", "[HorizontalDivider, Text(alpha)]");
	sl_test!(cfg, "alpha\t\n----", "[Text(alpha), HorizontalDivider]");
	sl_test!(
		cfg,
		"alpha\n\n\n----",
		"[Text(alpha), HorizontalDivider]",
		"[Warning(RepeatedEmptyLine)]"
	);
	sl_test!(cfg, "alpha\n\n----", "[Text(alpha), HorizontalDivider]");
	sl_test!(cfg, "alpha\n \n----", "[Text(alpha), HorizontalDivider]");
	sl_test!(cfg, "alpha\n----", "[Text(alpha), HorizontalDivider]");
	sl_test!(cfg, "alpha \n----", "[Text(alpha), HorizontalDivider]");
}

#[test]
fn invalid_characters() {
	let cfg = Configuration::default();

	sl_test!(cfg, "\0", "[Text(\0)]", "[Warning(InvalidCharacter)]");
	sl_test!(cfg, "\r", "[Text(\r)]", "[Warning(InvalidCharacter)]");
	sl_test!(cfg, "\x7f", "[Text(\u{7f})]", "[Warning(InvalidCharacter)]");
}

#[test]
fn link() {
	let cfg = Configuration::default();

	sl_test!(cfg, "[[FILE:alpha]]", "[Image(FILE:alpha, [])]");
	sl_test!(cfg, "[[File:alpha]]", "[Image(File:alpha, [])]");
	sl_test!(
		cfg,
		"[[alpha",
		"[Text([[alpha)]",
		"[Warning(InvalidLinkSyntax)]"
	);
	sl_test!(
		cfg,
		"[[alpha:beta]]",
		"[Link(alpha:beta, [Text(alpha:beta)])]"
	);
	sl_test!(
		cfg,
		"[[alpha:beta]]gamma",
		"[Link(alpha:beta, [Text(alpha:beta), Text(gamma)])]"
	);
	sl_test!(cfg, "[[alpha]]", "[Link(alpha, [Text(alpha)])]");
	sl_test!(
		cfg,
		"[[alpha]] beta",
		"[Link(alpha, [Text(alpha)]), Text( beta)]"
	);
	sl_test!(
		cfg,
		"[[alpha]]beta",
		"[Link(alpha, [Text(alpha), Text(beta)])]"
	);
	sl_test!(
		cfg,
		"[[alpha]]beta gamma",
		"[Link(alpha, [Text(alpha), Text(beta)]), Text( gamma)]"
	);
	sl_test!(cfg, "[[alpha]]ü", "[Link(alpha, [Text(alpha)]), Text(ü)]");
	sl_test!(
		cfg,
		"[[alpha|",
		"[Text([[alpha|)]",
		"[Warning(MissingEndTagRewinding)]"
	);
	sl_test!(
		cfg,
		"[[alpha|[beta]gamma]]",
		"[Link(alpha, [Text([beta]gamma)])]"
	);
	sl_test!(cfg, "[[alpha|]]", "[Link(alpha, [])]");
	sl_test!(
		cfg,
		"[[alpha|beta",
		"[Text([[alpha|beta)]",
		"[Warning(MissingEndTagRewinding)]"
	);
	sl_test!(
		cfg,
		"[[alpha|beta\ngamma]]",
		"[Link(alpha, [Text(beta\ngamma)])]"
	);
	sl_test!(
		cfg,
		"[[alpha|beta[[gamma]]]]",
		"[Text([[alpha|beta), Link(gamma, [Text(gamma)]), Text(]])]",
		"[Warning(InvalidLinkSyntax)]"
	);
	sl_test!(cfg, "[[alpha|beta]]", "[Link(alpha, [Text(beta)])]");
	sl_test!(
		cfg,
		"[[alpha|beta]]gamma",
		"[Link(alpha, [Text(beta), Text(gamma)]), Text(gamma)]"
	);
	sl_test!(cfg, "[[category:alpha]]", "[Category(category:alpha, [])]");
	sl_test!(
		cfg,
		"[[category:alpha]]beta",
		"[Category(category:alpha, []), Text(beta)]"
	);
	sl_test!(
		cfg,
		"[[category:alpha|beta]]",
		"[Category(category:alpha, [Text(beta)])]"
	);
	sl_test!(cfg, "[[file:alpha]]", "[Image(file:alpha, [])]");
	sl_test!(
		cfg,
		"[[file:alpha]]beta",
		"[Image(file:alpha, []), Text(beta)]"
	);
	sl_test!(
		cfg,
		"[[file:alpha|[[beta]]]]",
		"[Image(file:alpha, [Link(beta, [Text(beta)])])]"
	);
	sl_test!(
		cfg,
		"[[file:alpha|[[beta]]gamma]]",
		"[Image(file:alpha, [Link(beta, [Text(beta), Text(gamma)])])]"
	);
	sl_test!(cfg, "[[file:alpha|]]", "[Image(file:alpha, [])]");
	sl_test!(
		cfg,
		"[[file:alpha|beta[[gamma]]]]",
		"[Image(file:alpha, [Text(beta), Link(gamma, [Text(gamma)])])]"
	);
	sl_test!(
		cfg,
		"[[file:alpha|beta]]",
		"[Image(file:alpha, [Text(beta)])]"
	);
	sl_test!(
		cfg,
		"[[file:alpha|beta]]gamma",
		"[Image(file:alpha, [Text(beta)]), Text(gamma)]"
	);
	sl_test!(cfg, "[[image:alpha]]", "[Image(image:alpha, [])]");
	sl_test!(cfg, "[[|]]", "[Link(, [])]");
	sl_test!(cfg, "[[|alpha]]", "[Link(, [Text(alpha)])]");
	sl_test!(
		cfg,
		"alpha [[beta]]",
		"[Text(alpha ), Link(beta, [Text(beta)])]"
	);
	sl_test!(
		cfg,
		"alpha[[beta]]",
		"[Text(alpha), Link(beta, [Text(beta)])]"
	);
	sl_test!(
		cfg,
		"alpha[[beta]]gamma",
		"[Text(alpha), Link(beta, [Text(beta), Text(gamma)])]"
	);
}

#[test]
fn test_list_part1() {
	let cfg = Configuration::default();

	sl_test!(cfg, "#", "[OrderedList([ListItem([])])]");
	sl_test!(
		cfg,
		"#\n\n\nalpha",
		"[OrderedList([ListItem([])]), Text(alpha)]",
		"[Warning(RepeatedEmptyLine)]"
	);
	sl_test!(
		cfg,
		"#\n\nalpha",
		"[OrderedList([ListItem([])]), Text(alpha)]"
	);
	sl_test!(cfg, "#\n#", "[OrderedList([ListItem([]), ListItem([])])]");
	sl_test!(
		cfg,
		"#\n##",
		"[OrderedList([ListItem([OrderedList([ListItem([])])])])]"
	);
	sl_test!(cfg, "#\n##\n#", "[OrderedList([ListItem([OrderedList([ListItem([])])]), ListItem([])])]");
	sl_test!(
		cfg,
		"#\n*",
		"[OrderedList([ListItem([])]), UnorderedList([ListItem([])])]"
	);
	sl_test!(cfg, "#\n:", "[OrderedList([ListItem([])]), DefinitionList([DefinitionListItem(Details, [])])]");
	sl_test!(cfg, "#\n;", "[OrderedList([ListItem([])]), DefinitionList([DefinitionListItem(Term, [])])]");
	sl_test!(
		cfg,
		"#\nalpha",
		"[OrderedList([ListItem([])]), Text(alpha)]"
	);
	sl_test!(cfg, "# alpha", "[OrderedList([ListItem([Text(alpha)])])]");
	sl_test!(
		cfg,
		"##",
		"[OrderedList([ListItem([OrderedList([ListItem([])])])])]"
	);
	sl_test!(cfg, "##\n#", "[OrderedList([ListItem([OrderedList([ListItem([])])]), ListItem([])])]");
	sl_test!(cfg, "##\n#\n##", "[OrderedList([ListItem([OrderedList([ListItem([])])]), ListItem([OrderedList([ListItem([])])])])]");
	sl_test!(cfg, "##\n##", "[OrderedList([ListItem([OrderedList([ListItem([]), ListItem([])])])])]");
	sl_test!(
		cfg,
		"#=alpha=",
		"[OrderedList([ListItem([Text(=alpha=)])])]"
	);
	sl_test!(cfg, "#alpha", "[OrderedList([ListItem([Text(alpha)])])]");
	sl_test!(
		cfg,
		"#alpha\n#beta",
		"[OrderedList([ListItem([Text(alpha)]), ListItem([Text(beta)])])]"
	);
	sl_test!(cfg, "*", "[UnorderedList([ListItem([])])]");
	sl_test!(
		cfg,
		"*\n\nalpha",
		"[UnorderedList([ListItem([])]), Text(alpha)]"
	);
	sl_test!(
		cfg,
		"*\n#",
		"[UnorderedList([ListItem([])]), OrderedList([ListItem([])])]"
	);
	sl_test!(cfg, "*\n*", "[UnorderedList([ListItem([]), ListItem([])])]");
	sl_test!(
		cfg,
		"*\n**",
		"[UnorderedList([ListItem([UnorderedList([ListItem([])])])])]"
	);
	sl_test!(cfg, "*\n**\n*", "[UnorderedList([ListItem([UnorderedList([ListItem([])])]), ListItem([])])]");
	sl_test!(cfg, "*\n:", "[UnorderedList([ListItem([])]), DefinitionList([DefinitionListItem(Details, [])])]");
	sl_test!(cfg, "*\n;", "[UnorderedList([ListItem([])]), DefinitionList([DefinitionListItem(Term, [])])]");
	sl_test!(
		cfg,
		"*\nalpha",
		"[UnorderedList([ListItem([])]), Text(alpha)]"
	);
	sl_test!(cfg, "* alpha", "[UnorderedList([ListItem([Text(alpha)])])]");
	sl_test!(
		cfg,
		"* alpha\n* beta",
		"[UnorderedList([ListItem([Text(alpha)]), ListItem([Text(beta)])])]"
	);
	sl_test!(
		cfg,
		"**",
		"[UnorderedList([ListItem([UnorderedList([ListItem([])])])])]"
	);
	sl_test!(cfg, "**\n*", "[UnorderedList([ListItem([UnorderedList([ListItem([])])]), ListItem([])])]");
	sl_test!(cfg, "**\n*\n**", "[UnorderedList([ListItem([UnorderedList([ListItem([])])]), ListItem([UnorderedList([ListItem([])])])])]");
	sl_test!(cfg, "**\n**", "[UnorderedList([ListItem([UnorderedList([ListItem([]), ListItem([])])])])]");
	sl_test!(cfg, "*;\n*;", "[UnorderedList([ListItem([DefinitionList([DefinitionListItem(Term, []), DefinitionListItem(Term, [])])])])]");
	sl_test!(cfg, "*;\n*;*", "[UnorderedList([ListItem([DefinitionList([DefinitionListItem(Term, [])]), DefinitionList([DefinitionListItem(Term, [UnorderedList([ListItem([])])])])])])]", "[Warning(DefinitionTermContinuation)]");
}

#[test]
fn test_list_part2() {
	let cfg = Configuration::default();

	sl_test!(cfg, "*;*\n*;", "[UnorderedList([ListItem([DefinitionList([DefinitionListItem(Term, [UnorderedList([ListItem([])])])]), DefinitionList([DefinitionListItem(Term, [])])])])]", "[Warning(DefinitionTermContinuation)]");
	sl_test!(cfg, "*;*\n*;#", "[UnorderedList([ListItem([DefinitionList([DefinitionListItem(Term, [UnorderedList([ListItem([])])])]), DefinitionList([DefinitionListItem(Term, [OrderedList([ListItem([])])])])])])]", "[Warning(DefinitionTermContinuation)]");
	sl_test!(cfg, "*=alpha=", "[UnorderedList([ListItem([Text(=alpha=)])])]", "[]");
	sl_test!(cfg, "*alpha", "[UnorderedList([ListItem([Text(alpha)])])]", "[]");
	sl_test!(cfg, "*alpha\n*beta", "[UnorderedList([ListItem([Text(alpha)]), ListItem([Text(beta)])])]", "[]");
	sl_test!(cfg, ":", "[DefinitionList([DefinitionListItem(Details, [])])]", "[]");
	sl_test!(cfg, ":\n\nalpha", "[DefinitionList([DefinitionListItem(Details, [])]), Text(alpha)]", "[]");
	sl_test!(cfg, ":\n#", "[DefinitionList([DefinitionListItem(Details, [])]), OrderedList([ListItem([])])]", "[]");
	sl_test!(cfg, ":\n*", "[DefinitionList([DefinitionListItem(Details, [])]), UnorderedList([ListItem([])])]", "[]");
	sl_test!(cfg, ":\n:", "[DefinitionList([DefinitionListItem(Details, []), DefinitionListItem(Details, [])])]", "[]");
	sl_test!(cfg, ":\n::", "[DefinitionList([DefinitionListItem(Details, [DefinitionList([DefinitionListItem(Details, [])])])])]", "[]");
	sl_test!(cfg, ":\n::\n:", "[DefinitionList([DefinitionListItem(Details, [DefinitionList([DefinitionListItem(Details, [])])]), DefinitionListItem(Details, [])])]", "[]");
	sl_test!(cfg, ":\n;", "[DefinitionList([DefinitionListItem(Details, []), DefinitionListItem(Term, [])])]", "[]");
	sl_test!(cfg, ":\nalpha", "[DefinitionList([DefinitionListItem(Details, [])]), Text(alpha)]", "[]");
	sl_test!(cfg, ": alpha", "[DefinitionList([DefinitionListItem(Details, [Text(alpha)])])]", "[]");
	sl_test!(cfg, "::", "[DefinitionList([DefinitionListItem(Details, [DefinitionList([DefinitionListItem(Details, [])])])])]", "[]");
	sl_test!(cfg, "::\n:", "[DefinitionList([DefinitionListItem(Details, [DefinitionList([DefinitionListItem(Details, [])])]), DefinitionListItem(Details, [])])]", "[]");
	sl_test!(cfg, "::\n:\n::", "[DefinitionList([DefinitionListItem(Details, [DefinitionList([DefinitionListItem(Details, [])])]), DefinitionListItem(Details, [DefinitionList([DefinitionListItem(Details, [])])])])]", "[]");
	sl_test!(cfg, "::\n::", "[DefinitionList([DefinitionListItem(Details, [DefinitionList([DefinitionListItem(Details, []), DefinitionListItem(Details, [])])])])]", "[]");
	sl_test!(cfg, ":=alpha=", "[DefinitionList([DefinitionListItem(Details, [Text(=alpha=)])])]", "[]");
	sl_test!(cfg, ":alpha", "[DefinitionList([DefinitionListItem(Details, [Text(alpha)])])]", "[]");
	sl_test!(cfg, ":alpha\nbeta", dbg);
	sl_test!(cfg, ";", dbg);
	sl_test!(cfg, ";\n\nalpha", dbg);
	sl_test!(cfg, ";\n#", dbg);
	sl_test!(cfg, ";\n*", dbg);
	sl_test!(cfg, ";\n:", dbg);
	sl_test!(cfg, ";\n;;", dbg);
	sl_test!(cfg, ";\n;;\n;", dbg);
	sl_test!(cfg, ";\nalpha", dbg);
	sl_test!(cfg, "; alpha", dbg);
	sl_test!(cfg, ";;", dbg);
	sl_test!(cfg, ";;\n;", dbg);
	sl_test!(cfg, ";;\n;\n;;", dbg);
	sl_test!(cfg, ";;\n;;", dbg);
	sl_test!(cfg, ";=alpha=", dbg);
	sl_test!(cfg, ";alpha", dbg);
	sl_test!(cfg, ";alpha\nbeta", dbg);
	sl_test!(cfg, "alpha\t\n#", dbg);
	sl_test!(cfg, "alpha\n#", dbg);
	sl_test!(cfg, "alpha\n#\nbeta", dbg);
	sl_test!(cfg, "alpha\n*", dbg);
	sl_test!(cfg, "alpha\n*\nbeta", dbg);
	sl_test!(cfg, "alpha\n:", dbg);
	sl_test!(cfg, "alpha\n:\nbeta", dbg);
	sl_test!(cfg, "alpha\n;", dbg);
	sl_test!(cfg, "alpha\n;\nbeta", dbg);
	sl_test!(cfg, "alpha \n#", dbg);

	assert!(false);
}

const TEST_CASES: &[(&str, &[&str])] = &[
	(
		"magic word",
		&[
			"__ALPHA__",
			"__NOTC__ __TOC__",
			"__NOTC___TOC__",
			"__NOTC____TOC__",
			"__TOC_",
			"__TOC__",
			"__TOC__ alpha",
			"__TOC__alpha",
			"__ToC__",
			"__tOc__",
			"__toc__",
			"alpha __TOC__",
			"alpha __TOC__ beta",
			"alpha__TOC__",
			"alpha__TOC__beta",
		],
	),
	(
		"mix",
		&[
			" alpha\n {|\n beta\n |}\n gamma",
			" alpha\n {|\n|}",
			" alpha\n |}",
			" alpha\n |}\n beta",
			" {|\n alpha\n |}",
			" {|\n alpha\n|}",
			"*\n  alpha\n*",
			"----\t\n*",
			"----\n\n*",
			"----\n*",
			"----\n*\nalpha",
			"---- \n*",
			"<ref><!--",
			"=alpha=\n\n----",
			"=alpha=\n----",
			"{{alpha|<!--",
			"{|\n alpha\n |}",
			"{|\n alpha\n|}",
			"{|\n|}\t\n*",
			"{|\n|}\n*",
			"{|\n|}\n*\nalpha",
			"{|\n|} \n*",
		],
	),
	(
		"nowiki",
		&[
			"<MATH>''</MATH>",
			"<NOWIKI>''</NOWIKI>",
			"<mAtH>''</MaTh>",
			"<math>''</math>",
			"<math>''alpha",
			"<nOwIkI>''</NoWiKi>",
			"<nowiki>\n*alpha\n</nowiki>",
			"<nowiki>\n=alpha=\n</nowiki>",
			"<nowiki>''</nowiki>",
			"<nowiki>''alpha",
			"<nowiki><!-- alpha --></nowiki>",
			"<nowiki>{{</nowiki>",
			"<nowiki>{{alpha}}</nowiki>",
			"<nowiki>}}</nowiki>",
		],
	),
	(
		"paragraph break",
		&[
			"alpha\t\n\nbeta",
			"alpha\n\t\nbeta",
			"alpha\n\n\t beta",
			"alpha\n\n\tbeta",
			"alpha\n\n\n\nbeta",
			"alpha\n\n\nbeta",
			"alpha\n\nbeta",
			"alpha\n \nbeta",
			"alpha \n\nbeta",
		],
	),
	(
		"parameter",
		&[
			"*alpha}}}",
			"[[alpha|beta}}}]]",
			"{{{",
			"{{{\talpha}}}",
			"{{{\nalpha}}}",
			"{{{''}}}",
			"{{{[[alpha|beta}}}",
			"{{{alpha\t|beta}}}",
			"{{{alpha\t}}}",
			"{{{alpha\n|beta}}}",
			"{{{alpha\n}}}",
			"{{{alpha |beta}}}",
			"{{{alpha }}}",
			"{{{alpha|",
			"{{{alpha|\tbeta}}}",
			"{{{alpha|\t|}}}",
			"{{{alpha|\t}}}",
			"{{{alpha|\nbeta}}}",
			"{{{alpha|\n|}}}",
			"{{{alpha|\n}}}",
			"{{{alpha| beta|}}}",
			"{{{alpha| |}}}",
			"{{{alpha| }}}",
			"{{{alpha|beta\t|}}}",
			"{{{alpha|beta\n|}}}",
			"{{{alpha|beta |}}}",
			"{{{alpha|beta|",
			"{{{alpha|beta|\n}}}",
			"{{{alpha|beta|gamma}}}",
			"{{{alpha|beta|}}}",
			"{{{alpha|beta}}}",
			"{{{alpha|}}}",
			"{{{alpha}}}",
			"{{{|''}}}",
			"{{{||}}}",
			"{{{|}}}",
			"{{{}}}",
			"}}}",
		],
	),
	(
		"preformatted block",
		&[
			"  alpha",
			" alpha",
			" alpha\n\n\nbeta",
			" alpha\n\nbeta",
			" alpha\n beta",
			" alpha\n beta\n gamma",
			" alpha\n beta\ngamma",
			" alpha\nbeta",
			" alpha\nbeta\n gamma",
			"alpha\t\n beta",
			"alpha\n\n beta",
			"alpha\n \n beta",
			"alpha\n =beta=\ngamma",
			"alpha\n beta",
			"alpha\n beta\n gamma",
			"alpha\n beta\ngamma",
			"alpha \n beta",
		],
	),
	(
		"redirect",
		&[
			"\t#REDIRECT[[alpha]]",
			"\n\n#REDIRECT[[alpha]]",
			"\n #REDIRECT[[alpha]]",
			"\n#REDIRECT  [[alpha]]",
			" \n#REDIRECT[[alpha]]",
			"  #REDIRECT[[alpha]]",
			" #REDIRECT[[alpha]]",
			"#REDIRECT\t:[[alpha]]",
			"#REDIRECT\t[[alpha]]",
			"#REDIRECT\n\n[[alpha]]",
			"#REDIRECT\n [[alpha]]",
			"#REDIRECT\n:\n[[alpha]]",
			"#REDIRECT\n:[[alpha]]",
			"#REDIRECT\n[[alpha]]",
			"#REDIRECT \n[[alpha]]",
			"#REDIRECT  [[alpha]]",
			"#REDIRECT : [[alpha]]",
			"#REDIRECT :[[alpha]]",
			"#REDIRECT [[alpha]]",
			"#REDIRECT:\t[[alpha]]",
			"#REDIRECT:\n[[alpha]]",
			"#REDIRECT: [[alpha]]",
			"#REDIRECT:[[alpha]]",
			"#REDIRECT[[alpha]]",
			"#REDIRECT[[alpha]]\n\nbeta",
			"#REDIRECT[[alpha]]\n beta",
			"#REDIRECT[[alpha]]\nbeta",
			"#REDIRECT[[alpha]] \nbeta",
			"#REDIRECT[[alpha]]  beta",
			"#REDIRECT[[alpha]] beta",
			"#REDIRECT[[alpha]]''beta",
			"#REDIRECT[[alpha]]beta",
			"#REDIRECT[[alpha|]]",
			"#REDIRECT[[alpha|]]beta",
			"#REDIRECT[[alpha|beta\ngamma]]",
			"#REDIRECT[[alpha|beta]]",
			"#REDIRECT[[alpha|beta]]=gamma=",
			"#REDIRECT[[alpha|beta]]gamma",
			"#ReDiReCt[[alpha]]",
			"#rEdIrEcT[[alpha]]",
			"#redirect[[alpha]]",
		],
	),
	(
		"table",
		&[
			" {|\n |}",
			" {|\n|}",
			"alpha\n{|\nbeta\n|}",
			"{|",
			"{|\n |}",
			"{|\n!\n alpha\n|}",
			"{|\n!\n!\n|}",
			"{|\n!\nalpha\n\nbeta\n|}",
			"{|\n!\nalpha\n\n|}",
			"{|\n!\nalpha\nbeta\n|}",
			"{|\n!\nalpha \n|}",
			"{|\n!\n|\n|}",
			"{|\n!\n|-\n|}",
			"{|\n!\n|}",
			"{|\n! alpha\n|}",
			"{|\n!!\n|}",
			"{|\n!!!\n|}",
			"{|\n!!!!\n|}",
			"{|\n!!!|\n|}",
			"{|\n!alpha\n\nbeta\n|}",
			"{|\n!alpha\nbeta\n|}",
			"{|\n!alpha\nbeta|gamma\n|}",
			"{|\n!alpha\n|}",
			"{|\n!alpha!!beta\n|}",
			"{|\n!alpha!beta\n|}",
			"{|\n!alpha|beta\n|}",
			"{|\n!alpha||beta\n|}",
			"{|\n!|\n|}",
			"{|\n!|!!\n|}",
			"{|\n!|alpha\n|}",
			"{|\n!|alpha|beta\n|}",
			"{|\n!||\n|}",
			"{|\n!||alpha\n|}",
			"{|\n!|||\n|}",
			"{|\n*alpha\n|}",
			"{|\n=alpha=\n|}",
			"{|\nalpha\n|}",
			"{|\n|",
			"{|\n|\n alpha\n|}",
			"{|\n|\n!\n|}",
			"{|\n|\n*alpha\n|}",
			"{|\n|\n=alpha=\n|}",
			"{|\n|\nalpha\n\nbeta\n|}",
			"{|\n|\nalpha\n\n|}",
			"{|\n|\nalpha\nbeta\n|}",
			"{|\n|\nalpha \n|}",
			"{|\n|\n|\n|}",
			"{|\n|\n|-\n|}",
			"{|\n|\n|}",
			"{|\n| alpha\n|}",
			"{|\n|+\n alpha\n|}",
			"{|\n|+\n*alpha\n|}",
			"{|\n|+\n=alpha=\n|}",
			"{|\n|+\nalpha\n\nbeta\n|}",
			"{|\n|+\nalpha\nbeta\n|}",
			"{|\n|+\nalpha\n|}",
			"{|\n|+\n|+\n|}",
			"{|\n|+\n|}",
			"{|\n|+ alpha\n|}",
			"{|\n|+!!\n|}",
			"{|\n|+alpha\n\nbeta\n|}",
			"{|\n|+alpha\nbeta\n|}",
			"{|\n|+alpha\n|}",
			"{|\n|+alpha \n|}",
			"{|\n|+|\n|}",
			"{|\n|+|alpha|\n|}",
			"{|\n|+|alpha|beta\n|}",
			"{|\n|+||\n|}",
			"{|\n|+||alpha\n|}",
			"{|\n|+|||\n|}",
			"{|\n|-\n alpha\n|}",
			"{|\n|-\n!\n|}",
			"{|\n|-\n*alpha\n|}",
			"{|\n|-\n=alpha=\n|}",
			"{|\n|-\nalpha\n|}",
			"{|\n|-\n|\n|}",
			"{|\n|-\n|-\n|}",
			"{|\n|-\n|}",
			"{|\n|- alpha\n|}",
			"{|\n|-alpha\n\n|}",
			"{|\n|-alpha\n|}",
			"{|\n|-alpha \n|}",
			"{|\n|alpha\n\nbeta\n|}",
			"{|\n|alpha\nbeta\n|}",
			"{|\n|alpha\nbeta|gamma\n|}",
			"{|\n|alpha\n|}",
			"{|\n|alpha!!beta\n|}",
			"{|\n|alpha!beta\n|}",
			"{|\n|alpha|\n|}",
			"{|\n|alpha|beta\n|}",
			"{|\n|alpha||beta\n|}",
			"{|\n||\n|}",
			"{|\n||alpha\n|}",
			"{|\n|||\n|}",
			"{|\n||||\n|}",
			"{|\n|}",
			"{|\n|}\t\nalpha",
			"{|\n|}\n\n\nalpha",
			"{|\n|}\n\nalpha",
			"{|\n|}\nalpha",
			"{|\n|} \nalpha",
			"{|\n|}alpha",
			"{|alpha\nbeta\n|}",
			"{|alpha\n|}",
		],
	),
	(
		"tag",
		&[
			"</BR>",
			"</Br>",
			"</alpha",
			"</alpha>",
			"</b",
			"</b alpha>",
			"</b alpha>beta",
			"</b</b>",
			"</b<b>",
			"</b>",
			"</b> alpha",
			"</b>alpha",
			"</br\t>",
			"</br\n>",
			"</br >",
			"</br>",
			"</ref",
			"<BR>",
			"<Br>",
			"<alpha",
			"<alpha>",
			"<b",
			"<b alpha>",
			"<b alpha>beta",
			"<b</b>",
			"<b<b>",
			"<b>",
			"<b> alpha",
			"<b>alpha",
			"<br\t>",
			"<br\n>",
			"<br >",
			"<br>",
			"<r<ref>alpha</ref>beta",
			"<ref",
			"<ref />",
			"<ref >",
			"<ref/>",
			"<ref>",
			"<ref>\talpha</ref>",
			"<ref>\nalpha</ref>",
			"<ref> alpha</ref>",
			"<ref></ref>",
			"<ref>alpha\t</ref>",
			"<ref>alpha\n</ref>",
			"<ref>alpha </ref>",
			"<ref>alpha</ref>",
			"alpha<b>",
		],
	),
	(
		"template",
		&[
			"*alpha}}",
			"[[alpha|beta}}]]",
			"alpha {{beta}}",
			"alpha {{beta}} gamma",
			"alpha{{beta}}",
			"alpha{{beta}}gamma",
			"{{\nalpha}}",
			"{{''}}",
			"{{[[alpha|beta}}",
			"{{alpha",
			"{{alpha\n|beta}}",
			"{{alpha\n|}}",
			"{{alpha\n}}",
			"{{alpha|",
			"{{alpha|\nbeta}}",
			"{{alpha|\n}}",
			"{{alpha| beta}}",
			"{{alpha|''}}",
			"{{alpha|beta",
			"{{alpha|beta\n=gamma}}",
			"{{alpha|beta\n}}",
			"{{alpha|beta =gamma}}",
			"{{alpha|beta }}",
			"{{alpha|beta=\ngamma}}",
			"{{alpha|beta= gamma}}",
			"{{alpha|beta=gamma\n}}",
			"{{alpha|beta=gamma }}",
			"{{alpha|beta=gamma=delta}}",
			"{{alpha|beta=gamma|delta=epsilon}}",
			"{{alpha|beta=gamma|delta}}",
			"{{alpha|beta=gamma}}",
			"{{alpha|beta=}}",
			"{{alpha|beta|gamma=delta}}",
			"{{alpha|beta|gamma}}",
			"{{alpha|beta}",
			"{{alpha|beta}}",
			"{{alpha|beta}} gamma",
			"{{alpha|beta}}gamma",
			"{{alpha|}",
			"{{alpha|}}",
			"{{alpha}",
			"{{alpha}}",
			"{{alpha}} beta",
			"{{alpha}}beta",
			"}}",
		],
	),
];
