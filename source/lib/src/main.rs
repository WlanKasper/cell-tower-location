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
    // 0 - N45.766624-12.750064
    // 0 - N45.766613-12.751265
    let coordinates = WGS84 {
        shape: String::new(),
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

    let latitude = encode_latitude(coordinates.latitude);
    let longitude = encode_longitude(coordinates.longitude);

    println!("{latitude}");
    println!("{longitude}");
}

fn encode_latitude(latitude: f32) -> String {
    let is_negative: bool = latitude < 0.0;
    let encoded: f32 = (2_f32.powf(23.0) * latitude.abs()) / 90.0;

    let encoded: u32 = (is_negative as u32) << 23 as u32 | encoded.round() as u32;

    return format!("{:0>6X}", encoded);
}

fn encode_longitude(longitude: f32) -> String {
    let is_negative : bool = longitude < 0.0;
    let encoded: f32 = (2_f32.powf(24.0) * longitude.abs()) / 360.0;
    let encoded: u32 = encoded.round() as u32;
    let mut encoded = format!("{:0>24b}", encoded);

    if is_negative { encoded = to_twos_complement(encoded) }

    let encoded = isize::from_str_radix(&encoded, 2).unwrap();


    return format!("{:0>6X}", encoded);
}

fn to_twos_complement(s: String) -> String {
    let mut num: String = s.to_string();
    let n = num.len();

    let mut i = n as isize - 1;
    while let Some(c) = num.chars().nth(i as usize) {
        if c == '1' {
            break;
        }
        i -= 1;
    }

    if i == -1 {
        num = format!("1{}", num);
    } else {
        let mut k = i - 1;
        while k >= 0 {
            let c = num.chars().nth(k as usize).unwrap();
            if c == '1' {
                num.replace_range(k as usize..=k as usize, "0");
            } else {
                num.replace_range(k as usize..=k as usize, "1");
            }
            k -= 1;
        }
    }

    num
}
