.PHONY: check fmt clippy test test-all audit deny bench coverage build doc clean

# Run all CI checks locally
check: fmt clippy doc test deny audit

# Format check
fmt:
	cargo fmt --all -- --check

# Lint (zero warnings)
clippy:
	cargo clippy --all-features --all-targets -- -D warnings

# Run test suite (all features)
test:
	cargo test --all-features

# Full feature matrix test — every feature combo that matters
test-all: test
	@echo "── no_std (no default features) ──"
	cargo check --no-default-features
	@echo "── std only ──"
	cargo test
	@echo "── hoosh ──"
	cargo test --features hoosh
	@echo "── mcp ──"
	cargo test --features mcp
	@echo "── daimon ──"
	cargo test --features daimon
	@echo "── hoosh-llm ──"
	cargo test --features hoosh-llm
	@echo "── full ──"
	cargo test --features full
	@echo "✓ All feature combos pass"

# Security audit
audit:
	cargo audit

# Supply-chain checks (cargo-deny)
deny:
	cargo deny check

# Run benchmarks with history
bench:
	./scripts/bench-history.sh

# Generate coverage report
coverage:
	cargo llvm-cov --all-features --html --output-dir coverage/
	@echo "Coverage report: coverage/html/index.html"

# Build release
build:
	cargo build --release --all-features

# Generate documentation (warnings = errors)
doc:
	RUSTDOCFLAGS="-D warnings" cargo doc --no-deps --all-features

# Clean build artifacts
clean:
	cargo clean
	rm -rf coverage/
