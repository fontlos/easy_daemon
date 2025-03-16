use nix::errno::Errno;
use nix::sys::signal::{Signal, kill};
use nix::unistd::{ForkResult, Pid, dup2, execvp, fork};

use std::ffi::CString;
use std::fs::OpenOptions;
use std::os::unix::io::AsRawFd;

pub fn start(exe: &str, args: &[String], output: &str) -> Result<u32, String> {
    let exe = CString::new(exe).map_err(|e| e.to_string())?;
    // 构建完整的参数列表，包括 argv[0]
    let mut args_cstr = vec![CString::new(exe.as_bytes()).map_err(|e| e.to_string())?];
    for arg in args {
        args_cstr.push(CString::new(arg.as_str()).map_err(|e| e.to_string())?);
    }

    match unsafe { fork() } {
        Ok(ForkResult::Parent { child }) => {
            // 父进程直接退出，子进程继续运行
            Ok(child.as_raw() as u32)
        }
        Ok(ForkResult::Child) => {
            // 子进程脱离终端
            if let Err(e) = nix::unistd::setsid() {
                println!("[Error]: Failed to create new session: {}", e);
                std::process::exit(1);
            }

            // 关闭标准输入
            let dev_null = OpenOptions::new()
                .read(true)
                .open("/dev/null")
                .map_err(|e| e.to_string())?;
            dup2(dev_null.as_raw_fd(), 0).map_err(|e| e.to_string())?;

            // 重定向标准输出和标准错误
            if output == "/dev/null" {
                // 如果输出是 /dev/null，直接打开
                let dev_null = OpenOptions::new()
                    .write(true)
                    .open("/dev/null")
                    .map_err(|e| e.to_string())?;
                let log_fd = dev_null.as_raw_fd();
                dup2(log_fd, 1).map_err(|e| e.to_string())?; // 标准输出
                dup2(log_fd, 2).map_err(|e| e.to_string())?; // 标准错误
            } else {
                // 如果输出是普通日志文件，使用 OpenOptions 创建和截断文件
                let log_file = OpenOptions::new()
                    .create(true)
                    .append(true) // 使用 append 模式，避免清空日志文件
                    .open(output)
                    .map_err(|e| e.to_string())?;
                let log_fd = log_file.as_raw_fd();
                dup2(log_fd, 1).map_err(|e| e.to_string())?; // 标准输出
                dup2(log_fd, 2).map_err(|e| e.to_string())?; // 标准错误
            }

            // 执行目标程序
            match execvp(&exe, &args_cstr) {
                Ok(_) => {
                    std::process::exit(0);
                }
                Err(e) => {
                    println!("[Error]: Failed to exec: {}", e);
                    std::process::exit(1);
                }
            }
        }
        Err(_) => Err(String::from("Fork failed")),
    }
}

pub fn stop(pid: u32, check: &str) -> Result<(), ()> {
    // 检查进程是否存在
    let cmdline_path = format!("/proc/{}/cmdline", pid);
    let cmdline = match std::fs::read_to_string(&cmdline_path) {
        Ok(cmdline) => cmdline,
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
            println!("[Error]: PID {} does not exist", pid);
            return Err(());
        }
        Err(e) => {
            println!("[Error]: Failed to read cmdline for PID {}: {}", pid, e);
            return Err(());
        }
    };

    // 检查命令行参数是否匹配预期
    if !cmdline.contains(check) {
        println!("[Error]: PID {} does not match your target", pid);
        return Err(());
    }

    let pid = Pid::from_raw(pid as i32);

    // 发送 SIGTERM 信号，请求进程正常退出
    if let Err(e) = kill(pid, Signal::SIGTERM) {
        println!("[Error]: Failed to send SIGTERM to PID {}: {}", pid, e);
        return Err(());
    }

    println!("[Info]: Sent SIGTERM to PID {}, Waiting for 2s...", pid);

    // 等待 2 秒
    std::thread::sleep(std::time::Duration::from_secs(2));

    // 检查进程是否仍然存在
    if let Err(e) = kill(pid, None) {
        if e == Errno::ESRCH {
            println!("[Info]: PID {} stoped", pid);
            return Ok(());
        }
        println!("[Error]: Failed to check process {}: {}", pid, e);
        return Err(());
    }

    // 如果进程仍然存在，发送 SIGKILL 强制终止
    if let Err(e) = kill(pid, Signal::SIGKILL) {
        println!("[Error]: Failed to send SIGKILL to process {}: {}", pid, e);
        return Err(());
    }

    println!("[Info]: Sent SIGKILL to process {}", pid);
    Ok(())
}
