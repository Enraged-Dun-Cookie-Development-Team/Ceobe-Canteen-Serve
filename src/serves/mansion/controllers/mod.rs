pub mod mansion;

crate::generate_controller!(MansionController, "/mansion", mansion::get_mansion);
