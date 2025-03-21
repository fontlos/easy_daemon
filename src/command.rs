use clap::Parser;

#[derive(Parser)]
pub enum Command {
    /// 添加一个新的守护进程, 至少需要唯一的名字, 可执行程序路径
    Add {
        #[clap(short, long)]
        name: String,
        #[clap(short, long)]
        program: String,
        #[clap(short, long, allow_hyphen_values = true)]
        args: Option<Vec<String>>,
        /// 输出日志文件路径, 默认为 /dev/null, 设置为空字符串 "" 则不重定向日志
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
