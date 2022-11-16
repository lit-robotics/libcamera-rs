use libcamera_rs::{camera_manager::CameraManager, stream::StreamRole};

fn main() {
    let mgr = CameraManager::new().unwrap();

    let cameras = mgr.cameras();

    for i in 0..cameras.len() {
        let cam = cameras.get(i).unwrap();
        println!("Camera {}", i);
        println!("ID: {}", cam.id());

        println!("Properties: {:#?}", cam.properties());

        let config = cam.generate_configuration(&[StreamRole::ViewFinder]).unwrap();
        let view_finder_cfg = config.get(0).unwrap();
        println!("Available formats: {:#?}", view_finder_cfg.formats());
    }
}
