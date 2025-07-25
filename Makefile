# -------- Constants & Configuration --------

CARGO_LEPTOS := cargo leptos
DEV_RUSTFLAGS := RUSTFLAGS="--cfg erase_components"
SERVER_NAME := dh-amulet
WEB_PORT := 3000

# -------- Code Formatting --------

.PHONY: fmt
fmt:
	cargo fmt && leptosfmt ./**/*.rs

# -------- SSR Build & Run --------

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

# -------- Webserver Monitoring & Control --------

.PHONY: webserver
webserver:
	@lsof -i :$(WEB_PORT) || echo "✅ No process listening on port $(WEB_PORT)."

.PHONY: kill-webserver
kill-webserver:
	@echo "🔍 Checking for running $(SERVER_NAME) server on port $(WEB_PORT)..."
	@PID=$$(lsof -i :$(WEB_PORT) -sTCP:LISTEN -t -a -c $(SERVER_NAME)); \
	if [ -n "$$PID" ]; then \
		echo "🛑 Found $(SERVER_NAME) (PID: $$PID), stopping it..."; \
		kill $$PID; \
	else \
		echo "✅ No $(SERVER_NAME) server running on port $(WEB_PORT)."; \
	fi
