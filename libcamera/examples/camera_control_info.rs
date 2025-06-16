use libcamera::{camera_manager::CameraManager, controls::ControlId, logging::LoggingLevel, stream::StreamRole};

fn main() {
    let mgr = CameraManager::new().unwrap();

    mgr.log_set_level("Camera", LoggingLevel::Error);

    let cameras = mgr.cameras();

    for cam in cameras.iter() {
        println!("ID: {}", cam.id());

        println!("Properties: {:#?}", cam.properties());

        for (id, info) in cam.controls().into_iter() {
            println!("Control Info: {:#?}", info);
            println!("Control ID: {:#?}", ControlId::from_id(id));
        }
    }
}
