use logger::LoggerConfig;

fn main() {
    logger::init_std(LoggerConfig::default().disable_color().enable_color()).expect("start logger Error");

    log::info!("aabb");
    log::debug!("Ok {}", 11);
    log::error!("Bad Error");
    log::trace!("A Trace");
    log::warn!("warn")
}
