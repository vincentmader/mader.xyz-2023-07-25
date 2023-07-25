FROM rustlang/rust:nightly as builder

WORKDIR /var/www/mader.xyz

# Pre-compile Cargo dependencies.
# ─────────────────────────────────────────────────────────────────────────────

# Copy Cargo configuration files into container for dependency pre-compilation.
COPY Cargo.toml Cargo.lock ./

# Create dummy source files.
RUN mkdir src;\
    touch src/lib.rs

# Pre-compile Cargo dependencies.
RUN cargo build --release

# Get rid of dummy source files again.
RUN rm src/lib.rs;\
    rmdir src

# Copy actual source files into container.
# ─────────────────────────────────────────────────────────────────────────────

COPY src src
COPY Rocket.toml Rocket.toml
COPY static static
COPY templates templates
# ^ NOTE: "COPY templates ." leads to `Template initialization failed.`

# Start the server.
# ─────────────────────────────────────────────────────────────────────────────

EXPOSE 8000

CMD [ "cargo", "run", "--release" ]
