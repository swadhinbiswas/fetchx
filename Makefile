PREFIX ?= /usr/local
BINDIR ?= $(PREFIX)/bin
CARGO ?= cargo

.PHONY: all build install uninstall clean test help

all: build

build:
	@echo "Building FetchX..."
	$(CARGO) build --release

install: build
	@echo "Installing FetchX to $(BINDIR)..."
	@mkdir -p $(DESTDIR)$(BINDIR)
	@cp -p target/release/fetchx $(DESTDIR)$(BINDIR)/fetchx
	@chmod 755 $(DESTDIR)$(BINDIR)/fetchx
	@echo "FetchX installed successfully!"

uninstall:
	@echo "Uninstalling FetchX..."
	@rm -f $(DESTDIR)$(BINDIR)/fetchx
	@echo "FetchX uninstalled!"

clean:
	@echo "Cleaning build artifacts..."
	$(CARGO) clean
	@echo "Clean complete!"

test:
	@echo "Running tests..."
	$(CARGO) test

help:
	@echo "FetchX Build System"
	@echo ""
	@echo "Available targets:"
	@echo "  build    - Build FetchX in release mode"
	@echo "  install  - Build and install to $(BINDIR)"
	@echo "  uninstall- Remove FetchX from $(BINDIR)"
	@echo "  clean    - Remove build artifacts"
	@echo "  test     - Run tests"
	@echo "  help     - Show this help message"
	@echo ""
	@echo "Variables:"
	@echo "  PREFIX   - Installation prefix (default: /usr/local)"
	@echo "  BINDIR   - Binary directory (default: $(PREFIX)/bin)"
	@echo "  CARGO    - Cargo command (default: cargo)"
	@echo "  DESTDIR  - Destination directory for packaging"
