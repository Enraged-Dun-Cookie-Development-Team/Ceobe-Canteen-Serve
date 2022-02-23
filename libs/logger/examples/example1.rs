


fn main(){
    logger::init_std(Default::default()).expect("start logger Error");

    log::info!("aabb");
    log::debug!("Ok {}",11);
    println!("ccdd");
}