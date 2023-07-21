use std::rc::Rc;

use hyper;
use super::configuration::Configuration;

pub struct APIClient<C: hyper::client::Connect> {
  configuration: Rc<Configuration<C>>,
  coordinates_api: Box<dyn (::apis::CoordinatesApi)>,
}

impl<C: hyper::client::Connect> APIClient<C> {
  pub fn new(configuration: Configuration<C>) -> APIClient<C> {
    let rc = Rc::new(configuration);

    APIClient {
      configuration: rc.clone(),
      coordinates_api: Box::new(::apis::CoordinatesApiClient::new(rc.clone())),
    }
  }

  pub fn coordinates_api(&self) -> &dyn (::apis::CoordinatesApi){
    self.coordinates_api.as_ref()
  }


}
