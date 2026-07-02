// for features, see the "features" documentation

// special characters
// see https://discord.com/channels/1429178497875841034/1429178499473867017/1519622882454540298 
// for advice

use std::{collections::btree_map::Values, convert::Infallible, fmt::Error, num::ParseIntError, path::Path, process, str::FromStr};

struct Path (Buf: std::path::PathBuf)
enum Primitive {
    Str(String),
    Int(usize),
    Signed(isize),
    FilePath(Path)
}

impl ToString for Primitive {
    fn to_string (&self) -> String {
        use Primitive::*;
        match self {
            Str(string) => string.to_string(),
            Int(number) => number.to_string(),
            Signed(signed_num) => signed_num.to_string(),
            FilePath(path) => path.to_string()
        }
    }
}

impl FromStr for Primitive {
    type Err = Infallible; // falls back to storing directly

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use Primitive::*;

        match s {
            _ if s.parse::<isize>().is_ok() => {
                // parses as isize
                Ok(Signed(s.parse::<usize>().unwrap()))
            _ if s.parse::<isize>().is_ok() => {
                // parses as isize
                Ok(Signed(s.parse::<isize>().unwrap()))
            _ if s.parse::<usize>().is_ok() => {
                // parses as usize
                Ok(Int(s.parse::<usize>().unwrap()))
            }
            _  => Ok(Str(s.to_string()))
        }
    }
}

impl TryInto<Arg> for Primitive {
    type Error = Error;
    fn try_into(self) -> Result<Arg, Error> {
        todo!()
    }
}

struct  Arg {
    arguments: Vec<String>
}

// enum Argument <'a> {
//     String(), 
//     Integer(),
//     Signed(),
//     Array {  
//         kind: &'a Argument<'a>,
//         length: usize
//     },
//     AdditiveSet {
//         // specific values take precedent over types
//         // 
//         accepted_types: Vec<&'a Value<'a>>,
//         accepted_vals: Vec<&'a Value<'a>>,  
//         allowed: Vec<&'a Value<'a>>
//     }
//     // Regex()
// }

struct Variable <'a> {
    name: &'a str,
    mutable: bool,
    strict_type: bool,
    val: Primitive
}

impl Variable <'_> {
    const fn valid_name(name: &str) -> bool {
        // a valid name must be [A-z], [0-9], and may include [_]
        // it must not start with [0-9]
        // ascii only

        let bytes = name.as_bytes();
        // given this is a `const` function
        // iterating over a &str must be done via as_bytes

        // variables must not start with [0-9]
        let first = bytes[0] as char;
        // check if the first character is a number
        if first.is_ascii_digit() {
            return false; // starts with a number, invalid
        }

        // this is a glorified for loop
        // for loops aren't allowed in const
        // it iterates through every character
        // (bytes used to be an array, but treated as character)
        let mut i = 0;
        while i < bytes.len() {
            let character = bytes[i] as char;
            let valid = matches!(
            character, 
                  'a'..='z'
                | 'A'..='Z'
                | '_'
                | '0'..='9'
            );
            // any invalid character invalidates the entire name
            if !valid {
                return false;
            } else {
                // continue the loop if valid
                i += 1
            }; 
        }   
        true // nothing was invalid
    }
    const fn infer_type(value: &str) -> Primitive {
        value.parse::<Primitive>();
    }
}

struct Env <'session> {
    // session lifetime is until user exits shell
    // basically just until program terminates
    // but multiple sessions may be active at once
    // so it is possible to do so discreetly
    variables: Vec<&'session Variable<'session>>,
}



struct Special {}
// must not be searched
// has to be parsed
// gods ima have to make a "am parsing" struct aren't I

impl Special {
    // all Special characters seperate commands

    // just delimits
        const BASIC: [&str; 2] = /* one of*/ [";","\n"];
    
    // sends output as input to another command
        const PIPE_STD: [&str; 1] = ["|"];
        const PIPE_ERR: [&str; 1] = ["|e"];
        const PIPE_BOTH: [&str; 1] = ["|&"];
    // runs in the background
        const BACKGROUND: [&str; 2] = ["&b", "&b;"];
        // but doesn't give output
        const QUIET_BACKGROUND:  [&str; 2] = ["&q", "&q;"];
        // gives fd for stdout pipe
        const BACKGROUND_STD: [&str; 1] = ["b|"];
}

struct Output {
    // for storing stdout & stderr later

}

trait Command {
    fn run(&self) -> Result<(), Error>;
    fn layout_length(&self) -> Option<usize>;
    fn run_bg(&self) -> Result<() , Error> {
        let process = todo!();
    } // return an stdout & stderr pipe handle later
}

trait BuiltInCommand: Command {
    const ARG_LAYOUT: Layout;
    const NAME: &'static str;
    fn run() -> Result<(), Error>;

}


struct ExternalCommand <'a> {
    name: &'a str,
    layout: Option<Layout>,
    arguments: Vec<Primitive>
}

impl Command for ExternalCommand <'_>{
    fn run(&self) -> Result<(), Error> {
        let process = process::Command::new(self.name);
        let args: Vec<String> = {
            let layout_length = self.layout_length()
            let mut smaller_scope_args = Vec::<String>::with_capacity();
            self.arguments.iter().for_each(
                |var|
                var.spread()
            )   
        };
    }
    fn layout_length(&self) -> Option<usize> {
        match self.layout {
            Some(layout) => Some(layout.len()),
            None => None(),
        }
    }
}

// impl BuiltInCommand {
//     const fn is_builtin (command: &str) -> Result<BuiltInCommand> {
//         match BuiltInCommand
//         ::COMMANDS
//         .iter()
//         .find(|builtin| builtin == command ) {
//             Some(command) => {
//                 todo!(); // need to figure out how to call the relevant function
//                 // not really sure how to do that
//                 // match statement?
//                 match command {
                    
//                 }
//             },
//             None => Err("not built in"),
//         }
//     }
    
//     fn cd {

//     }
// }

struct Layout (Vec<Value>)
impl Layout {
    fn len(&self)-> &usize  {
        &self.args.len()
    }
}

struct Line <'a>{
    command: Option<>
    // basically just a str with impls and a name 

}
mod tests {
    // add tests probably
}


// #[cfg(test)]
// mod tests {
//     todo!();
// }