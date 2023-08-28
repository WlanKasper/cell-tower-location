#![allow(unused_qualifications)]

use validator::Validate;
use serde_json::Value;
use std::convert::TryInto;

use crate::models;
#[cfg(any(feature = "client", feature = "server"))]
use crate::header;

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct Wgs84Decoded {
    /// from -90 to +90 [degrees]
    #[serde(rename = "latitude")]
    pub latitude: serde_json::Value,

    /// from -180 to +180 [degrees]
    #[serde(rename = "longitude")]
    pub longitude: serde_json::Value,

    /// increment of 5 from 0 to 327 675 [meters]
    #[serde(rename = "innerRadius")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub inner_radius: Option<serde_json::Value>,

    /// from 0 to 1 800 000 [meters]
    #[serde(rename = "uncertaintyRadius")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub uncertainty_radius: Option<serde_json::Value>,

    /// increment of 2 from 0 to 359.9...9 [degrees]
    #[serde(rename = "offsetAngle")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub offset_angle: Option<serde_json::Value>,

    /// increment of 2 from 0.0...1 to 360 [degrees]
    #[serde(rename = "includedAngle")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub included_angle: Option<serde_json::Value>,

    #[serde(rename = "confidence")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub confidence: Option<serde_json::Value>,

}


impl Wgs84Decoded {
    #[allow(clippy::new_without_default)]
    pub fn new(latitude: serde_json::Value, longitude: serde_json::Value, ) -> Wgs84Decoded {
        Wgs84Decoded {
            latitude,
            longitude,
            inner_radius: None,
            uncertainty_radius: None,
            offset_angle: None,
            included_angle: None,
            confidence: None,
        }
    }

    pub fn decode(encoded: Wgs84Encoded) -> Wgs84Decoded {
        let binary_values: Vec<&str> = encoded.split_whitespace().collect();

        return Wgs84Decoded {
            latitude: Self::dec_latitude(&format!("{}{}{}", binary_values[1], binary_values[2], binary_values[3])),
            longitude: Self::dec_longitude(&format!("{}{}{}", binary_values[4], binary_values[5], binary_values[6])),
            inner_radius: Self::dec_inner_radius(&format!("{}{}", binary_values[7], binary_values[8])),
            uncertainty_radius: Self::dec_uncertainty_radius(binary_values[9]),
            offset_angle: Self::dec_offset_angle(binary_values[10]),
            included_angle: Self::dec_included_angle(binary_values[11]),
            confidence: Self::dec_confidence(binary_values[12]),
        }
    }

    fn dec_latitude(_bin: &str) -> f32 {
        let mut lat_int: i64 = Self::bin_to_i64(_bin);

        let is_neg = lat_int >> 23;
        lat_int = lat_int & ((1 << 23) - 1);

        let mut latitude: f32 = (90.0 * lat_int as f32) / 2_i32.pow(23) as f32;
        if is_neg == 1 {latitude *= -1.0}

        return latitude;
    }

    fn dec_longitude(_bin: &str) -> f32 {
        let long = if _bin.starts_with('1') {
            - Self::from_twos_complement(_bin)
        } else {
            Self::bin_to_i64(_bin)
        };

        let long: f32 = (long * 360) as f32 / (2_i32.pow(24) as f32);

        return long;
    }

    fn dec_inner_radius(_bin: &str) -> u16{
        let inner_radius: i64 = Self::bin_to_i64(_bin);

        return (inner_radius as u16) * 5;
    }

    fn dec_uncertainty_radius(_bin: &str) -> u16 {
        let uncertainty_radius: i64 = Self::bin_to_i64(_bin);

        return (1.1_f32.powf(uncertainty_radius as f32).round() as u16 * 10) - 1;
    }

    fn dec_offset_angle(_bin: &str) -> u16 {
        let offset_angle: i64 = Self::bin_to_i64(_bin);

        return (offset_angle as u16 + 1) * 2;
    }

    fn dec_included_angle(_bin: &str) -> u16{
        let included_angle: i64 = Self::bin_to_i64(_bin);

        return (included_angle as u16 + 1) * 2;
    }

    fn dec_confidence(_bin: &str) -> u8{
        return Self::bin_to_i64(_bin) as u8;
    }

    fn bin_to_i64(_bin: &str) -> i64{
        return i64::from_str_radix(_bin, 2).unwrap();
    }

    fn from_twos_complement(bin_str: &str) -> i64 {
        let flipped: String = bin_str.chars().map(|c| match c {
            '0' => '1',
            '1' => '0',
            _ => c,
        }).collect();

        let result: i64 = i64::from_str_radix(&flipped, 2).unwrap() + 1;

        return result;
    }
}

