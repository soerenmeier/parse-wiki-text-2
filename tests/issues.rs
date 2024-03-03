use std::time::Duration;

use parse_wiki_text_2::{Configuration, ParseError};

const MAX_EXEC: Duration = Duration::from_millis(200);

#[test]
fn issue_1() {
	let s = "{".repeat(40);
	let r = Configuration::default().parse_with_timeout(&s, MAX_EXEC);

	// todo fix this so we don't get an error
	match r {
		Err(ParseError::TimedOut {
			execution_time,
			output,
		}) => {
			let dif = execution_time - MAX_EXEC;
			assert!(
				dif < Duration::from_millis(10),
				"expected timeout to be within 10ms of MAX_EXEC, got {:?}",
				dif
			);

			assert!(
				!output.warnings.is_empty(),
				"expected warnings to be present"
			)
		}
		_ => panic!("expected timeout"),
	}
}
