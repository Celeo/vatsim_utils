default: verify

verify:
  @cargo t
  @cargo +nightly clippy

doc-build:
  @cargo doc --no-deps

doc-view:
  @python3 -m http.server --directory target/doc 5000
