use clap::{Parser, Subcommand};
use convert_case::{Case, Casing};

#[derive(Parser)]
#[command(
    name = "hx-utils",
    about = "A small collection of utilities for use with Helix",
    version
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Convert text to specified case
    #[command(name = "c")]
    ConvertCase {
        /// Case type to transform
        case_type: String,
    },
}

#[derive(Debug)]
enum CaseType {
    Pascal,
    Camel,
    Snake,
    ScreamingSnake,
    Kebab,
}

impl CaseType {
    fn from_str(s: &str) -> Result<Self, String> {
        match s.to_lowercase().as_str() {
            "p" | "pascal" => Ok(CaseType::Pascal),
            "c" | "camel" => Ok(CaseType::Camel),
            "s" | "snake" => Ok(CaseType::Snake),
            "ss" | "screaming-snake" => Ok(CaseType::ScreamingSnake),
            "k" | "kebab" => Ok(CaseType::Kebab),
            _ => Err(format!("Unknown case type: {}", s)),
        }
    }

    fn transform(&self, text: &str) -> String {
        match self {
            CaseType::Pascal => text.to_case(Case::Pascal),
            CaseType::Camel => text.to_case(Case::Camel),
            CaseType::Snake => text.to_case(Case::Snake),
            CaseType::ScreamingSnake => text.to_case(Case::ScreamingSnake),
            CaseType::Kebab => text.to_case(Case::Kebab),
        }
    }
}

fn transform_line(case_type: &CaseType, input: &str) -> String {
    let mut result = String::new();
    let mut current_word = String::new();

    for c in input.chars() {
        if c.is_whitespace() {
            if !current_word.is_empty() {
                result.push_str(&case_type.transform(&current_word));
                current_word.clear();
            }
            result.push(c);
        } else {
            current_word.push(c);
        }
    }

    // Handle the last word if it exists
    if !current_word.is_empty() {
        result.push_str(&case_type.transform(&current_word));
    }

    result
}

fn run(args: &[String]) -> Result<String, String> {
    let cli = Cli::parse_from(args);
    match cli.command {
        Commands::ConvertCase { case_type } => {
            let case_type = CaseType::from_str(&case_type)?;
            let mut input = String::new();
            std::io::Read::read_to_string(&mut std::io::stdin(), &mut input)
                .map_err(|e| format!("Failed to read input: {}", e))?;
            Ok(transform_line(&case_type, &input))
        }
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    match run(&args) {
        Ok(output) => println!("{}", output),
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_type_from_str() {
        assert!(matches!(CaseType::from_str("p"), Ok(CaseType::Pascal)));
        assert!(matches!(CaseType::from_str("pascal"), Ok(CaseType::Pascal)));
        assert!(matches!(CaseType::from_str("c"), Ok(CaseType::Camel)));
        assert!(matches!(CaseType::from_str("camel"), Ok(CaseType::Camel)));
        assert!(matches!(CaseType::from_str("s"), Ok(CaseType::Snake)));
        assert!(matches!(CaseType::from_str("snake"), Ok(CaseType::Snake)));
        assert!(matches!(
            CaseType::from_str("ss"),
            Ok(CaseType::ScreamingSnake)
        ));
        assert!(matches!(CaseType::from_str("k"), Ok(CaseType::Kebab)));
        assert!(matches!(CaseType::from_str("kebab"), Ok(CaseType::Kebab)));
        assert!(CaseType::from_str("invalid").is_err());
    }

    #[test]
    fn test_transform() {
        let test_str = "hello world";
        assert_eq!(CaseType::Pascal.transform(test_str), "HelloWorld");
        assert_eq!(CaseType::Camel.transform(test_str), "helloWorld");
        assert_eq!(CaseType::Snake.transform(test_str), "hello_world");
        assert_eq!(CaseType::ScreamingSnake.transform(test_str), "HELLO_WORLD");
        assert_eq!(CaseType::Kebab.transform(test_str), "hello-world");
    }

    #[test]
    fn test_transform_line() {
        let complex_input = "hello   world\tsnake_case_here\nPascalCaseWord\tcamelCaseTest   SCREAMING_SNAKE_CASE\nkebab-case-test";

        assert_eq!(
            transform_line(&CaseType::Pascal, complex_input),
            "Hello   World\tSnakeCaseHere\nPascalCaseWord\tCamelCaseTest   ScreamingSnakeCase\nKebabCaseTest"
        );

        assert_eq!(
            transform_line(&CaseType::Camel, complex_input),
            "hello   world\tsnakeCaseHere\npascalCaseWord\tcamelCaseTest   screamingSnakeCase\nkebabCaseTest"
        );

        assert_eq!(
            transform_line(&CaseType::Snake, complex_input),
            "hello   world\tsnake_case_here\npascal_case_word\tcamel_case_test   screaming_snake_case\nkebab_case_test"
        );

        assert_eq!(
            transform_line(&CaseType::ScreamingSnake, complex_input),
            "HELLO   WORLD\tSNAKE_CASE_HERE\nPASCAL_CASE_WORD\tCAMEL_CASE_TEST   SCREAMING_SNAKE_CASE\nKEBAB_CASE_TEST"
        );

        assert_eq!(
            transform_line(&CaseType::Kebab, complex_input),
            "hello   world\tsnake-case-here\npascal-case-word\tcamel-case-test   screaming-snake-case\nkebab-case-test"
        );
    }
}
