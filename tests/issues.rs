use std::time::{Duration, Instant};

use parse_wiki_text_2::Configuration;

const MAX_EXEC: Duration = Duration::from_millis(200);

#[test]
fn issue_1() {
	let start = Instant::now();
	let s = "{".repeat(40);
	let _ = Configuration::default().parse(&s);

	eprintln!("elapsed {}ms", start.elapsed().as_millis());
	if start.elapsed() > MAX_EXEC {
		panic!("long recursion took {}ms", start.elapsed().as_millis());
	}
}
