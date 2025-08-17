// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use tracing::{error, info};
use warp::Filter;

/// 启动网页服务器
///
/// # 参数
/// - port: 服务器监听端口
pub async fn start_server(port: u16) {
    // 获取用户主目录
    let home_dir = match home::home_dir() {
        Some(path) => path,
        None => {
            error!("无法获取用户主目录");
            panic!("无法获取用户主目录");
        }
    };

    // 构建静态文件目录路径
    let static_dir = home_dir.join(".local/share/pageos");

    // 创建静态文件路由
    let static_files = warp::get()
        .and(warp::fs::dir(static_dir))
        .with(warp::log("pageos::server"));

    // 设置服务器地址
    let addr = ([127, 0, 0, 1], port);

    info!(
        "启动服务器: http://{}.{}.{}.{}:{}",
        addr.0[0], addr.0[1], addr.0[2], addr.0[3], addr.1
    );

    // 启动服务器
    warp::serve(static_files).run(addr).await;
}
