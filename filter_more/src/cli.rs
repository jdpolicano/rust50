use std::env;

pub struct CommandOptions {
    pub flag: CliFlag,
    pub source: String,
    pub output: String
}

pub enum CliFlag {
    Blur,
    Edges,
    GreyScale,
    Reflect,
}


impl CommandOptions {
    pub fn new() -> Self {
        Self {
            flag: CliFlag::Blur, // default to blur the image
            source: String::new(),
            output: String::new()
        }
    }

    pub fn build() -> Result<Self, String> {
        let err_message = String::from("Usage: ./filter [flag] infile outfile");
        let mut args = env::args();
        let mut default = CommandOptions::new();

        args.next(); // skip executable info

        default.flag = if let Some(f) = args.next() {
            match &f[1..] {
                "b" => CliFlag::Blur,
                "e" => CliFlag::Edges,
                "g" => CliFlag::GreyScale,
                "r" => CliFlag::Reflect,
                _ => {
                    return Err(format!("Recieved invalid flag: {}", f));
                }
            }
        } else {
            return Err(err_message)
        };

        if let Some(i_file) = args.next() {
            default.source.push_str(&i_file);
        } else {
            return Err(err_message)
        };

        if let Some(o_file) = args.next() {
           default.output.push_str(&o_file);
        } else {
            return Err(err_message)
        };


        Ok(default)
    }
}

