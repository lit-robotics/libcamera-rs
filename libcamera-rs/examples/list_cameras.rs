use libcamera_rs::CameraManager;

fn main() {
    let mgr = CameraManager::new().unwrap();

    let cameras = mgr.cameras();

    for i in 0..cameras.len() {
        let cam = cameras.get(i).unwrap();
        println!("Camera {} ID: {}", i, cam.id())
    }
}