/// Converts the Wgs84Decoded value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for Wgs84Decoded {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![
            // Skipping latitude in query parameter serialization

            // Skipping longitude in query parameter serialization

            // Skipping innerRadius in query parameter serialization

            // Skipping uncertaintyRadius in query parameter serialization

            // Skipping offsetAngle in query parameter serialization

            // Skipping includedAngle in query parameter serialization

            // Skipping confidence in query parameter serialization

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a Wgs84Decoded value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for Wgs84Decoded {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub latitude: Vec<serde_json::Value>,
            pub longitude: Vec<serde_json::Value>,
            pub inner_radius: Vec<serde_json::Value>,
            pub uncertainty_radius: Vec<serde_json::Value>,
            pub offset_angle: Vec<serde_json::Value>,
            pub included_angle: Vec<serde_json::Value>,
            pub confidence: Vec<serde_json::Value>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing Wgs84Decoded".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "latitude" => intermediate_rep.latitude.push(<serde_json::Value as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "longitude" => intermediate_rep.longitude.push(<serde_json::Value as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "innerRadius" => intermediate_rep.inner_radius.push(<serde_json::Value as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "uncertaintyRadius" => intermediate_rep.uncertainty_radius.push(<serde_json::Value as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "offsetAngle" => intermediate_rep.offset_angle.push(<serde_json::Value as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "includedAngle" => intermediate_rep.included_angle.push(<serde_json::Value as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "confidence" => intermediate_rep.confidence.push(<serde_json::Value as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing Wgs84Decoded".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(Wgs84Decoded {
            latitude: intermediate_rep.latitude.into_iter().next().ok_or_else(|| "latitude missing in Wgs84Decoded".to_string())?,
            longitude: intermediate_rep.longitude.into_iter().next().ok_or_else(|| "longitude missing in Wgs84Decoded".to_string())?,
            inner_radius: intermediate_rep.inner_radius.into_iter().next(),
            uncertainty_radius: intermediate_rep.uncertainty_radius.into_iter().next(),
            offset_angle: intermediate_rep.offset_angle.into_iter().next(),
            included_angle: intermediate_rep.included_angle.into_iter().next(),
            confidence: intermediate_rep.confidence.into_iter().next(),
        })
    }
}

// Methods for converting between header::IntoHeaderValue<Wgs84Decoded> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<Wgs84Decoded>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<Wgs84Decoded>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for Wgs84Decoded - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<Wgs84Decoded> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <Wgs84Decoded as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into Wgs84Decoded - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct Wgs84Encoded {
    #[serde(rename = "wgs84")]
    pub wgs84: serde_json::Value,

}


impl Wgs84Encoded {
    #[allow(clippy::new_without_default)]
    pub fn new(wgs84: serde_json::Value, ) -> Wgs84Encoded {
        Wgs84Encoded {
            wgs84,
        }
    }

    pub fn normilize(&mut self) {
        self.wgs84 = match Self::value_to_string(Self::remove_spaces_from_value(&self.wgs84)) {
            Ok(res) => {
                res
                    .chars()
                    .chunks(8)
                    .into_iter()
                    .map(|chunk| chunk.collect::<String>())
                    .join(" ")
            },
            Err(error_msg) => Error,
        }
    }

    fn remove_spaces_from_value(value: &serde_json::Value) -> serde_json::Value {
        if let Value::String(s) = value {
            let new_string = s.replace(" ", "");
            Value::String(new_string)
        } else {
            value.clone()
        }
    }

    pub fn encode(decoded: &Wgs84Decoded) -> Wgs84Encoded {
        return Wgs84Encoded {
            wgs84: serde_json::json!(format!(
                "{}{}{}{}{}{}{}",
                Self::enc_latitude(decoded.latitude),
                Self::enc_longitude(decoded.longitude),
                Self::enc_inner_radius(decoded.inner_radius),
                Self::enc_uncertainty_radius(decoded.uncertainty_radius),
                Self::enc_offset_angle(decoded.offset_angle),
                Self::enc_included_angle(decoded.included_angle),
                Self::enc_confidence(decoded.confidence)
            )),
        };
    }

