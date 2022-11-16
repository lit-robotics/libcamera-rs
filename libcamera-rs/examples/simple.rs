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

    let mut cfgs = cam.generate_configuration(&[StreamRole::ViewFinder]).unwrap();

    println!("Generated config: {:#?}", cfgs);

    if cfgs.validate().is_invalid() {
        panic!("Error validating camera configuration");
    }

    cam.configure(&mut cfgs).expect("Unable to configure camera");

    let mut alloc = FrameBufferAllocator::new(&cam);

    let cfg = cfgs.get(0).unwrap();
    let stream = cfg.stream().unwrap();
    alloc.allocate(&stream).unwrap();

    let buffers = alloc.buffers(&stream);
    println!("Allocated {} buffers", buffers.len());

    let mut reqs = Vec::new();

    for i in 0..buffers.len() {
        let mut req = cam.create_request(None).unwrap();
        req.add_buffer(&stream, &alloc.buffers(&stream).get(i).unwrap())
            .unwrap();
        reqs.push(req);
    }

    cam.on_request_completed(|req| {
        println!("Req: {:#?}", req.metadata());
    });

    cam.start(None).unwrap();

    for req in reqs {
        cam.queue_request(req).unwrap();
    }

    std::thread::sleep(std::time::Duration::from_secs(5));

    cam.stop().unwrap();
}
