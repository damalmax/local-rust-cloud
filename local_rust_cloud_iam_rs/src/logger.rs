use std::io::Write;

use chrono::Local;
use env_logger::Builder;
use log::LevelFilter;

pub fn init_with_level(level: LevelFilter) {
    Builder::new()
        .format(|buf, record| writeln!(buf, "{} [{}] - {}", Local::now().format("%Y-%m-%dT%H:%M:%S"), record.level(), record.args()))
        .filter_level(level)
        // .filter(Option::Some("local_rust_cloud_common"), LevelFilter::Debug)
        // .filter(Option::Some("local_rust_cloud_sts_rs"), LevelFilter::Debug)
        .init();
}
