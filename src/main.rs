mod command;
mod config;
mod execution;

use clap::Parser;

use command::Command;
use config::Config;

fn main() {
    let mut config = Config::load();

    let command = Command::parse();

    match command {
        Command::Add {
            name,
            program: exe,
            args,
            output: std_out,
        } => {
            config.add(name, exe, args, std_out);
        }
        Command::Run { name } => {
            if let Some(daemon) = config.daemons.get_mut(&name) {
                let pid = match execution::start(&daemon.exe, &daemon.args, &daemon.output) {
                    Ok(pid) => pid,
                    Err(e) => {
                        println!("[Error]: Failed to start [{}]: {}", name, e);
                        return;
                    }
                };
                println!("[Info]: Daemon [{}] started with pid {}", name, pid);
                daemon.pid = pid;
            } else {
                println!("[Error]: Daemon [{}] not found", name);
            }
        }
        Command::Stop { name } => {
            if let Some(daemon) = config.daemons.get_mut(&name) {
                let pid = daemon.pid;
                // 判断名字和pid是否匹配
                if pid != 0 {
                    if let Ok(()) = execution::stop(pid, &daemon.exe) {
                        daemon.pid = 0;
                    }
                } else {
                    println!("[Error]: Daemon [{}] not running", name);
                }
            } else {
                println!("[Error]: Daemon [{}] not found", name);
            }
        }
        Command::Delete { name } => {
            config.daemons.remove(&name);
        }
        Command::List { all } => {
            for (name, daemon) in &config.daemons {
                if all || daemon.pid != 0 {
                    println!("[Info]: Daemon [{}] PID: {}", name, daemon.pid);
                }
            }
        }
    }
    config.save();
}
