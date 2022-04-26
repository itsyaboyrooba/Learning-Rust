use std::process::Command;

fn os_commands_example_1() {

    // OS check
    let echo_cmd = if cfg!(target_os = "windows") {
        Command::new("cmd")
                    .args(["/C", "echo aye there from Windows!"])
                    .output()
                    .expect("Failed to execute command.")
    } else {
        Command::new("sh")
                    .args(["-c", "echo aye there from Linux!"])
                    .output()
                    .expect("Failed to execute command.")
    };
    // Spacing
    println!("\n\n");

    // Output
    let cmd_output = String::from_utf8(echo_cmd.stdout).expect("Could not parse byte response.");
    println!("{}", cmd_output);

    println!("\n\n");
}

fn os_commands_example_2() {

    println!("\n\n");

    let mut cmd_root = Command::new("ls");

    cmd_root.status().expect("Failed to execute list command.");

    println!("\n\n");

    // Goes into ./src folder and executes "ls"
    cmd_root.current_dir("src").status().expect("Failed to execute list command.");
}

fn main() {
    //os_commands_example_1();
    os_commands_example_2();
}
