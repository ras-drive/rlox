pub struct SyntaxError;

impl SyntaxError {
    pub fn report(line: usize, message: String, line_content: String) {
        let error = SyntaxError::error(line, message, line_content);
        println!("{error}");
    }

    fn error(line: usize, message: String, line_content: String) -> String {
        let mut str = String::new();
        str.push_str(format!("Error: {message}\n").as_str()); // ("Error: {message}");
        str.push_str(format!("\t{line} | {line_content}").as_str());

        str
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_error() {
        let line = 1;
        let message = String::from("test");
        let content = String::from("test source code");
        let test_error = SyntaxError::error(line, message.clone(), content.clone());
        assert_eq!(
            test_error,
            String::from(format!("Error: {message}\n\t{line} | {content}"))
        )
    }
}
