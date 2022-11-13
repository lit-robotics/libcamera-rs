use libcamera_rs::{camera_manager::CameraManager, properties, stream::StreamRole};

fn main() {
    let mgr = CameraManager::new().unwrap();

    let cameras = mgr.cameras();

    let cam = cameras.get(0).expect("No cameras found");

    println!(
        "Using camera: {}",
        *cam.properties().get::<properties::Model>().unwrap()
    );

    let mut cfgs = cam.generate_configuration(&[StreamRole::StillCapture]).unwrap();
    let cfg = cfgs.get_mut(0).unwrap();

    println!("Generated config: {:#?}", cfg);

    if cfgs.validate().is_invalid() {
        panic!("Error validating camera configuration");
    }
}
