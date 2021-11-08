#[derive(Eq, PartialEq, Debug)]
pub struct Options {
    pub prompt: String,
    pub suppress_byte_counts: bool,
    pub file_name: Option<String>
}

impl Options {
    pub fn new(args: &[String]) -> Result<Options, &str> {
        let mut options = Options {
            prompt: String::from(""),
            suppress_byte_counts: false,
            file_name: None
        };

        let mut i = 1;
        while i < args.len() {
            let arg = &args[i];
            if arg == "-s" {
                options.suppress_byte_counts = true;
            }
            else if arg == "-p" {
                if i + 1 >= args.len() {
                    return Err("missing prompt");
                }

                let prompt = &args[i + 1];
                options.prompt = prompt.clone();
                i = i + 1;
            }
            else if i == args.len() -1 {
                options.file_name = Some(arg.clone());
            }
            else {
                return Err("unexpected argument");
            }

            // Increment
            i = i + 1;
        }

        return Ok(options);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_options() {
        let args = [String::from("ned")];
        let result = Options::new(&args).unwrap();
        assert_eq!(result, Options {
            prompt: String::from(""),
            suppress_byte_counts: false,
            file_name: None
        });
    }

    #[test]
    fn suppress_errors() {
        let args = [String::from("ned"), String::from("-s")];
        let result = Options::new(&args).unwrap();
        assert_eq!(result, Options {
            prompt: String::from(""),
            suppress_byte_counts: true,
            file_name: None
        });
    }

    #[test]
    fn with_prompt() {
        let args = [String::from("ned"), String::from("-p"), String::from("*")];
        let result = Options::new(&args).unwrap();
        assert_eq!(result, Options {
            prompt: String::from("*"),
            suppress_byte_counts: false,
            file_name: None
        });
    }

    #[test]
    fn file_name() {
        let args = [String::from("ned"), String::from("foo.txt")];
        let result = Options::new(&args).unwrap();
        assert_eq!(result, Options {
            prompt: String::from(""),
            suppress_byte_counts: false,
            file_name: Some(String::from("foo.txt"))
        });
    }

    #[test]
    fn all_three() {
        let args = [String::from("ned"), String::from("-s"), String::from("-p"), String::from("*"), String::from("foo.txt")];
        let result = Options::new(&args).unwrap();
        assert_eq!(result, Options {
            prompt: String::from("*"),
            suppress_byte_counts: true,
            file_name: Some(String::from("foo.txt"))
        });
    }

    #[test]
    fn error_prompt() {
        let args = [String::from("ned"), String::from("-p")];
        let result = Options::new(&args);
        assert_eq!(result, Err("missing prompt"));
    }
}

