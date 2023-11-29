use std::{error::Error, ffi::CStr};

use nix::{unistd::execvp, sys::{ptrace, signal::{Signal, raise}}};

pub fn children_proc(mut app: String, mut args: Vec<String>) -> Result<(), Box<dyn Error>> {
    ptrace::traceme()?;
    raise(Signal::SIGSTOP)?; // wait for traceme and signal parent about ready

    app.push('\0');
    let app_c_str = CStr::from_bytes_with_nul(app.as_bytes())?;

    let mut args_c_str: Vec<&CStr> = Vec::new();
    for arg in args.iter_mut() {
        arg.push('\0');
        let arg_c_str = CStr::from_bytes_with_nul(arg.as_bytes())?;
        args_c_str.push(arg_c_str);
    };

    execvp(&app_c_str, &args_c_str)?;
    Ok(())
}