# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [1.0.0] - 2022-05-26
## Added
- Derive attribute to override the root path message.

## [0.4.0] - 2021-05-08
## Added
- Derive attributes to avoid the generation of GET and PUT methods.
- Support for `RwLock`.

## [0.3.0] - 2021-02-28
### Added
- Derive attribute to skip configuration fields.
- On set callbacks.
- Custom field validators.

## [0.2.0] - 2021-02-18
### Added
- Option to enable json content type for all requests.

### Fixed
- All generated methods are now `pub`.

## [0.1.1] - 2021-02-11
### Fixed
- Re-export of libraries so users don't have to include them in their `Cargo.toml`.

## [0.1.0] - 2021-02-11
### Added
- First version.
