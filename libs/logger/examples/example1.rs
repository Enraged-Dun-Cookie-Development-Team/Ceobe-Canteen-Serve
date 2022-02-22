


fn main(){
    logger::init(log::LevelFilter::Trace).expect("start logger Error");

    log::info!("aabb");
    log::debug!("Ok {}",11);
    println!("ccdd");
}