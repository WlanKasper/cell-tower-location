use cell_tower_geolocation::WGS84;

fn main() {
    let mut coordinates = WGS84 {
        wgs84: String::new(),
        shape: "A0".to_string(),
        latitude: 51.508037,
        longitude: 12.751280,
        inner_radius: 55,
        uncertainty_radius: 57,
        offset_angle: 348,
        included_angle: 66,
        confidence: 90,
    };

    coordinates.encode();
    coordinates.display();
}
