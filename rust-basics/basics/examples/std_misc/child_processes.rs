use std::process::{Command, Output, Stdio};
use std::io::prelude::{Write, Read};

fn print_process_output(label: &str, output: &Output) {
    println!("{} :", label);
    if output.status.success() {
        let s = String::from_utf8_lossy(&output.stdout);
        println!("Process succeeded and stdout was:\n{}", s);
    } else {
        let s = String::from_utf8_lossy(&output.stderr);
        println!("Process failed and stderr was:\n{}", s);
    }
}

fn child_processes_examples() {
    let output_1: Output = Command::new("rustc")
        .arg("--version")
        .output().unwrap_or_else(|e| {
            panic!("failed to execute process: {} ", e)
        });

    let output_2: Output = Command::new("rustc")
        .arg("--this-is-invalid-flag")
        .output().unwrap_or_else(|e| {
            panic!("failed to execute process: {} ", e)
        });

    print_process_output("output_1", &output_1);
    print_process_output("output_2", &output_2);
}

static PANGRAM: &'static str = "the quick brown fox jumps over the lazy dog\n";

fn process_pangram() {
    // Spawn the `wc` command
    let mut cmd = if cfg!(target_family = "windows") {
        let mut cmd = Command::new("powershell");
        cmd.arg("-Command").arg("$input | Measure-Object -Line -Word -Character");
        cmd
    } else {
        Command::new("wc")
    };

    let program = if cfg!(target_family = "windows") { "powershell" } else { "wc" };

    let process = match cmd
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn() {
            Err(why) => panic!("couln't spawn {}: {}", program, why),
            Ok(process) => process,
        };
    
    match process.stdin.unwrap().write_all(PANGRAM.as_bytes()) {
        Err(why) => panic!("couldn't write to {} stdin: {}", program, why),
        Ok(_) => println!("sent pangram to {}", program),
    }

    let mut s = String::new();
    match process.stdout.unwrap().read_to_string(&mut s) {
        Err(why) => panic!("couldn't read {} stdout: {}", program, why),
        Ok(_) => print!("{} responded with:\n{}", program, s),
    }
}

fn wait() {
    let mut child = if cfg!(target_family = "windows") {
    Command::new("powershell")
        .arg("-Command")
        .arg("Start-Sleep -Seconds 5")
        .spawn()
        .unwrap()
} else {
    Command::new("sleep")
        .arg("5")
        .spawn()
        .unwrap()
};
    println!("starting wait...");
    let _result = child.wait().unwrap();
    println!("reached end of wait.");
}

pub fn child_processes_demo() {
    // child_processes_examples();
    // process_pangram();
    wait();
}