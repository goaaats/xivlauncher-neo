use std::process::Command;
use sysinfo::{Pid, ProcessExt, System, SystemExt};

mod tests;

lazy_static! {
    static ref SYS: System = System::new_all();
}

/// Runs the command with the provided arguments.
pub fn run_command(command: &str, args: &[&str]) {
    let mut p = Command::new(command);
    p.args(args);
    p.output().expect("failed to execute process");
}

/// Runs the command as an administrator, with the provided arguments.
pub fn run_command_elevated(command: &str, args: &[&str]) {
    let real_command = "runas";
    let mut real_args = vec![command];
    real_args.extend(args);

    let mut p = Command::new(real_command);
    p.args(real_args);
    p.output().expect("failed to execute process");
}

/// Closes the specified process.
pub fn close_process(process_name: &str) {
    let mut process_id: Pid = 0;
    for (pid, process) in SYS.processes() {
        if process.name() == process_name {
            process_id = pid.to_owned();
        }
    }

    if process_id == Default::default() {
        return;
    }

    // TODO: actually close the process; how this is done is platform-dependent.
}

/// Returns true if the specified process is running.
pub fn is_process_running(process_name: &str) -> bool {
    for (_, process) in SYS.processes() {
        if process.name() == process_name {
            return true;
        }
    }

    false
}