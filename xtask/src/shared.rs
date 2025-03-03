pub trait StringExtensions {
    fn camel_to_snake_case(&self) -> String;

    fn snake_to_title_case(&self) -> String;
}

impl StringExtensions for String {
    fn camel_to_snake_case(&self) -> String {
        self.chars()
            .flat_map(|char| {
                if char.is_uppercase() {
                    vec!['_', char.to_ascii_lowercase()]
                } else {
                    vec![char]
                }
            })
            .collect::<String>()
            .trim_start_matches('_')
            .to_string()
    }

    fn snake_to_title_case(&self) -> String {
        self.split('_')
            .map(|word| {
                let mut chars = word.chars();

                match chars.next() {
                    Some(first_char) => {
                        first_char.to_uppercase().collect::<String>() + chars.as_str()
                    }
                    None => String::new(),
                }
            })
            .collect::<Vec<_>>()
            .join(" ")
    }
}
