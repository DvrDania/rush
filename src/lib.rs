use regex::Regex;

pub fn validate(args: &Vec<String>) -> Result<(), String> {
    if let 1 = args.len() {
        return Err("Not enough arguments".to_string());
    }

    let hex_color = Regex::new(r"^#(?:[0-9a-fA-F]{3}){1,2}$").unwrap();

    let color = &args[1];
    if let false = hex_color.is_match(color) {
        return Err("Invalid color format".to_string());
    };

    Ok(())
}

pub fn start(args: &Vec<String>) {
    match validate(args) {
        Ok(_) => {
            // Continue if arguments are valid
            todo!()
        }
        Err(s) => println!("{s}"),
    }
}

#[cfg(test)]
mod tests {
    use regex::Regex;

    #[test]
    fn valid_ffffff() {
        let hex_color = Regex::new(r"^#(?:[0-9a-fA-F]{3}){1,2}$").unwrap();
        assert_eq!(hex_color.is_match("#ffffff"), true);
    }
    #[test]
    fn valid_000000() {
        let hex_color = Regex::new(r"^#(?:[0-9a-fA-F]{3}){1,2}$").unwrap();
        assert_eq!(hex_color.is_match("#000000"), true);
    }
    #[test]
    fn invalid_a02f1g() {
        let hex_color = Regex::new(r"^#(?:[0-9a-fA-F]{3}){1,2}$").unwrap();
        assert_eq!(hex_color.is_match("#a02f1g"), false);
    }
    #[test]
    fn invalid_fffff() {
        let hex_color = Regex::new(r"^#(?:[0-9a-fA-F]{3}){1,2}$").unwrap();
        assert_eq!(hex_color.is_match("#fffff"), false);
    }
}
