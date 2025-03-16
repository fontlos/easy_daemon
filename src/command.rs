use clap::Parser;

#[derive(Parser)]
pub enum Command {
    /// 添加一个新的守护进程, 至少需要唯一的名字, 可执行程序路径
    Add {
        #[clap(short, long)]
        name: String,
        #[clap(short, long)]
        program: String,
        #[clap(short, long)]
        args: Option<Vec<String>>,
        #[clap(short, long)]
        output: Option<String>,
    },
    /// 使用名字启动一个守护进程
    Run {
        #[clap(value_parser)]
        name: String,
    },
    // 这将尝试启动所有的守护进程
    // Start,
    /// 使用名字停止一个守护进程
    Stop {
        #[clap(value_parser)]
        name: String,
    },
    /// 使用名字删除一个守护进程
    Delete {
        #[clap(value_parser)]
        name: String,
    },
    List {
        /// 显示所有守护进程, 包括已经停止的
        #[clap(short, long)]
        all: bool,
    },
}
