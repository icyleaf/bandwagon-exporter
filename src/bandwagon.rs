use bytes::Bytes;
use http_body_util::{BodyExt, Full};
use hyper_util::client::legacy::connect::HttpConnector;
use hyper_rustls::HttpsConnector;
use hyper_util::client::legacy::Client;
use hyper_util::rt::TokioExecutor;
use serde::Deserialize;

type HttpClient = Client<HttpsConnector<HttpConnector>, Full<Bytes>>;

#[derive(Clone)]
pub struct Kiwivm {
  pub endpoint: String,
  client: HttpClient,
}

#[derive(Deserialize, Debug)]
pub struct ServiceInfo {
  pub vm_type: String,                                // Hypervizor type (ovz or kvm)
  pub hostname: String,                               // Hostname of the VPS
  pub node_alias: String,                             // Internal nickname of the physical node
  pub node_location: String,                          // Physical location (country, state)
  pub node_location_id: String,                       // Id value of Physical location
  pub location_ipv6_ready: bool,                      // Whether IPv6 is supported at the current location
  pub plan: String,                                   // Name of plan
  pub plan_monthly_data: i64,                         // Allowed monthly data transfer (bytes). Needs to be multiplied by monthly_data_multiplier - see below.
  pub monthly_data_multiplier: i64,                   // Some locations offer more expensive bandwidth; this variable contains the bandwidth accounting coefficient.
  pub plan_disk: i64,                                 // Disk quota (bytes)
  pub plan_ram: i64,                                  // RAM (bytes)
  pub plan_swap: i64,                                 // SWAP (bytes)
  pub os: String,                                     // Operating system
  pub email: String,                                  // Primary e-mail address of the account
  pub data_counter: i64,                              // Data transfer used in the current billing month. Needs to be multiplied by monthly_data_multiplier - see below.
  pub data_next_reset: i64,                           // Date and time of transfer counter reset (UNIX timestamp)
  pub ip_addresses: Vec<String>,                      // IPv4 and IPv6 addresses assigned to VPS (Array)
  pub private_ip_addresses: Vec<String>,              // Private IPv4 addresses assigned to VPS (Array)
  // pub plan_private_network_available: bool,           //
  // pub location_private_network_available: bool,       //
  // pub rdns_api_available: bool,                       //
  // pub available_isos: Vec<String>,                 //
  pub suspended: bool,                                //
  pub policy_violation: bool,                         // Whether VPS is suspended
  pub suspension_count: Option<i64>,                  // Number of times service was suspended in current calendar year
  pub total_abuse_points: i64,                        // Total abuse points accumulated in current calendar year
  pub max_abuse_points: i64,                          // Maximum abuse points allowed by plan in a calendar year

  pub error: i64,                                      // Error code
  pub message: Option<String>,                        // Error message
}

#[derive(Deserialize, Debug)]
pub struct RateLimitStatus {
  pub remaining_points_15min: i64,                    // Number of "points" available to use in the current 15-minute interval
  pub remaining_points_24h: i64,                      // Number of "points" available to use in the current 24-hour interval
}

impl ServiceInfo {
  #[allow(dead_code)]
  pub fn ip_address(&self) -> String {
    self.ip_addresses
      .get(0)
      .unwrap()
      .clone()
  }
}

impl Kiwivm {
  pub fn new(endpoint: String) -> Kiwivm {
    let https = hyper_rustls::HttpsConnectorBuilder::new()
      .with_native_roots()
      .expect("Failed to load native root certificates")
      .https_only()
      .enable_http1()
      .build();

    Kiwivm {
      endpoint,
      client: Client::builder(TokioExecutor::new()).build(https)
    }
  }

  /// Allows to forcibly stop a VPS that is stuck and cannot be stopped by normal means.
  ///
  /// Please use this feature with great care as any unsaved data will be lost.
  #[allow(dead_code)]
  pub async fn get_service_info(&self,
    veid: &String, api_key: &String
  ) -> Result<ServiceInfo, Box<dyn std::error::Error + Send + Sync>> {
    let url = format!("{}/getServiceInfo?veid={}&api_key={}", self.endpoint, *veid, *api_key).parse()?;
    let res = self.client.get(url).await?;
    let body_bytes = res.into_body().collect().await?.to_bytes();
    let service_info = serde_json::from_slice(&body_bytes)?;

    Ok(service_info)
  }

  /// When you perform too many API calls in a short amount of time, KiwiVM API may start dropping your requests for a few minutes.
  ///
  /// This call allows monitoring this matter.
  #[allow(dead_code)]
  pub async fn get_rate_limit_status(&self,
    veid: &String, api_key: &String
  ) -> Result<RateLimitStatus, Box<dyn std::error::Error + Send + Sync>> {
    let url = format!("{}/getRateLimitStatus?veid={}&api_key={}", self.endpoint, veid, api_key).parse()?;
    let res = self.client.get(url).await?;
    let body_bytes = res.into_body().collect().await?.to_bytes();
    let rate_limit_status = serde_json::from_reader(&body_bytes[..])?;

    Ok(rate_limit_status)
  }
}
