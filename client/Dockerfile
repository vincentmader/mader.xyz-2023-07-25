FROM rustlang/rust:nightly as builder

WORKDIR /var/www/mader.xyz

# Install dependencies needed for yew.rs compilation.
# ─────────────────────────────────────────────────────────────────────────────

RUN rustup target add wasm32-unknown-unknown
RUN cargo install --locked wasm-bindgen-cli
RUN cargo install --locked trunk

# Build yew.rs client crate.
# ─────────────────────────────────────────────────────────────────────────────

COPY Cargo.toml Cargo.lock ./
COPY src src
COPY index.html index.html

RUN trunk build --release

# Start the server.
# ─────────────────────────────────────────────────────────────────────────────

EXPOSE 8080

CMD [ "trunk", "serve", "--release", "--address", "0.0.0.0" ]
