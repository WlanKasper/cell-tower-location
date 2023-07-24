use cell_tower_geolocation::WGS84;

fn main() {
    let mut coordinates = WGS84 {
        wgs84: String::new(),
        shape: String::new(),
        latitude: 51.508037,
        longitude: 12.751280,
        inner_radius: 55,
        uncertainty_radius: 57,
        offset_angle: 348,
        included_angle: 66,
        confidence: 90,
    };

    coordinates.encode();

    println!("{0}", coordinates.wgs84);
}

// A0 49 41 81 09 11 4C 00 0B 12 AE 21 5A
// A0 49 41 81 09 11 4D 00 0B 13 AE 21 5A
