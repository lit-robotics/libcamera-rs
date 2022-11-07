use libcamera_meta::control_ids;

fn main() {
    let control_ids = control_ids();
    println!("{:#?}", control_ids);
}
