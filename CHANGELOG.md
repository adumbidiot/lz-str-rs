# Changelog
## [Unreleased]
### Added
- Added `IntoWideIter` impl for `&String`

## [0.2.0] - 2022-10-20
### Added
- Added all functions from lz-string
- Added most of the tests from lz-string
- Added basic Python Binding

### Removed
- Removed `str_to_u32_vec`

### Changed 
- Renamed `compress_uri` to `compress_to_encoded_uri_component`
- Renamed `decompress_uri` to `decompress_from_encoded_uri_component`
- Renamed `compress_str` to `compress`
- Renamed `decompress` to `decompress`
- Renamed `compress` to `compress_internal`
- Renamed `decompress` to `decompress_internal`
- Changed many interfaces to use the `IntoWideIter` instead of u32 slices

### Fixed
- Fix issues with compressing and decompressing long, nonrepeating inputs

## [0.0.1] - 2023-01-23
### Added
- Initial port of lz-string to Rust
- Add `compress`/`decompress` functions
- Add `compress_str`/`compress_str` functions
- Add `compress_uri`/`compress_uri` functions
- Add utility `str_to_u32_vec` function

[0.1.0]: https://github.com/adumbidiot/lz-str-rs/releases/tag/0.1.0