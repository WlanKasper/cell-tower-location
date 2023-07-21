/* 
 * Cell Tower Geolocation
 *
 * This is a cell tower geolocation converter on the OpenAPI 3.0 specification.
 *
 * OpenAPI spec version: 0.1.0
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */


#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct Wgs84Decoded {
  /// from -90 to +90 [degrees]
  #[serde(rename = "latitude")]
  latitude: f64,
  /// from -180 to +180 [degrees]
  #[serde(rename = "longitude")]
  longitude: f64,
  /// increment of 5 from 0 to 327 675 [meters]
  #[serde(rename = "innerRadiuse")]
  inner_radiuse: Option<f32>,
  /// from 0 to 1 800 000 [meters]
  #[serde(rename = "uncertaintyRadiuse")]
  uncertainty_radiuse: Option<f32>,
  /// increment of 2 from 0 to 359.9...9 [degrees]
  #[serde(rename = "offsetAngle")]
  offset_angle: Option<f32>,
  /// increment of 2 from 0.0...1 to 360 [degrees]
  #[serde(rename = "includedAngle")]
  included_angle: Option<f32>,
  #[serde(rename = "confidence")]
  confidence: Option<f32>
}

impl Wgs84Decoded {
  pub fn new(latitude: f64, longitude: f64) -> Wgs84Decoded {
    Wgs84Decoded {
      latitude: latitude,
      longitude: longitude,
      inner_radiuse: None,
      uncertainty_radiuse: None,
      offset_angle: None,
      included_angle: None,
      confidence: None
    }
  }

  pub fn set_latitude(&mut self, latitude: f64) {
    self.latitude = latitude;
  }

  pub fn with_latitude(mut self, latitude: f64) -> Wgs84Decoded {
    self.latitude = latitude;
    self
  }

  pub fn latitude(&self) -> &f64 {
    &self.latitude
  }


  pub fn set_longitude(&mut self, longitude: f64) {
    self.longitude = longitude;
  }

  pub fn with_longitude(mut self, longitude: f64) -> Wgs84Decoded {
    self.longitude = longitude;
    self
  }

  pub fn longitude(&self) -> &f64 {
    &self.longitude
  }


  pub fn set_inner_radiuse(&mut self, inner_radiuse: f32) {
    self.inner_radiuse = Some(inner_radiuse);
  }

  pub fn with_inner_radiuse(mut self, inner_radiuse: f32) -> Wgs84Decoded {
    self.inner_radiuse = Some(inner_radiuse);
    self
  }

  pub fn inner_radiuse(&self) -> Option<&f32> {
    self.inner_radiuse.as_ref()
  }

  pub fn reset_inner_radiuse(&mut self) {
    self.inner_radiuse = None;
  }

  pub fn set_uncertainty_radiuse(&mut self, uncertainty_radiuse: f32) {
    self.uncertainty_radiuse = Some(uncertainty_radiuse);
  }

  pub fn with_uncertainty_radiuse(mut self, uncertainty_radiuse: f32) -> Wgs84Decoded {
    self.uncertainty_radiuse = Some(uncertainty_radiuse);
    self
  }

  pub fn uncertainty_radiuse(&self) -> Option<&f32> {
    self.uncertainty_radiuse.as_ref()
  }

  pub fn reset_uncertainty_radiuse(&mut self) {
    self.uncertainty_radiuse = None;
  }

  pub fn set_offset_angle(&mut self, offset_angle: f32) {
    self.offset_angle = Some(offset_angle);
  }

  pub fn with_offset_angle(mut self, offset_angle: f32) -> Wgs84Decoded {
    self.offset_angle = Some(offset_angle);
    self
  }

  pub fn offset_angle(&self) -> Option<&f32> {
    self.offset_angle.as_ref()
  }

  pub fn reset_offset_angle(&mut self) {
    self.offset_angle = None;
  }

  pub fn set_included_angle(&mut self, included_angle: f32) {
    self.included_angle = Some(included_angle);
  }

  pub fn with_included_angle(mut self, included_angle: f32) -> Wgs84Decoded {
    self.included_angle = Some(included_angle);
    self
  }

  pub fn included_angle(&self) -> Option<&f32> {
    self.included_angle.as_ref()
  }

  pub fn reset_included_angle(&mut self) {
    self.included_angle = None;
  }

  pub fn set_confidence(&mut self, confidence: f32) {
    self.confidence = Some(confidence);
  }

  pub fn with_confidence(mut self, confidence: f32) -> Wgs84Decoded {
    self.confidence = Some(confidence);
    self
  }

  pub fn confidence(&self) -> Option<&f32> {
    self.confidence.as_ref()
  }

  pub fn reset_confidence(&mut self) {
    self.confidence = None;
  }

}



