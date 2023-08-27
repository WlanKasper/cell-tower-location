#![allow(unused_qualifications)]

use validator::Validate;

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
    #[serde(rename = "innerRadiuse")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub inner_radiuse: Option<serde_json::Value>,

    /// from 0 to 1 800 000 [meters]
    #[serde(rename = "uncertaintyRadiuse")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub uncertainty_radiuse: Option<serde_json::Value>,

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
            inner_radiuse: None,
            uncertainty_radiuse: None,
            offset_angle: None,
            included_angle: None,
            confidence: None,
        }
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

            // Skipping innerRadiuse in query parameter serialization

            // Skipping uncertaintyRadiuse in query parameter serialization

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
            pub inner_radiuse: Vec<serde_json::Value>,
            pub uncertainty_radiuse: Vec<serde_json::Value>,
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
                    "innerRadiuse" => intermediate_rep.inner_radiuse.push(<serde_json::Value as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "uncertaintyRadiuse" => intermediate_rep.uncertainty_radiuse.push(<serde_json::Value as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
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
            inner_radiuse: intermediate_rep.inner_radiuse.into_iter().next(),
            uncertainty_radiuse: intermediate_rep.uncertainty_radiuse.into_iter().next(),
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

