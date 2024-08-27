# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).


## [0.0.3] - 2024-08-26
 
### Added
- Unittests for `eorzean_time` and `eorzean_weather` modules.

### Changed
- Integer timestamps will now be interpreted as seconds since epoch instead of milliseconds.
 
### Fixed
- Incorrect offset used when converting time to EorzeanDate causing for index out of bounds sometimes when reading moon phase prefix
 