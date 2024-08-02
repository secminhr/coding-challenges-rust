use std::{env, fmt::Display, fs, process::ExitCode};

fn main() -> ExitCode {
    let args = env::args().nth(1);
    if args == None {
        println!("help: json-parser <FILE>");
        return ExitCode::from(1);
    }
    let filepath = args.unwrap();
    let result = fs::read_to_string(filepath);
    if result.is_err() {
        let error = result.err().unwrap();
        println!("Error: {}", error);
        return ExitCode::from(1);
    }

    match parse(result.unwrap()) {
        Ok(json) => {
            print!("{}", json);
            ExitCode::SUCCESS
        }
        Err(msg) => {
            println!("{}", msg);
            ExitCode::from(1)
        }
    }
}

#[derive(PartialEq, Eq, Debug)]
struct JsonObject {}

impl Display for JsonObject {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("{}")
    }
}

fn parse<S: Into<String>>(into_string: S) -> Result<JsonObject, String> {
    let string: String = into_string.into();
    if string.is_empty() {
        Err("Error: empty string".into())
    } else {
        Ok(JsonObject {})
    }
}

#[cfg(test)]
mod test {

    use crate::{parse, JsonObject};

    #[test]
    fn test_empty_string() {
        let result = parse("");
        assert_eq!(result, Err("Error: empty string".into()));
    }

    #[test]
    fn test_empty_json() {
        let result = parse("{}");
        assert_eq!(result, Ok(JsonObject {}))
    }

    #[test]
    fn test_empty_json_format() {
        let json = JsonObject {};
        assert_eq!(json.to_string(), "{}")
    }
}
