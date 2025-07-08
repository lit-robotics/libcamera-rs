use libcamera::{camera_manager::CameraManager, controls::ControlId, logging::LoggingLevel};

fn main() {
    let mgr = CameraManager::new().unwrap();

    mgr.log_set_level("Camera", LoggingLevel::Error);

    let cameras = mgr.cameras();

    // Grab the first camera, if one exists
    if let Some(cam) = cameras.iter().next() {
        println!("ID: {}", cam.id());

        // Read the first ControlInfo
        if let Some((id, control_info)) = cam.controls().into_iter().next() {
            // Attempt to get ControlID
            match ControlId::try_from(id) {
                Ok(control) => println!("Control Id {} - {:?}", id, control),
                Err(_) => println!("Control Id {id} - UNKOWN"),
            }

            println!("Control Max: {:?}", control_info.max());
            println!("Control Min: {:?}", control_info.min());
            println!("Control Defualt: {:?}", control_info.def());

            let values = control_info.values();

            // Some controls only support specific values within their ranges.
            // this will display those possible values if they exist
            if !values.is_empty() {
                println!("Supported Values:");
                for value in values {
                    println!("{value:?}");
                }
            }
        }
    } else {
        eprintln!("No cameras found");
    };
}
