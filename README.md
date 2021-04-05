# imxrt-ccm

Clock Control Module (CCM) driver for i.MX RT processors.

> :information_source: This experimental driver was not fully integrated into
imxrt-rs projects. It's marked read only while we evaluate its usefulness. Use
the CCM APIs provided by your HAL or RAL to configure clocks.

#### [API Docs (main branch)][main-api-docs]

[main-api-docs]: https://imxrt-rs.github.io/imxrt-ccm/

`imxrt-ccm` is a lower-level Rust driver. It helps you

- configure processor clocks
- associate peripherals with clock gates
- enable and disable peripheral clock gates

`imxrt-ccm` APIs should be re-exported by other i.MX RT packages, like hardware
abstraction layers (HAL). If your HAL re-exports `imxrt-ccm` APIs, your HAL
should provide documentation and examples for using these APIs.

To use `imxrt-ccm` in a larger library, see its API documentation.

### License

Licensed under either of

- [Apache License, Version 2.0](http://www.apache.org/licenses/LICENSE-2.0) ([LICENSE-APACHE](./LICENSE-APACHE))
- [MIT License](http://opensource.org/licenses/MIT) ([LICENSE-MIT](./LICENSE-MIT))

at your option.

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
