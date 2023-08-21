use cell_tower_geolocation::WGS84;

fn main() {
    let mut wgs84 = WGS84 {
        wgs84: String::new(),
        shape: "A0".to_string(),
        latitude: -51.508037,
        longitude: 12.751280,
        inner_radius: 8500,
        uncertainty_radius: 400,
        offset_angle: 280,
        included_angle: 30,
        confidence: 90,
    };

    wgs84.decode();
    wgs84.encode();
    wgs84.display();
}

// 1100 1001010000 0110000001
// 10010010100000110000001
// 10010010 10000011 00000011

// 100100101000001100000010
// 10010010100000110000001
// 100100101000001100000010