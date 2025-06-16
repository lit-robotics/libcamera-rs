use libcamera::{camera_manager::CameraManager, controls::ControlId, logging::LoggingLevel};

fn main() {
    let mgr = CameraManager::new().unwrap();

    mgr.log_set_level("Camera", LoggingLevel::Error);

    let cameras = mgr.cameras();

    for cam in cameras.iter() {
        println!("ID: {}", cam.id());

        for (id, _) in cam.controls() {
            let id = ControlId::from_id(id).unwrap();
            println!("{:#?}", id.name());
            println!("  Vendor: {:#?}", id.vendor());
            println!("  Control Type: {:#?}", id.control_type());
            println!("  Direction: {:#?}", id.direction());
            println!("  Is Array: {:#?}", id.is_array());
            println!("  Is Input: {:#?}", id.is_input());
            println!("  Is Output: {:#?}", id.is_output());
            println!("  Enumarators: {:?}", id.enumerators_map());
        }
    }
}
