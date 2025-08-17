// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::io;
mod server;

use clap::Parser;
use tokio::process::Command;
use tracing::{error, info};

/// 命令行参数
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// 服务监听端口
    #[arg(short, long, default_value_t = 12800)]
    port: u16,

    /// 启动子进程命令
    #[arg(short, long)]
    command: Option<String>,
}

#[tokio::main]
async fn main() {
    // 初始化日志
    tracing_subscriber::fmt::init();

    let args = match Args::try_parse() {
        Ok(args) => args,
        Err(e) => {
            error!("参数解析失败: {}", e);
            std::process::exit(1);
        }
    };
    info!("启动参数: {:?}", args);

    // 启动子进程（如果存在）
    let child = if let Some(cmd) = &args.command {
        info!("启动子进程: {}", cmd);
        Some(
            Command::new("sh")
                .arg("-c")
                .arg(cmd)
                .kill_on_drop(true)
                .spawn()
                .expect("启动子进程失败"),
        )
    } else {
        None
    };

    // 启动服务器
    info!("启动服务器，监听端口: {}", args.port);
    server::start_server(args.port).await;

    // 等待子进程退出（如果存在）
    if let Some(mut child) = child {
        match child.wait().await {
            Ok(status) => info!("子进程已退出，状态码: {}", status),
            Err(e) => error!("等待子进程时出错: {}", e),
        }
    }
}
