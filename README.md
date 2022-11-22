# Cloudera CDP Maven Sync Helper

This tool parses the HTML table on the [Maven Artifacts](https://docs.cloudera.com/cdp-private-cloud-base/7.1.7/runtime-release-notes/topics/xml-maven.html) page for Cloudera CDP 7.1.7 (LTS Release) and generates a Maven POM snippet with a `<dependency>` section for each entry in the table.

This can be used to load all dependencies into a local maven cache.

All paths and CSS Selectors are hardcoded but the code is short and it's easy to change.

Tested with Rust 1.65.0.

To run:
```
cargo run
```
