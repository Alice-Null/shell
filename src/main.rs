#[cfg(target_os = "windows")]
compile_error!{"get a real operating system please\n(real means unix like) (if you're seeing this after a full release that's a bug)"}

// making a parser first
mod parser;
mod types;
mod commands;
mod types_discriminant;

fn main () {
    todo!();
}
