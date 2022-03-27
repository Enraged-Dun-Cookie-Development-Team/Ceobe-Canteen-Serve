use super::modules::{loading_model, MansionDb};

pub mod mansion;

crate::generate_controller!(
    MansionController,
    "/mansion",
    mansion::save_mansion,
    mansion::get_mansion,
    mansion::get_all_id,
    mansion::remove_mansion
);

crate::extra_module!(
    MansionController=>crate::generate_collection_register!(
        MansionDb=>loading_model
    )
);
