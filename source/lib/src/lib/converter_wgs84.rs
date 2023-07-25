
use itertools::Itertools;


pub struct WGS84 {
    pub wgs84: String,
    pub shape: String,
    pub latitude: f32,
    pub longitude: f32,
    pub inner_radius: u16,
    pub uncertainty_radius: u16,
    pub offset_angle: u16,
    pub included_angle: u16,
    pub confidence: u8,
}

impl WGS84 {

    pub fn encode(&mut self) {
        let wgs84 = format!(
            "{}{}{}{}{}{}{}{}",
            Self::enc_shape(&self.shape),
            Self::enc_latitude(self.latitude),
            Self::enc_longitude(self.longitude),
            Self::enc_inner_radius(self.inner_radius),
            Self::enc_uncertainty_radius(self.uncertainty_radius),
            Self::enc_offset_angle(self.offset_angle),
            Self::enc_included_angle(self.included_angle),
            Self::enc_confidence(self.confidence)
        );

        self.wgs84 = wgs84
            .chars()
            .chunks(8)
            .into_iter()
            .map(|chunk| chunk.collect::<String>())
            .join(" ");
    }

    pub fn decode(&mut self) {}

    // =====================================================================

    fn enc_shape(shape: &str) -> String {
        let enc = match shape {
            "00" => "00000000".to_string(),
            "10" => "00010000".to_string(),
            "30" => "00110000".to_string(),
            "50" => "01010000".to_string(),
            "80" => "10000000".to_string(),
            "90" => "10010000".to_string(),
            "A0" => "10100000".to_string(),
            _ => "".to_string(),
        };

        return enc.to_string();
    }

    fn enc_latitude(latitude: f32) -> String {
        let is_neg: bool = latitude < 0.0;
        let lat: f32 = (2_f32.powf(23.0) * latitude.abs()) / 90.0;
        let lat: u32 = lat.round() as u32;

        let enc: u32 = (is_neg as u32) << 23 as u32 | lat;

        return format!("{:0<24b}", enc);
    }

    fn enc_longitude(longitude: f32) -> String {
        let is_neg: bool = longitude < 0.0;
        let long: f32 = (2_f32.powf(24.0) * longitude.abs()) / 360.0;
        let long: u32 = long.round() as u32;

        let mut _enc = format!("{:0>24b}", long);

        if is_neg {
            _enc = Self::to_twos_complement(_enc)
        }

        let enc = isize::from_str_radix(&_enc, 2).unwrap();

        return format!("{:0<24b}", enc);
    }

    fn enc_inner_radius(ir: u16) -> String {
        let ir: u16 = ir / 5;
        let enc: String = format!("{:0<16b}", ir);

        return enc;
    }

    fn enc_uncertainty_radius(ur: u16) -> String {
        let ur: f32 = ((10 + ur) / 10) as f32;

        let _enc: f32 = ur.log(1.1);
        let _enc: u16 = _enc.round() as u16;

        let mut enc: String = format!("{:0<7b}", _enc);
        enc = "0".to_owned() + &enc;

        return enc;
    }

    fn enc_offset_angle(oa: u16) -> String {
        let oa: u16 = oa / 2;
        let enc: String = format!("{:0<8b}", oa);

        return enc;
    }

    fn enc_included_angle(ia: u16) -> String {
        let ia: u16 = ia / 2;
        let enc: String = format!("{:0<8b}", ia);

        return enc;
    }

    fn enc_confidence(cf: u8) -> String {
        let mut enc: String = format!("{:0<7b}", cf);
        enc = "0".to_owned() + &enc;

        return enc;
    }

    // =====================================================================

    fn dec_latitude(_bin: String) {

    }

    fn dec_longitude(_bin: String) {

    }

    fn dec_inner_radius(_bin: String) {

    }

    fn dec_uncertainty_radius(_bin: String) {

    }

    fn dec_offset_angle(_bin: String) {

    }

    fn dec_included_angle(_bin: String) {

    }

    fn dec_confidence(_bin: String) {

    }

    // =====================================================================

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

        return num;
    }

    // =====================================================================

    pub fn display(&self) {
        println!("=======================================================================");
        println!("WGS84: ------------------------ {}", &self.wgs84);
        println!("=======================================================================");
        println!("Shape: ------------------------ {}", &self.shape);
        println!("Latitude: --------------------- {}", &self.latitude);
        println!("Longitude: -------------------- {}", &self.longitude);
        println!("Inner radius: ----------------- {}", &self.inner_radius);
        println!("Uncertainty radius: ----------- {}", &self.uncertainty_radius);
        println!("Offset angle: ----------------- {}", &self.offset_angle);
        println!("Included angle: --------------- {}", &self.included_angle);
        println!("Confidence: ------------------- {}", &self.confidence);
        println!("=======================================================================");
    }
}
