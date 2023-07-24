struct WGS84 {
    shape: String,
    latitude: f32,
    longitude: f32,
    inner_radius: u16,
    uncertainty_radius: u16, 
    offset_angle: u16,
    included_angle: u16,
    confidence: u8,
}

fn main() {
    let coordinates = WGS84 {
        shape: "A0",
        latitude: 45.766617,
        longitude: 12.751280,
        inner_radius: 60,
        uncertainty_radius: 410, 
        offset_angle: 350,
        included_angle: 68,
        confidence: 90,
    };

    // Add check of values
    // Ex: lat > 90 || lat < -90

    set_latitude(coordinates.latitude)

}

fn set_latitude(latitude: f32) {
    let encoded: [u32; 3];

    let sign: bool = latitude >= 0;
    let latitude = latitude.to_bits(); 

    println!(latitude)
}