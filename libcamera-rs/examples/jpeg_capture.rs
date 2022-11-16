use std::time::Duration;

use libcamera_rs::{
    camera_manager::CameraManager, framebuffer_allocator::FrameBufferAllocator,
    framebuffer_map::MemoryMappedFrameBuffer, properties, stream::StreamRole,
};

fn main() {
    let filename = std::env::args().nth(1).expect("Usage ./jpeg_capture <filename.jpg>");

    let mgr = CameraManager::new().unwrap();

    let cameras = mgr.cameras();

    let cam = cameras.get(0).expect("No cameras found");

    println!(
        "Using camera: {}",
        *cam.properties().get::<properties::Model>().unwrap()
    );

    let mut cam = cam.acquire().expect("Unable to acquire camera");

    // This will generate default configuration for each specified role
    let mut cfgs = cam.generate_configuration(&[StreamRole::ViewFinder]).unwrap();

    // TODO: Implement pixel format enum to manually set format
    assert_eq!(
        cfgs.get(0).unwrap().get_pixel_format().to_string(),
        "MJPEG",
        "Only MJPEG format is supported"
    );

    println!("Generated config: {:#?}", cfgs);

    if cfgs.validate().is_invalid() {
        panic!("Error validating camera configuration");
    }

    cam.configure(&mut cfgs).expect("Unable to configure camera");

    let mut alloc = FrameBufferAllocator::new(&cam);

    // Allocate frame buffers for the the stream
    let cfg = cfgs.get(0).unwrap();
    let stream = cfg.stream().unwrap();
    alloc.allocate(&stream).unwrap();

    let buffers = alloc.buffers(&stream);
    println!("Allocated {} buffers", buffers.len());

    // Create capture requests and attach buffers
    let mut reqs = Vec::new();
    for i in 0..buffers.len() {
        let mut req = cam.create_request(None).unwrap();
        req.add_buffer(&stream, &alloc.buffers(&stream).get(i).unwrap())
            .unwrap();
        reqs.push(req);
    }

    // Completed capture requests are returned as a callback
    let (tx, rx) = std::sync::mpsc::channel();
    cam.on_request_completed(move |req| {
        tx.send(req).unwrap();
    });

    cam.start(None).unwrap();

    // Multiple requests can be queued at a time, but for this example we just want a single frame
    cam.queue_request(reqs.pop().unwrap()).unwrap();

    println!("Waiting for camera request execution");
    let req = rx.recv_timeout(Duration::from_secs(2)).expect("Camera request failed");

    println!("Camera request {:?} completed!", req);
    println!("Metadata: {:#?}", req.metadata());

    // Get framebuffer for our stream
    let framebuffer = req.find_buffer(&stream).unwrap();
    println!("FrameBuffer metadata: {:#?}", framebuffer.metadata());

    // Memory map framebuffer to obtain &[u8] slice to raw data.
    // For continuous streaming this should be done once during initilization since buffers are reused.
    let mapped_fb = MemoryMappedFrameBuffer::from_framebuffer(&framebuffer).unwrap();

    // MJPEG format has only one data plane containing encoded jpeg data with all the headers
    let planes = mapped_fb.planes();
    let jpeg_data = planes.get(0).unwrap();

    std::fs::write(&filename, jpeg_data).unwrap();
    println!("Written {} bytes to {}", jpeg_data.len(), &filename);

    // Everything is cleaned up automatically by Drop implementations
}
