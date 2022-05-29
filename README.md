# simple_server_status

A simple and fast way to get server status on Linux systems.

## Features

All features are enabled by default:

- CPU (`cpu`)
  - `cpu_usage() -> Option<f32>`
  - `cpu_local_usage() -> Option<f32>`
  - `cpu_stolen_usage() -> Option<f32>` (useful on VPS's to measure noisy neighbors)
- RAM (`ram`)
  - `ram_usage() -> Option<f32>`
  - `ram_swap_usage() -> Option<f32>`
- TCP (`tcp`)
  - `tcp_connections() -> Option<usize>`

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