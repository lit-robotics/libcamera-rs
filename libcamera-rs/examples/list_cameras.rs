use libcamera_rs::{camera_manager::CameraManager, properties, stream::StreamRole};

fn main() {
    let mgr = CameraManager::new().unwrap();

    let cameras = mgr.cameras();

    for i in 0..cameras.len() {
        let cam = cameras.get(i).unwrap();
        println!("Camera {}", i);
        println!("ID: {}", cam.id());

        let props = cam.properties();
        println!("Location: {:?}", props.get::<properties::Location>());
        println!("Rotation: {:?}", props.get::<properties::Rotation>());
        println!("Model: {:?}", props.get::<properties::Model>());
        println!("UnitCellSize: {:?}", props.get::<properties::UnitCellSize>());
        println!("PixelArraySize: {:?}", props.get::<properties::PixelArraySize>());
        println!("SensorSensitivity: {:?}", props.get::<properties::SensorSensitivity>());
        println!(
            "ColorFilterArrangement: {:?}",
            props.get::<properties::ColorFilterArrangement>()
        );
        println!("");

        let config = cam.generate_configuration(&[StreamRole::ViewFinder]).unwrap();
        let view_finder_cfg = config.get(0).unwrap();
        let formats = view_finder_cfg.formats();
        for pixel_format in &formats.pixel_formats() {
            println!("{:?}", pixel_format);
            for size in formats.sizes(&pixel_format) {
                println!("  {:?}", size);
            }
            println!("  {:?}", formats.range(&pixel_format));
        }
    }
}
