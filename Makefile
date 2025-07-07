# -------- variables --------

CARGO_LEPTOS=cargo leptos

# use this flags when developing for faster compile
DEV_RUSTFLAGS=RUSTFLAGS="--cfg erase_components"

# -------- Leptos fmt --------

.PHONY: fmt
fmt:
	cargo fmt && leptosfmt ./**/*.rs

# -------- SSR dh-amulets

.PHONY: dev-dh-amulets
dev-dh-amulets:
	$(DEV_RUSTFLAGS) $(CARGO_LEPTOS) watch

.PHONY: build-dh-amulets
build-dh-amulets:
	$(CARGO_LEPTOS) build --release

.PHONY: run-dh-amulets
run-dh-amulets:
	$(CARGO_LEPTOS) serve --release

# -------- Cleanup --------

.PHONY: clean
clean:
	cargo clean
