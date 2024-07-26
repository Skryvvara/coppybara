all: build

.PHONY: build
build: clean
	@cargo build --release

.PHONY: vendor
vendor:
	@cargo vendor

.PHONY: clean
clean:
	@cargo clean
