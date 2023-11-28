use std::{error::Error, ffi::CStr};

use nix::{
    unistd::{fork, ForkResult, execvp},
    sys::{
        ptrace,
        wait::{WaitStatus, waitpid},
        signal::{Signal, raise}
    }, libc::user_regs_struct,
};

fn main() -> Result<(), Box<dyn Error>> {
    let fork_result = unsafe {fork()}?;
    match fork_result {
        ForkResult::Child => {
            ptrace::traceme()?;
            raise(Signal::SIGSTOP)?; // wait for traceme and signal parent about ready

            let app = CStr::from_bytes_with_nul(b"ls\0")?;
            let arg = CStr::from_bytes_with_nul(b"-l\0")?;
            let app_args: Vec<&CStr> = vec![app, arg];
            execvp(app, &app_args)?;
        },
        ForkResult::Parent { child } => {
            waitpid(child, None)?;
            ptrace::setoptions(
                child,
                ptrace::Options::PTRACE_O_TRACESYSGOOD
            )?;
            
            loop {
                ptrace::syscall(child, None)?;
                let status = waitpid(child, None)?;
                match status {
                    WaitStatus::PtraceSyscall(_) => {
                        let regs = ptrace::getregs(child)?;
                        if regs.orig_rax == 12 { // Get BRK syscall
                            print_regs(&regs);
                        }
                    }
                    WaitStatus::Stopped(_, sig) => {
                        println!("Stopped: {:?}", sig);
                    },
                    WaitStatus::Exited(_, exit_code) => {
                        println!("Exit code = {exit_code}");
                        break; // Exit the loop when the child process exits
                    }
                    status => println!("Child process did not exit successfully: {:?}", status),
                }
            }
        }
    }
    Ok(())
}

fn print_regs(regs: &user_regs_struct) {
    println!("SYSCALL BRK");
    let orig_rax = regs.orig_rax;
    let rax = regs.rax;
    let rbx = regs.rbx; // allocated size = current break + new_size
    let rdi = regs.rdi; // initial syscall argument will not be stored in rbx, but rather in rdi
    println!("\tprig_rax = {}", orig_rax);
    println!("\trax = {:#x}", rax);
    println!("\trbx = {:#x}", rbx);
    println!("\trdi = {:#x}", rdi);
}