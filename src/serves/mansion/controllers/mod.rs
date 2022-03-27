use super::modules::{MansionDb, loading_model};

pub mod mansion;

crate::generate_controller!(MansionController, "/mansion" ,mansion::save_mansion);

crate::extra_module!(
    MansionController=>crate::generate_collection_register!(
        MansionDb=>loading_model
    )
);