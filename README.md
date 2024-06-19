# simple_server_status

A simple and fast way to get server status on Linux systems.

## Features

All features are enabled by default:

- CPU (`cpu`)
  - `cpu_usage() -> Option<f32>` (0.0..=1.0)
  - `cpu_local_usage() -> Option<f32>` (0.0..=1.0)
  - `cpu_stolen_usage() -> Option<f32>` (0.0..=1.0, useful on VPS's to measure noisy neighbors)
- Network (`net`)
  - `net_bandwidth() -> Option<u64>` (bytes/s)
  - `net_reception_bandwidth() -> Option<u64>` (bytes/s)
  - `net_transmission_bandwidth() -> Option<u64>` (bytes/s)
- RAM (`ram`)
  - `ram_usage() -> Option<f32>` (0.0..=1.0)
  - `ram_swap_usage() -> Option<f32>` (0.0..=1.0)
- TCP (`tcp`)
  - `tcp_connections() -> Option<usize>` (count)
- UDP (`udp`)
  - `udp_sockets() -> Option<usize>` (count)

Note: Must call `update()` first, to make a measurement.

## Limitations

Only supports Linux for now. Will return `Err`, `None`, or `0` on unsupported platforms,
depending on the operation.

## License

Licensed under either of

* Apache License, Version 2.0
  ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
* MIT license
  ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.