# dbap [![Build Status](https://travis-ci.org/mitchmindtree/dbap.svg?branch=master)](https://travis-ci.org/mitchmindtree/dbap) [![Crates.io](https://img.shields.io/crates/v/dbap.svg)](https://crates.io/crates/dbap) [![Crates.io](https://img.shields.io/crates/l/dbap.svg)](https://github.com/mitchmindtree/dbap/blob/master/LICENSE-MIT) [![docs.rs](https://docs.rs/dbap/badge.svg)](https://docs.rs/dbap/)

An implementation of Distance Based Audio Panning by Trond Lossius et al.

The main types of interest are **Speaker** and **SpeakerGains**.

The **Speaker** type allows the user to describe the position of each speaker
along with a custom "weight".

The **SpeakerGains** type is an iterator that, given a slice of **Speaker**s,
yields the distance based amplitude panning gain for each speaker.

## License

Licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

**Contributions**

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
