use std::process::{Command, Stdio};

fn main() {
   let mut input = Command::new("");
   let stdout = input.stdout(Stdio::piped()).output().unwrap();
   let stdout_str = String::from_utf8(stdout.stdout).unwrap();

   println!("{}", stdout_str);
}
