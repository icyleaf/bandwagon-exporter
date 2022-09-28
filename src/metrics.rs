
use prometheus::{self, IntGaugeVec, TextEncoder, Encoder};
use prometheus::{register_int_gauge_vec, opts};
use lazy_static::lazy_static;

use crate::bandwagon::{ServiceInfo, RateLimitStatus};
use crate::configuration::Node;

lazy_static! {
  pub static ref DATA_COUNTER: IntGaugeVec = register_int_gauge_vec!(
    opts!("bandwagon_data_counter", "Data transfer used in the current billing month (bytes)."),
    &["hostname", "ip_address"]
  ).expect("Can't create a metric");

  pub static ref PLAN_MONTHLY_DATA: IntGaugeVec = register_int_gauge_vec!(
    opts!("bandwagon_plan_monthly_data", "Allowed monthly data transfer (bytes)"),
    &["hostname", "ip_address"]
  ).expect("Can't create a metric");

  pub static ref DATA_NEXT_RESET: IntGaugeVec = register_int_gauge_vec!(
    opts!("bandwagon_data_next_reset", "Date and time of transfer counter reset (UNIX timestamp)"),
    &["hostname", "ip_address"]
  ).expect("Can't create a metric");

  pub static ref NODE_INFO: IntGaugeVec = register_int_gauge_vec!(
    opts!("bandwagon_node_info", "Node information"),
    &["hostname", "ip_address", "os", "suspended", "location", "vm_type", "plan", "disk", "ram", "swap"]
  ).expect("Can't create a metric");

  pub static ref RATE_LIMIT_REMAINING_15MIN: IntGaugeVec = register_int_gauge_vec!(
    opts!("bandwagon_api_rate_limit_remaining_points_15min", "API rate limit number of 'points' available to use in the current 15-minutes interval"),
    &["veid", "api_key"]
  ).expect("Can't create a metric");

  pub static ref RATE_LIMIT_REMAINING_24H: IntGaugeVec = register_int_gauge_vec!(
    opts!("bandwagon_api_rate_limit_remaining_points_24h", "API rate limit number of 'points' available to use in the current 24-hour interval"),
    &["veid", "api_key"]
  ).expect("Can't create a metric");
}

#[allow(dead_code)]
pub fn set_data_counter(server_info: &ServiceInfo) {
  DATA_COUNTER
    .with_label_values(&[
      &server_info.hostname, &server_info.ip_address()
    ])
    .set(server_info.data_counter);
}

#[allow(dead_code)]
pub fn set_plan_monthly_data(server_info: &ServiceInfo) {
  PLAN_MONTHLY_DATA
    .with_label_values(&[
      &server_info.hostname, &server_info.ip_address()
    ])
    .set(server_info.plan_monthly_data);
}

#[allow(dead_code)]
pub fn set_data_next_reset(server_info: &ServiceInfo) {
  DATA_NEXT_RESET
    .with_label_values(&[
      &server_info.hostname, &server_info.ip_address()
    ])
    .set(server_info.data_next_reset);
}

#[allow(dead_code)]
pub fn set_api_rate_limit_status(server: &Node, rate_limit_status: &RateLimitStatus) {
  RATE_LIMIT_REMAINING_15MIN
    .with_label_values(&[
      &server.veid, &server.api_key
    ])
    .set(rate_limit_status.remaining_points_15min);

  RATE_LIMIT_REMAINING_24H
    .with_label_values(&[
      &server.veid, &server.api_key
    ])
    .set(rate_limit_status.remaining_points_24h);
}

/// label:
/// - hostname
/// - ip_address
/// - os
/// - suspended
/// - location
/// - vm_type
/// - plan
/// - disk
/// - ram
/// - swap
#[allow(dead_code)]
pub fn set_node_info(server_info: &ServiceInfo) {
  let suspended = if server_info.suspended { "1" } else { "0" };
  NODE_INFO
    .with_label_values(&[
      &server_info.hostname,
      &server_info.ip_address(),
      &server_info.os,
      suspended,
      &server_info.node_location,
      &server_info.vm_type,
      &server_info.plan,
      &server_info.plan_disk.to_string(),
      &server_info.plan_ram.to_string(),
      &server_info.plan_swap.to_string()
    ])
    .set(1);
}

pub fn render_prometheus_text_data() -> String {
  let encoder = TextEncoder::new();
  let mut buffer = vec![];
  encoder.encode(&prometheus::gather(), &mut buffer)
    .expect("Failed to encode metrics");

  let response = String::from_utf8(buffer.clone()).expect("Failed to convert bytes to string");
  buffer.clear();

  response
}
