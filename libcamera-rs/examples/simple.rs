use libcamera_rs::{camera_manager::CameraManager, properties, stream::StreamRole};

fn main() {
    let mgr = CameraManager::new().unwrap();

    let cameras = mgr.cameras();

    let cam = cameras.get(0).expect("No cameras found");

    println!(
        "Using camera: {}",
        *cam.properties().get::<properties::Model>().unwrap()
    );

    let mut cam = cam.acquire().expect("Unable to acquire camera");

    let mut cfgs = cam.generate_configuration(&[StreamRole::StillCapture]).unwrap();
    let cfg = cfgs.get_mut(0).unwrap();

    println!("Generated config: {:#?}", cfg);

    if cfgs.validate().is_invalid() {
        panic!("Error validating camera configuration");
    }

    cam.configure(&mut cfgs).expect("Unable to configure camera");

    let req = cam.create_request(None).unwrap();

    cam.start(None).unwrap();

    cam.queue_request(&req).unwrap();

    cam.stop().unwrap();
}
