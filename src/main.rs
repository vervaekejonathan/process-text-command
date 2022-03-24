use std::process::Command;
use std::process::Stdio;

fn command(command: &str) {
    // dummy command to create a empty stdout
    let mut cmd = Command::new("echo")
        .stdout(Stdio::piped())
        .spawn()
        .expect("command failed");

    // split the command based on the pipe symbol
    for depiped_command in command.split("|") {
        // split the sub-command based on the whitespaces
        // first element is the actual command,
        // other elements are added as arguments
        let mut w = depiped_command.split_whitespace();
        let i = w.next().unwrap_or_default();
        let mut c = Command::new(i);

        for argument in w {
            c.arg(argument);
        }

        // execute the command, using the previous command's output as input
        // write output to pipe so the next command can read it in
        cmd = c
            .stdin(cmd.stdout.unwrap())
            .stdout(Stdio::piped())
            .spawn()
            .expect("command failed");
    }

    // dummy command to read out the stdout of the last piped command and print it
    let output = Command::new("cat")
        .stdin(cmd.stdout.unwrap())
        .output()
        .expect("command failed");

    println!("{}", String::from_utf8_lossy(&output.stdout));
    println!("{}", String::from_utf8_lossy(&output.stderr));
}

fn main() {
    // supports infinite pipes
    command("ls -al / | grep -i etc | grep root | grep 4096");
}
