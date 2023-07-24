#![allow(unused_imports, unused_qualifications, unused_extern_crates)]
extern crate chrono;
extern crate uuid;


use serde::ser::Serializer;

use std::collections::HashMap;
use models;
use swagger;


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Wgs84Decoded {
    /// from -90 to +90 [degrees]
    #[serde(rename = "latitude")]
    pub latitude: f64,

    /// from -180 to +180 [degrees]
    #[serde(rename = "longitude")]
    pub longitude: f64,

    /// increment of 5 from 0 to 327 675 [meters]
    #[serde(rename = "innerRadiuse")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub inner_radiuse: Option<f64>,

    /// from 0 to 1 800 000 [meters]
    #[serde(rename = "uncertaintyRadiuse")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub uncertainty_radiuse: Option<f64>,

    /// increment of 2 from 0 to 359.9...9 [degrees]
    #[serde(rename = "offsetAngle")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub offset_angle: Option<f64>,

    /// increment of 2 from 0.0...1 to 360 [degrees]
    #[serde(rename = "includedAngle")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub included_angle: Option<f64>,

    #[serde(rename = "confidence")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub confidence: Option<f64>,

}

impl Wgs84Decoded {
    pub fn new(latitude: f64, longitude: f64, ) -> Wgs84Decoded {
        Wgs84Decoded {
            latitude: latitude,
            longitude: longitude,
            inner_radiuse: None,
            uncertainty_radiuse: None,
            offset_angle: None,
            included_angle: None,
            confidence: None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Wgs84Encoded {
    #[serde(rename = "wgs84")]
    pub wgs84: String,

}

impl Wgs84Encoded {
    pub fn new(wgs84: String, ) -> Wgs84Encoded {
        Wgs84Encoded {
            wgs84: wgs84,
        }
    }
}
