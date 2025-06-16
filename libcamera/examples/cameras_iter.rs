use libcamera::{camera_manager::CameraManager, logging::LoggingLevel, stream::StreamRole};

fn main() {
    let mgr = CameraManager::new().unwrap();

    mgr.log_set_level("Camera", LoggingLevel::Error);

    let cameras = mgr.cameras();

    for cam in cameras.iter() {
        println!("ID: {}", cam.id());

        println!("Properties: {:#?}", cam.properties());
        println!("Controls: {:#?}", cam.controls());

        let config = cam.generate_configuration(&[StreamRole::ViewFinder]).unwrap();
        let view_finder_cfg = config.get(0).unwrap();
        println!("Available formats: {:#?}", view_finder_cfg.formats());
    }
}
