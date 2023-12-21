use std::process::Command;
use std::io::{Error, ErrorKind};

// We can add a directory into the $dir variable.
fn error_handling_example(dir: &str) {

    println!("\n\n");

    let mut list_cmd = Command::new("ls");

        // We can add a directory($dir) after "ls" command.
        match list_cmd.current_dir(dir).status() {

            // Ok lets the command to continue
            Ok(cmd) => Some(cmd),

            // If error:
            Err(e) => match e.kind() {

                // If error is like/kind a missing directory, print this:
                ErrorKind::NotFound => {
                    println!("Directory not found!");
                    None
                },

                // For any other error kind, PANIC!
                _ => panic!("An unexpected error has occured!")
            },
    };
    println!("\n\n");
}

fn error_handling_example2(dir: &str) -> Result<f64, Error> {

    println!("\n\n");

    let mut list_cmd = Command::new("ls");

    // If you use "?" in status, you need output. We define "-> Result<f64, Error>" in function and later return "Ok(1)"
    list_cmd.current_dir(dir).status()?;

    println!("\n\n");

    // Return an f64
    Ok(1)
}

fn main() {

    /*
    error_handling_example("src");

    // "/lib doesn't exist"
    error_handling_example("lib");
    */

    error_handling_example2("src");

}