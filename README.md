# Bandwagon exporter

A Prometheus Exporter for BandwagonHost KiwiVM. Inspired from [bandwagonhost_cloud_exporter](https://github.com/weiqiang333/bandwagonhost_cloud_exporter) by Go language.

## Usage

Start the binary with `-h` to get the complete syntax. The parameters are:

| Parameter | Required | Valid values | Default | Description |
| -- | -- | -- | -- | -- |
| `CONFIG_PATH` | yes | *.json/yaml/toml | | Config path, see [examples](config/).
| `-m` | no | HOST:PORT | 0.0.0.0:9103 | Specify the serivce with port. This is the target your Prometheus instance should point to.
| `--metrics-path` | no | URI Path | /metrics | This is the path of URI, must starts with `/` char.

Once started, the tool will listen on the specified port (or the default one, 9103, if not specified) and return a Prometheus valid response at the url `/metrics`. So to check if the tool is working properly simply browse the `http://0.0.0.0:9103` (or whichever address with port you choose).

## Deopy

> TODO

## Prometheus Metrics

```text
# HELP bandwagon_api_rate_limit_remaining_points_15min API rate limit number of 'points' available to use in the current 15-minutes interval
# TYPE bandwagon_api_rate_limit_remaining_points_15min gauge
bandwagon_api_rate_limit_remaining_points_15min{veid="1234567"} 900
# HELP bandwagon_api_rate_limit_remaining_points_24h API rate limit number of 'points' available to use in the current 24-hour interval
# TYPE bandwagon_api_rate_limit_remaining_points_24h gauge
bandwagon_api_rate_limit_remaining_points_24h{veid="1234567"} 19900
# HELP bandwagon_api_request_total The total of request bandwagon API since run this CLI
# TYPE bandwagon_api_request_total counter
bandwagon_api_request_total{veid="1234567"} 22
# HELP bandwagon_data_counter Data transfer used in the current billing month (bytes).
# TYPE bandwagon_data_counter gauge
bandwagon_data_counter{hostname="hostname",ip_address="1.2.3.4"} 112214184728
# HELP bandwagon_data_next_reset Date and time of transfer counter reset (UNIX timestamp)
# TYPE bandwagon_data_next_reset gauge
bandwagon_data_next_reset{hostname="hostname",ip_address="1.2.3.4"} 1666319224
# HELP bandwagon_node_info Node information
# TYPE bandwagon_node_info gauge
bandwagon_node_info{disk="21474836480",hostname="hostname",ip_address="1.2.3.4",location="US, California",os="debian-11-x86_64",plan="kvmv3-20g-1g-500g-ca-cn2gia-dc9",ram="1073741824",suspended="0",swap="0",vm_type="kvm"} 1
# HELP bandwagon_plan_monthly_data Allowed monthly data transfer (bytes)
# TYPE bandwagon_plan_monthly_data gauge
bandwagon_plan_monthly_data{hostname="hostname",ip_address="1.2.3.4"} 536870912000
```
