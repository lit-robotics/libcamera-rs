use libcamera::{camera_manager::CameraManager, controls::ControlId, logging::LoggingLevel};

fn main() {
    let mgr = CameraManager::new().unwrap();

    mgr.log_set_level("Camera", LoggingLevel::Error);

    let cameras = mgr.cameras();

    //Grab the first camera, if one exists
    for cam in cameras.iter().take(1) {
        println!("ID: {}", cam.id());

        //Read the first ControlInfo
        for (id, control_info) in cam.controls().into_iter().take(1) {
            //Attempt to get ControlID
            match ControlId::try_from(id) {
                Ok(id) => println!("Control Id {:?} - {:?}", id as u32, id),
                Err(_) => println!("Control Id {:?} - UNKOWN", id),
            }

            println!("Control Max: {:?}", control_info.max());
            println!("Control Min: {:?}", control_info.min());
            println!("Control Defualt: {:?}", control_info.def());

            let values = control_info.values();

            //Some controls only support specific values within their ranges.
            //this will display those possible values if they exist
            if values.len() > 0 {
                println!("Supported Values:");
                for value in values {
                    println!("{:?}", value);
                }
            }
        }
    }
}
