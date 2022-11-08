use libcamera_rs::{properties, CameraManager};

fn main() {
    let mgr = CameraManager::new().unwrap();

    let cameras = mgr.cameras();

    for i in 0..cameras.len() {
        let cam = cameras.get(i).unwrap();
        println!("Camera {}", i);
        println!("ID: {}", cam.id());

        let props = cam.properties();
        println!("Model: {}", props.get::<properties::Model>().unwrap().0);
        println!(
            "Location: {:?}",
            props.get::<properties::Location>().unwrap()
        );
    }
}
