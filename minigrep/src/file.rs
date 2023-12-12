use std::fs;

#[derive(Debug)]
pub struct File {
    pub contents: String,
}

impl PartialEq for File {
    fn eq(&self, rhs: &Self) -> bool {
        self.contents == rhs.contents
    }
}

impl File {
    pub fn new(filename: &str) -> Result<File, std::io::Error> {
        Ok(File {
            contents: fs::read_to_string(filename)?,
        })
    }

    pub fn search(&self, pattern: &str, ignore_case: bool) -> Result<SearchResult, &'static str> {
        let contents = if ignore_case {
            self.contents.to_lowercase()
        } else {
            self.contents.clone()
        };
        let pattern = if ignore_case {
            pattern.to_lowercase()
        } else {
            String::from(pattern)
        };
        Ok(SearchResult {
            matching_lines: contents
                .lines()
                .filter(|line| line.contains(&pattern))
                .map(|line| String::from(line))
                .collect(),
        })
    }
}

#[derive(Debug)]
pub struct SearchResult {
    pub matching_lines: Vec<String>,
}

impl PartialEq for SearchResult {
    fn eq(&self, rhs: &Self) -> bool {
        self.matching_lines == rhs.matching_lines
    }
}

impl SearchResult {
    pub fn print(&self) {
        if self.matching_lines.is_empty() {
            println!("no results");
        } else {
            println!("found results:");
            for line in &self.matching_lines {
                println!("{line}");
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn search_no_match() {
        assert_eq!(
            File {
                contents: String::from("a"),
            }
            .search("b", false),
            Ok(SearchResult {
                matching_lines: vec![]
            })
        );
    }

    #[test]
    fn search_one_match() {
        assert_eq!(
            File {
                contents: String::from("a")
            }
            .search("a", false),
            Ok(SearchResult {
                matching_lines: vec![String::from("a")]
            })
        );
    }

    #[test]
    fn search_n_matches() {
        assert_eq!(
            File {
                contents: String::from("a\nb\na")
            }
            .search("a", false),
            Ok(SearchResult {
                matching_lines: vec![String::from("a"), String::from("a")]
            })
        );
    }

    #[test]
    fn search_ignore_case_match() {
        assert_eq!(
            File {
                contents: String::from("A")
            }
            .search("a", true),
            Ok(SearchResult {
                matching_lines: vec![String::from("a")]
            })
        );
    }
}
