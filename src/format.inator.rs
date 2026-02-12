use serde::Deserialize;

#[derive(Debug, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum InatorCase {
    #[default]
    Default,
    Snake,
    Camel,
    Kebab,
    NoSpaces,
    Lower,
    Upper,
}

#[derive(Debug, Deserialize, Default)]
pub struct FormatQuery {
    pub case: Option<InatorCase>,
    #[serde(default)]
    pub strip_special: bool,
}

fn strip_special_chars(name: &str) -> String {
    name.chars()
        .filter(|ch| ch.is_alphanumeric() || ch.is_whitespace() || *ch == '-')
        .collect()
}

/// Apply the requested formatting to an inator name.
/// Replaces spaces and hyphens according to the format,
/// preserving the original name as much as possible.
pub fn apply_format(name: &str, query: &FormatQuery) -> String {
    let name = if query.strip_special {
        strip_special_chars(name)
    } else {
        name.to_string()
    };
    format_inator(&name, query.case.as_ref().unwrap_or(&InatorCase::Default))
}

fn format_inator(name: &str, format: &InatorCase) -> String {
    match format {
        InatorCase::Default => name.to_string(),
        InatorCase::Snake => name.replace([' ', '-'], "_"),
        InatorCase::Camel => {
            let mut result = String::with_capacity(name.len());
            let mut capitalize_next = false;
            for ch in name.chars() {
                if ch == ' ' || ch == '-' {
                    capitalize_next = true;
                } else if capitalize_next {
                    result.extend(ch.to_uppercase());
                    capitalize_next = false;
                } else {
                    result.push(ch);
                }
            }
            result
        }
        InatorCase::Kebab => name.replace(' ', "-"),
        InatorCase::NoSpaces => name.replace([' ', '-'], ""),
        InatorCase::Lower => name.to_lowercase(),
        InatorCase::Upper => name.to_uppercase(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_preserves_name() {
        assert_eq!(format_inator("Shrink-inator", &InatorCase::Default), "Shrink-inator");
    }

    #[test]
    fn test_snake_case() {
        assert_eq!(format_inator("Shrink-inator", &InatorCase::Snake), "Shrink_inator");
        assert_eq!(format_inator("De Love-inator", &InatorCase::Snake), "De_Love_inator");
    }

    #[test]
    fn test_camel_case() {
        assert_eq!(format_inator("Shrink-inator", &InatorCase::Camel), "ShrinkInator");
        assert_eq!(format_inator("De Love-inator", &InatorCase::Camel), "DeLoveInator");
    }

    #[test]
    fn test_kebab_case() {
        assert_eq!(format_inator("Shrink-inator", &InatorCase::Kebab), "Shrink-inator");
        assert_eq!(format_inator("De Love-inator", &InatorCase::Kebab), "De-Love-inator");
    }

    #[test]
    fn test_no_spaces() {
        assert_eq!(format_inator("Shrink-inator", &InatorCase::NoSpaces), "Shrinkinator");
        assert_eq!(format_inator("De Love-inator", &InatorCase::NoSpaces), "DeLoveinator");
    }

    #[test]
    fn test_lower() {
        assert_eq!(format_inator("Shrink-inator", &InatorCase::Lower), "shrink-inator");
        assert_eq!(format_inator("De Love-inator", &InatorCase::Lower), "de love-inator");
    }

    #[test]
    fn test_upper() {
        assert_eq!(format_inator("Shrink-inator", &InatorCase::Upper), "SHRINK-INATOR");
        assert_eq!(format_inator("De Love-inator", &InatorCase::Upper), "DE LOVE-INATOR");
    }

    #[test]
    fn test_strip_special_alone() {
        let query = FormatQuery { case: None, strip_special: true };
        assert_eq!(apply_format("Smell (good)-inator", &query), "Smell good-inator");
        assert_eq!(apply_format("What's-this?-inator", &query), "Whats-this-inator");
    }

    #[test]
    fn test_strip_special_with_format() {
        let query = FormatQuery { case: Some(InatorCase::Snake), strip_special: true };
        assert_eq!(apply_format("Smell (good)-inator", &query), "Smell_good_inator");

        let query = FormatQuery { case: Some(InatorCase::Camel), strip_special: true };
        assert_eq!(apply_format("Smell (good)-inator", &query), "SmellGoodInator");
    }
}
