<!--
Copyright 2019 Fredrik Portström <https://portstrom.com>
This is free software distributed under the terms specified in
the file LICENSE at the top-level directory of this distribution.
-->

# Parse Wiki Text

Parse wiki text from Mediawiki into a tree of elements.

![Parse Wiki Text](https://portstrom.com/parse_wiki_text.svg)

## Introduction

Wiki text is a format that follows the PHP maxim “Make everything as inconsistent and confusing as possible”. There are hundreds of millions of interesting documents written in this format, distributed under free licenses on sites that use the Mediawiki software, mainly Wikipedia and Wiktionary. Being able to parse wiki text and process these documents would allow access to a significant part of the world's knowledge.

The Mediawiki software itself transforms a wiki text document into an HTML document in an outdated format to be displayed in a browser for a human reader. It does so through a [step by step procedure](https://www.mediawiki.org/wiki/Manual:Parser.php) of string substitutions, with some of the steps depending on the result of previous steps. [The main file for this procedure](https://doc.wikimedia.org/mediawiki-core/master/php/Parser_8php_source.html) has 6200 lines of code and the [second biggest file](https://doc.wikimedia.org/mediawiki-core/master/php/Preprocessor__DOM_8php_source.html) has 2000, and then there is a [1400 line file](https://doc.wikimedia.org/mediawiki-core/master/php/ParserOptions_8php_source.html) just to take options for the parser.

What would be more interesting is to parse the wiki text document into a structure that can be used by a computer program to reason about the facts in the document and present them in different ways, making them available for a great variety of applications.

Some people have tried to parse wiki text using regular expressions. This is incredibly naive and fails as soon as the wiki text is non-trivial. The capabilities of regular expressions don't come anywhere close to the complexity of the weirdness required to correctly parse wiki text. One project did a brave attempt to use a parser generator to parse wiki text. Wiki text was however never designed for formal parsers, so even parser generators are of no help in correctly parsing wiki text.

Wiki text has a long history of poorly designed additions carelessly piled on top of each other. The syntax of wiki text is different in each wiki depending on its configuration. You can't even know what's a start tag until you see the corresponding end tag, and you can't know where the end tag is unless you parse the entire hierarchy of nested tags between the start tag and the end tag. In short: If you think you understand wiki text, you don't understand wiki text.

Parse Wiki Text attempts to take all uncertainty out of parsing wiki text by converting it to another format that is easy to work with. The target format is Rust objects that can ergonomically be processed using iterators and match expressions.

## Design goals

### Correctness

Parse Wiki Text is designed to parse wiki text exactly as parsed by Mediawiki. Even when there is obviously a bug in Mediawiki, Parse Wiki Text replicates that exact bug. If there is something Parse Wiki Text doesn't parse exactly the same as Mediawiki, please report it as an issue.

### Speed

Parse Wiki Text is designed to parse a page in as little time as possible. It parses tens of thousands of pages per second on each processor core and can quickly parse an entire wiki with millions of pages. If there is anything that can be changed to make Parse Wiki Text faster, please report it as an issue.

### Safety

Parse Wiki Text is designed to work with untrusted inputs. If any input doesn't parse safely with reasonable resources, please report it as an issue. No unsafe code is used.

### Platform support

Parse Wiki Text is designed to run in a wide variety of environments, such as:

- servers running machine code
- browsers running Web Assembly
- embedded in other programming languages

Parse Wiki Text can be deployed anywhere with no dependencies.

## Caution

Wiki text is a legacy format used by legacy software. Parse Wiki Text is intended only to recover information that has been written for wikis running legacy software, replicating the exact bugs found in the legacy software. Please don't use wiki text as a format for new applications. Wiki text is a horrible format with an astonishing amount of inconsistencies, bad design choices and bugs. For new applications, please use a format that is designed to be easy to process, such as JSON or even better [CBOR](http://cbor.io). See [Wikidata](https://www.wikidata.org/wiki/Wikidata:Main_Page) for an example of a wiki that uses JSON as its format and provides a rich interface for editing data instead of letting people write code. If you need to take information written in wiki text and reuse it in a new application, you can use Parse Wiki Text to convert it to an intermediate format that you can further process into a modern format.

## Site configuration

Wiki text has plenty of features that are parsed in a way that depends on the configuration of the wiki. This means the configuration must be known before parsing.

- External links are parsed only when the scheme of the URI of the link is in the configured list of valid protocols. When the scheme is not valid, the link is parsed as plain text.
- Categories and images superficially look they same way as links, but are parsed differently. These can only be distinguished by knowing the namespace aliases from the configuration of the wiki.
- Text matching the configured set of magic words is parsed as magic words.
- Extension tags have the same syntax as HTML tags, but are parsed differently. The configuration tells which tag names are to be treated as extension tags.

The configuration can be seen by making a request to the [site info](https://www.mediawiki.org/wiki/API:Siteinfo) resource on the wiki. The utility [Fetch site configuration](https://github.com/portstrom/fetch_mediawiki_configuration) fetches the parts of the configuration needed for parsing pages in the wiki, and outputs Rust code for instantiating a parser with that configuration. Parse Wiki Text contains a default configuration that can be used for testing.

## Limitations

Wiki text was never designed to be possible to parse into a structured format. It's designed to be parsed in multiple passes, where each pass depends on the output on the previous pass. Most importantly, templates are expanded in an earlier pass and formatting codes are parsed in a later pass. This means the formatting codes you see in the original text are not necessarily the same as the parser will see after templates have been expanded. Luckily this is as bad for human editors as it is for computers, so people tend to avoid writing templates that cause formatting codes to be parsed in a way that differs from what they would expect from reading the original wiki text before expanding templates. Parse Wiki Text assumes that templates never change the meaning of formatting codes around them.

## Sandbox

A sandbox ([Github](https://github.com/portstrom/parse_wiki_text_sandbox), [try online](https://portstrom.com/parse_wiki_text_sandbox/)) is available that allows interactively entering wiki text and inspecting the result of parsing it.

## Comparison with Mediawiki Parser

There is another crate called Mediawiki Parser ([crates.io](https://crates.io/crates/mediawiki_parser), [Github](https://github.com/vroland/mediawiki-parser)) that does basically the same thing, parsing wiki text to a tree of elements. That crate however doesn't take into account any of the astonishing amount of weirdness required to correctly parse wiki text. That crate admittedly only parses a subset of wiki text, with the intention to report errors for any text that is too weird to fit that subset, which is a good intention, but when examining it, that subset is quickly found to be too small to parse pages from actual wikis, and even worse, the error reporting is just an empty promise, and there's no indication when a text is incorrectly parsed.

That crate could possibly be improved to always report errors when a text isn't in the supported subset, but pages found in real wikis very often don't conform to the small subset of wiki text that can be parsed without weirdness, so it still wouldn't be useful. Improving that crate to correctly parse a large enough subset of wiki text would be as much effort as starting over from scratch, which is why Parse Wiki Text was made without taking anything from Mediawiki Parser. Parse Wiki Text aims to correctly parse all wiki text, not just a subset, and report warnings when encountering weirdness that should be avoided.

## Examples

The default configuration is used for testing purposes only.
For parsing a real wiki you need a site-specific configuration.
Reuse the same configuration when parsing multiple pages for efficiency.

```rust
use parse_wiki_text::{Configuration, Node};
let wiki_text = concat!(
    "==Our values==\n",
    "*Correctness\n",
    "*Speed\n",
    "*Ergonomics"
);
let result = Configuration::default().parse(wiki_text);
assert!(result.warnings.is_empty());
for node in result.nodes {
    if let Node::UnorderedList { items, .. } = node {
        println!("Our values are:");
        for item in items {
            println!("- {}", item.nodes.iter().map(|node| match node {
                Node::Text { value, .. } => value,
                _ => ""
            }).collect::<String>());
        }
    }
}
```
