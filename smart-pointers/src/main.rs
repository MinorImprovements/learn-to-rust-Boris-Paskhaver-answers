use std::error::Error;
use std::fmt::Display;

trait TextTransformer {
    fn transform(&self, text: &str) -> Result<String, Box<dyn Error>>;
}

#[derive(Debug)]
enum CustomError {
    ContainsPizza,
    TextWasEmpty,
}

impl Display for CustomError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ContainsPizza => write!(f,"Error Message: Something went wrong: Hey, there's a pizza emoji in the text. So cheesy. Moving on to next transform"),
            Self::TextWasEmpty => write!(f,"Error Message: Something went wrong: The string has nothing left in it. Moving on to next transform" ),
        }
    }
}

impl Error for CustomError {}

struct WhitespaceTransformer {
    start: bool,
    end: bool,
}

impl TextTransformer for WhitespaceTransformer {
    fn transform(&self, text: &str) -> Result<String, Box<dyn Error>> {
        if text.contains("🍕") {
            return Err(Box::new(CustomError::ContainsPizza));
        }

        let new_str = match self {
            x if x.start && x.end => text.trim(),
            x if x.start => text.trim_start(),
            x if x.end => text.trim_end(),
            _ => text,
        };

        match new_str {
            "" => {
                return Err(Box::new(CustomError::TextWasEmpty));
            }
            something => Ok(something.to_string()),
        }
    }
}

enum Case {
    Uppercase,
    Lowercase,
}

struct CaseTransformer {
    case: Case,
}

impl TextTransformer for CaseTransformer {
    fn transform(&self, text: &str) -> Result<String, Box<dyn Error>> {
        match self.case {
            Case::Uppercase => Ok(text.to_uppercase()),
            Case::Lowercase => Ok(text.to_lowercase()),
        }
    }
}

fn apply_transformations(text: String, pipeline: Vec<Box<dyn TextTransformer>>) -> String {
    pipeline
        .into_iter()
        .fold(text, |acc, instruction| match instruction.transform(&acc) {
            Ok(output) => output,
            Err(e) => {
                eprintln!("{e}");
                acc
            }
        })
}

fn main() {
    // Input
    let text = String::from("  homer simpson  ");
    // Output
    // Content: "HOMER SIMPSON"

    // Input
    //let text = String::from("  data  🍕  ");
    // Output
    // Error Message: Something went wrong: Hey, there's a pizza emoji in the text. So cheesy. Moving on to next transform
    // Content: "  DATA  🍕  "

    // Input
    let text = String::from("    ");
    // Output:
    // Error Message: Something went wrong: The string has nothing left in it. Moving on to next transform
    // Content: "    "

    let pipeline: Vec<Box<dyn TextTransformer>> = vec![
        Box::new(WhitespaceTransformer {
            start: true,
            end: true,
        }),
        Box::new(CaseTransformer {
            case: Case::Uppercase,
        }),
    ];

    let transformed_text = apply_transformations(text, pipeline);
    println!("Output: {transformed_text}");
}
