use libcamera_rs::{
    camera_manager::CameraManager, framebuffer_allocator::FrameBufferAllocator, properties, stream::StreamRole,
};

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

    println!("Generated config: {:#?}", cfgs);

    if cfgs.validate().is_invalid() {
        panic!("Error validating camera configuration");
    }

    cam.configure(&mut cfgs).expect("Unable to configure camera");

    let mut alloc = FrameBufferAllocator::new(&cam);

    let cfg = cfgs.get(0).unwrap();
    let stream = cfg.stream().unwrap();
    alloc.allocate(&stream).unwrap();

    println!("Allocated {} buffers", alloc.buffers(&stream).len());

    let mut req = cam.create_request(None).unwrap();
    req.add_buffer(&stream, &alloc.buffers(&stream).get(0).unwrap())
        .unwrap();

    cam.start(None).unwrap();

    cam.queue_request(&req).unwrap();

    cam.stop().unwrap();
}
