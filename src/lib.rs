pub mod minigrep {
    use std::{
        env,
        io::{BufRead, BufReader, Error, Read},
        vec,
    };
    pub trait Searchable {
        fn search(&mut self, pat: &str) -> Result<Vec<&str>, Error>;
    }

    pub struct SearchInterface<T>
    where
        T: Read,
    {
        ignore_case: bool,
        buf_reader: BufReader<T>,
        matched_lines: Vec<String>,
    }

    impl<T> Searchable for SearchInterface<T>
    where
        T: Read,
    {
        fn search<'b>(&mut self, pat: &'b str) -> Result<Vec<&str>, Error> {
            let search_str = if self.ignore_case {
                pat.to_lowercase()
            } else {
                pat.to_string()
            };
            let mut line = String::new();
            loop {
                if self.buf_reader.read_line(&mut line)? == 0 {
                    break;
                }
                if self.ignore_case {
                    if line.to_lowercase().contains(&search_str) {
                        self.matched_lines.push(line.clone());
                    }
                } else {
                    if line.contains(&search_str) {
                        self.matched_lines.push(line.clone());
                    }
                }
                line.clear();
            }
            let mut matched_lines: Vec<&str> = vec![];
            for line in &self.matched_lines {
                matched_lines.push(line.trim());
            }
            Ok(matched_lines)
        }
    }

    impl<T> SearchInterface<T>
    where
        T: Read,
    {
        pub fn new(ignore_case: Option<bool>, input_src: T) -> Self {
            let env_var = env::var("IGNORE_CASE").unwrap_or("0".to_string());
            let default_flag = if env_var == "1".to_string() {
                true
            } else {
                false
            };
            SearchInterface {
                ignore_case: ignore_case.unwrap_or(default_flag),
                buf_reader: BufReader::new(input_src),
                matched_lines: vec![],
            }
        }
    }
}
#[cfg(test)]
mod tests {

    use crate::minigrep::{SearchInterface, Searchable};

    #[test]
    fn case_sensitive_search_1() {
        let actual = vec![
            "2: Apple announced a new iPhone today.",
            "3: The company is named Apple, but they sell fruit?",
        ];
        let mut search_object = SearchInterface::new(
            Some(false),
            b"1: An apple a day keeps the doctor away.
2: Apple announced a new iPhone today.
3: The company is named Apple, but they sell fruit?
4: I prefer a green apple over a red one."
                .as_slice(),
        );
        let res = search_object.search("Apple").unwrap();
        assert_eq!(actual, res);
    }
    #[test]
    fn case_sensitive_search_2() {
        let actual = vec!["1: help me!", "3: He is very helpful."];
        let mut search_object = SearchInterface::new(
            Some(false),
            b"1: help me!
2: I need some HELP over here.
3: He is very helpful.
4: Can anyone HElp?"
                .as_slice(),
        );
        let res = search_object.search("help").unwrap();
        assert_eq!(actual, res);
    }
    #[test]
    fn case_insensitive_search_1() {
        let actual = vec![
            "1: An apple a day keeps the doctor away.",
            "2: Apple announced a new iPhone today.",
            "3: The company is named Apple, but they sell fruit?",
            "4: I prefer a green apple over a red one.",
        ];
        let mut search_object = SearchInterface::new(
            Some(true),
            b"1: An apple a day keeps the doctor away.
2: Apple announced a new iPhone today.
3: The company is named Apple, but they sell fruit?
4: I prefer a green apple over a red one."
                .as_slice(),
        );
        let res = search_object.search("Apple").unwrap();
        assert_eq!(actual, res);
    }
    #[test]
    fn case_insensitive_search_2() {
        let actual = vec![
            "1: help me!",
            "2: I need some HELP over here.",
            "3: He is very helpful.",
            "4: Can anyone HElp?",
        ];
        let mut search_object = SearchInterface::new(
            Some(true),
            b"1: help me!
2: I need some HELP over here.
3: He is very helpful.
4: Can anyone HElp?"
                .as_slice(),
        );
        let res = search_object.search("help").unwrap();
        assert_eq!(actual, res);
    }
    #[test]
    fn env_var_test_1() {
        let actual = vec![
            "1: help me!",
            "2: I need some HELP over here.",
            "3: He is very helpful.",
            "4: Can anyone HElp?",
        ];
        unsafe {
            std::env::set_var("IGNORE_CASE", "1");
        }
        let mut search_object = SearchInterface::new(
            None,
            b"1: help me!
2: I need some HELP over here.
3: He is very helpful.
4: Can anyone HElp?"
                .as_slice(),
        );
        let res = search_object.search("help").unwrap();
        assert_eq!(actual, res);
    }
    #[test]
    fn env_var_test_2() {
        let actual = vec!["1: help me!", "3: He is very helpful."];
        unsafe {
            std::env::set_var("IGNORE_CASE", "0");
        }
        let mut search_object = SearchInterface::new(
            None,
            b"1: help me!
2: I need some HELP over here.
3: He is very helpful.
4: Can anyone HElp?"
                .as_slice(),
        );
        let res = search_object.search("help").unwrap();
        assert_eq!(actual, res);
    }
}