    fn value_to_f32(value: Value) -> Result<f32, &'static str> {
        if let Some(parsed_f32) = value.as_f64().and_then(|n| <f64 as TryInto<T>>::try_into(n).ok()) {
            Ok(parsed_f32 as f32)
        } else {
            Err("Value couldn't be converted to f32")
        }
    }

    fn option_value_to_u16(option_value: Option<Value>) -> Result<u16, &'static str> {
        if let Some(value) = option_value {
            if let Some(parsed_u16) = value.as_u64().and_then(|n| n.try_into().ok()) {
                Ok(parsed_u16)
            } else {
                Err("Value couldn't be converted to u16")
            }
        } else {
            Err("Option is None")
        }
    }
    
    fn option_value_to_u8(option_value: Option<Value>) -> Result<u8, &'static str> {
        if let Some(value) = option_value {
            if let Some(parsed_u8) = value.as_u64().and_then(|n| <u64 as TryInto<T>>::try_into(n).ok()) {
                if parsed_u8 <= u8::MAX.into() {
                    Ok(parsed_u8 as u8)
                } else {
                    Err("Value is out of u8 range")
                }
            } else {
                Err("Value couldn't be converted to u8")
            }
        } else {
            Err("Option is None")
        }
    }
    
    fn value_to_string(value: Value) -> Result<String, &'static str> {
        if let Some(parsed_string) = value.as_str() {
            Ok(parsed_string.to_string())
        } else {
            Err("Value couldn't be converted to String")
        }
    }

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

    fn enc_latitude(latitudeValue: serde_json::Value) -> String {
        return match Self::value_to_f32(latitudeValue) {
            Ok(latitude) => {
                let is_neg: bool = latitude < 0.0;
                let lat: f32 = (2_f32.powf(23.0) * latitude.abs()) / 90.0;
                let lat: u32 = lat.round() as u32;
        
                let enc: u32 = (is_neg as u32) << 23 | lat;
        
                format!("{:0>24b}", enc)
            },
            Err(error_msg) => {
                println!("Error: {}", error_msg);
                String::from("Error")
            },
        };
    }

    fn enc_longitude(longitudeValue: serde_json::Value) -> String {
        return match Self::value_to_f32(longitudeValue) {
            Ok(longitude) => {
                let is_neg: bool = longitude < 0.0;
                let long: f32 = (2_f32.powf(24.0) * longitude.abs()) / 360.0;
                let long: u32 = long.round() as u32;
        
                let mut _enc = format!("{:0>24b}", long);
        
                if is_neg {
                    _enc = Self::to_twos_complement(_enc)
                }
        
                let enc = isize::from_str_radix(&_enc, 2).unwrap();
        
                format!("{:0>24b}", enc)
            },
            Err(error_msg) => {
                println!("Error: {}", error_msg);
                String::from("Error")
            },
        };
    }

    fn enc_inner_radius(irValue: Option<serde_json::Value>) -> String {
        return match Self::option_value_to_u16(irValue) {
            Ok(ir) => {
                let ir: u16 = ir / 5;
                let enc: String = format!("{:0>16b}", ir);
        
                enc
            },
            Err(error_msg) => {
                println!("Error: {}", error_msg);
                String::from("Error")
            },
        };
    }

    fn enc_uncertainty_radius(urValue: Option<serde_json::Value>) -> String {
        return match Self::option_value_to_u16(urValue) {
            Ok (ur) => {
                let ur: f32 = ((10 + ur) / 10) as f32;

                let _enc: f32 = ur.log(1.1);
                let _enc: u16 = _enc.round() as u16;
        
                let mut enc: String = format!("{:0>7b}", _enc);
                enc = "0".to_owned() + &enc;
        
                enc
            },
            Err(error_msg) => {
                println!("Error: {}", error_msg);
                String::from("Error")
            },
        };
    }

    fn enc_offset_angle(oaValue: Option<serde_json::Value>) -> String {
        return match Self::option_value_to_u16(oaValue) {
            Ok(oa) => {
                let oa: u16 = oa / 2;
                let enc: String = format!("{:0>8b}", oa);
        
                enc
            },
            Err(error_msg) => {
                println!("Error: {}", error_msg);
                String::from("Error")
            },
        };
    }

    fn enc_included_angle(iaValue: Option<serde_json::Value>) -> String {
        return match Self::option_value_to_u16(iaValue) {
            Ok(ia) => {
                let ia: u16 = ia / 2;
                let enc: String = format!("{:0>8b}", ia);
        
                enc
            },
            Err(error_msg) => {
                println!("Error: {}", error_msg);
                String::from("Error")
            },
        };
    }

    fn enc_confidence(cfValue: Option<serde_json::Value>) -> String {
        return match Self::option_value_to_u8(cfValue) {
            Ok(cf) => {
                let mut enc: String = format!("{:0>7b}", cf);
                enc = "0".to_owned() + &enc;
        
                enc
            }
            Err(error_msg) => {
                println!("Error: {}", error_msg);
                String::from("Error")
            },
        };
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

        return num;
    }
}

/// Converts the Wgs84Encoded value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for Wgs84Encoded {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![
            // Skipping wgs84 in query parameter serialization

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a Wgs84Encoded value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for Wgs84Encoded {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub wgs84: Vec<serde_json::Value>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing Wgs84Encoded".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "wgs84" => intermediate_rep.wgs84.push(<serde_json::Value as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing Wgs84Encoded".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(Wgs84Encoded {
            wgs84: intermediate_rep.wgs84.into_iter().next().ok_or_else(|| "wgs84 missing in Wgs84Encoded".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<Wgs84Encoded> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<Wgs84Encoded>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<Wgs84Encoded>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for Wgs84Encoded - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<Wgs84Encoded> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <Wgs84Encoded as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into Wgs84Encoded - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}

