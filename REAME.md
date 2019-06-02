spherical-harmonics
========
[![hassle on travis-ci.com](https://travis-ci.com/Jasper-Bekkers/spherical-harmonics.svg?branch=master)](https://travis-ci.com/Jasper-Bekkers/spherical-harmonics)
[![Latest version](https://img.shields.io/crates/v/spherical-harmonics.svg)](https://crates.io/crates/spherical-harmonics)
[![Documentation](https://docs.rs/spherical-harmonics/badge.svg)](https://docs.rs/spherical-harmonics)
[![Lines of code](https://tokei.rs/b1/github/Jasper-Bekkers/spherical-harmonics)](https://github.com/Jasper-Bekkers/spherical-harmonics)
![MIT](https://img.shields.io/badge/license-MIT-blue.svg)

This crate provides a rust native port of the Google [Spherical Harmonics](https://github.com/google/spherical-harmonics) library.

- [Documentation](https://docs.rs/spherical-harmonics)

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
spherical_harmonics = "0.2.0"
```

Then acquire `dxcompiler.dll` directly from [AppVeyor](https://ci.appveyor.com/project/antiagainst/directxshadercompiler/branch/master/artifacts) or compile it from source according to the instructions in the [DirectXShaderCompiler](https://github.com/Microsoft/DirectXShaderCompiler) GitHub repository and make sure it's in the executable enviroment.

DxcValidator also requires `dxil.dll` which can be grabbed from any recent Windows 10 SDK flight.
More info: https://www.wihlidal.com/blog/pipeline/2018-09-16-dxil-signing-post-compile/

## License

Licensed under MIT license ([LICENSE](LICENSE) or http://opensource.org/licenses/MIT)

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in this crate by you, shall be licensed as above, without any additional terms or conditions.

Contributions are always welcome; please look at the [issue tracker](https://github.com/Jasper-Bekkers/spherical-harmonics/issues) to see what known improvements are documented.
