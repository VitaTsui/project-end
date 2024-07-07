use rbdc_mysql::driver::MysqlDriver;
use tracing::{info, instrument};

use crate::{yaml::CONFIG, RB};

// 初始化数据库链接池
#[instrument]
pub async fn init_db() {
    info!("--------开始连接数据库-------");

    let mysql_config = &CONFIG.mysql;

    let mysql_url = format!(
        "mysql://{}:{}@{}:{}/{}",
        mysql_config.username,
        mysql_config.password,
        mysql_config.host,
        mysql_config.port,
        mysql_config.database
    );

    RB.link(MysqlDriver {}, &mysql_url)
        .await
        .expect("--------数据库连接失败--------");

    info!("--------数据库连接成功-------");
}
