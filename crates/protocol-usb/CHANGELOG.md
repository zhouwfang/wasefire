# Changelog

## 0.2.0-git

### Major

- Implement `wasefire_protocol::Connection` for `Connection` renaming some existing methods

### Minor

- Migrate to removal of `platform::protocol::Api::disable()`
- Add error message when missing udev rule
- Implement `Debug` for `Candidate<T>` and `Connection<T>`
- Implement `Display` for `Connection<T>`

### Patch

- Fail to compile if `device` and `host` features are used together
- Update dependencies
- Remove workaround lint false positive

## 0.1.0

<!-- Increment to skip CHANGELOG.md test: 5 -->
