pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

pub fn search_case_insensitive<'a>(
    query: &str,
    contents: &'a str,
) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }

    results
}

pub fn count_occurrences<'a>(query: &str, contents: &'a str) -> u32 { 
    let query = query.to_lowercase();
    let mut occurrences: u32 = 0;

    for line in contents.lines() {
        let matches: Vec<&str> = line.matches(&query).collect();
        let count = matches.len() as u32;
        if count > 0 {
            occurrences += count;
        }
    }

    occurrences
}

pub fn count_occurrences_insensitive<'a>(query: &str, contents: &'a str) -> u32 {
    let query = query.to_lowercase();
    let mut occurrences: u32 = 0;

    for line in contents.lines() {
        let line_lowercase = line.to_lowercase();
        let matches: Vec<&str> = line_lowercase.matches(&query).collect();
        let count = matches.len() as u32;
        if count > 0 {
            occurrences += count;
        }
    }

    occurrences
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }

    #[test]
    fn count_sensitive_occurrences() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(1, count_occurrences(query, contents));
    }

    #[test]
    fn count_insensitive_occurrences() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(2, count_occurrences_insensitive(query, contents));
    }
}