pub fn search_case_sensitive <'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
	let mut results: Vec<&str> = Vec::new();

	for line in contents.lines() {
		if line.contains(&query) {
			results.push(line);
		}
	}

    results
}

pub fn search_case_insensitive <'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
	let query = query.to_lowercase();
	let mut results: Vec<&str> = Vec::new();

	for line in contents.lines() {
		if line.to_lowercase().contains(&query) {
			results.push(line);
		}
	}

    results
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn one_result() {
		let query = "fast";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

		assert_eq!(vec!["safe, fast, productive."], search_case_sensitive(query, contents));
	}

	#[test]
	fn case_insensitive() {
		let query = "rUst";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

		assert_eq!(vec!["Rust:", "Trust me."], search_case_insensitive(query, contents));
	}
}