# octui Development Makefile

# Colors for output
RED := \033[0;31m
GREEN := \033[0;32m
YELLOW := \033[0;33m
BLUE := \033[0;34m
NC := \033[0m # No Color

VERSION := $(shell grep '^version' Cargo.toml | head -1 | cut -d'"' -f2)

# Default target - show help
.PHONY: help
help:
	@echo "$(BLUE)octui Development Commands$(NC)"
	@echo ""
	@echo "$(GREEN)Development:$(NC)"
	@echo "  make lint            - Format code and run clippy"
	@echo "  make test            - Run tests"
	@echo "  make check           - Run all checks (lint + test)"
	@echo "  make run             - Run octui locally"
	@echo ""
	@echo "$(GREEN)Building:$(NC)"
	@echo "  make build           - Build debug binary"
	@echo "  make build-release   - Build release binary for Apple Silicon"
	@echo "  make build-all       - Build release binaries for all targets"
	@echo ""
	@echo "$(GREEN)Publishing (dry-run):$(NC)"
	@echo "  make test-crates     - Test crates.io publishing (dry-run)"
	@echo ""
	@echo "$(GREEN)Publishing (actual):$(NC)"
	@echo "  make publish-crates  - Actually publish to crates.io"
	@echo ""
	@echo "$(GREEN)Utilities:$(NC)"
	@echo "  make clean           - Clean build artifacts"
	@echo "  make create-archives - Create release archives"
	@echo ""
	@echo "$(YELLOW)Current version: $(VERSION)$(NC)"

# Lint - format and check code
.PHONY: lint
lint:
	@echo "$(BLUE)Formatting code...$(NC)"
	cargo fmt
	@echo "$(BLUE)Running clippy...$(NC)"
	cargo clippy -- -D warnings
	@echo "$(GREEN)✓ Lint passed$(NC)"

# Run tests
.PHONY: test
test:
	@echo "$(BLUE)Running tests...$(NC)"
	cargo test --verbose
	@echo "$(GREEN)✓ Tests passed$(NC)"

# Run all checks (mirrors CI)
.PHONY: check
check: lint test
	@echo "$(BLUE)Checking documentation...$(NC)"
	cargo doc --no-deps --document-private-items
	@echo "$(GREEN)✓ All checks passed$(NC)"

# Run locally
.PHONY: run
run:
	cargo run

# Build debug binary
.PHONY: build
build:
	cargo build

# Build release binary for Apple Silicon
.PHONY: build-release
build-release:
	@echo "$(BLUE)Building for Apple Silicon (aarch64-apple-darwin)...$(NC)"
	cargo build --release --target aarch64-apple-darwin
	@echo "$(GREEN)✓ Built Apple Silicon binary$(NC)"

# Build release binaries for all macOS targets
.PHONY: build-all
build-all:
	@echo "$(BLUE)Building for Apple Silicon...$(NC)"
	cargo build --release --target aarch64-apple-darwin
	@echo "$(BLUE)Building for Intel...$(NC)"
	cargo build --release --target x86_64-apple-darwin
	@echo "$(GREEN)✓ Built all release binaries$(NC)"

# Test crates.io publishing (dry-run)
.PHONY: test-crates
test-crates:
	@echo "$(BLUE)Testing crates.io publishing (dry-run)...$(NC)"
	cargo publish --dry-run
	@echo "$(GREEN)✓ Dry-run passed$(NC)"

# Actually publish to crates.io
.PHONY: publish-crates
publish-crates:
	@echo "$(YELLOW)⚠ This will ACTUALLY publish octui $(VERSION) to crates.io!$(NC)"
	@read -p "Are you sure? (y/N) " -n 1 -r; \
	echo; \
	if [[ $$REPLY =~ ^[Yy]$$ ]]; then \
		cargo publish; \
	else \
		echo "$(YELLOW)Cancelled$(NC)"; \
	fi

# Create release archives
.PHONY: create-archives
create-archives: build-all
	@echo "$(BLUE)Creating release archives...$(NC)"
	@mkdir -p target/release-archives
	@cd target/aarch64-apple-darwin/release && \
	tar czf ../../release-archives/octui-$(VERSION)-aarch64-apple-darwin.tar.gz octui && \
	echo "$(GREEN)✓ Created Apple Silicon archive$(NC)"
	@cd target/x86_64-apple-darwin/release && \
	tar czf ../../release-archives/octui-$(VERSION)-x86_64-apple-darwin.tar.gz octui && \
	echo "$(GREEN)✓ Created Intel archive$(NC)"

# Clean build artifacts
.PHONY: clean
clean:
	@echo "$(BLUE)Cleaning build artifacts...$(NC)"
	cargo clean
	@echo "$(GREEN)✓ Cleaned$(NC)"
