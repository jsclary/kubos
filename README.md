<img alt="κύβος" src="assets/images/kubos-logo.png?raw=true" width="65%"><br>

[![windows](../../actions/workflows/windows.yml/badge.svg?event=push)](../../actions/workflows/windows.yml)
[![mac](../../actions/workflows/macos.yml/badge.svg?event=push)](../../actions/workflows/macos.yml)
[![linux](../../actions/workflows/linux.yml/badge.svg?event=push)](../../actions/workflows/linux.yml)
[![android](../../actions/workflows/android.yml/badge.svg?event=push)](../../actions/workflows/android.yml)
[![ios](../../actions/workflows/ios.yml/badge.svg?event=push)](../../actions/workflows/ios.yml)

# kubos
Kubos is a rust-based cross-platform game utilizing the Bevy ECS. This project was started primarily a testbed for development of new Bevy plugins and libraries for specialized networking, compression and physics with the hope it will evolve into a full-featured, playable game.

Goals:
- **Distributed Physics**:
    Instead of computing physics using hardware dedicated to a specific region within the world, regardless of whether any players are in that region, physics should be scalable independent of the size of the world. Coallescing load-shifting can be used to minimize duplication of data and network traffic.
- **Volumetric Compression**:
    Mesh and Axis Aligned Block data should utilize advanced compression techniques to minimize level load bandwidth while keeping client-side CPU usage to a minimum.
- **Modern Netowkring Standards**:
    Client communication should utilize HTTP/3 and additional QUIC protocols with efficient parameter encoding.
   
## License
Except where noted below and/or in individual files, all files in this repository are dual-licensed under either:

* MIT License ([LICENSE-MIT](LICENSE-MIT) or [http://opensource.org/licenses/MIT](http://opensource.org/licenses/MIT))
* Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or [http://www.apache.org/licenses/LICENSE-2.0](http://www.apache.org/licenses/LICENSE-2.0))

`SPDX-License-Identifier: MIT OR Apache-2.0`

Explicit alternate licensing for certain files (such as Creative Commons CC0 or BY, but not ND or NC) is allowed so long as the terms are reasonably compatible with the above.

## Your contributions
Contributions are welcome but you must agree to the [Developer Certificate of Origin (DCO)](https://developercertificate.org/) and the above licensing terms.
