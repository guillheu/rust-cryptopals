FROM rust as test-build

WORKDIR /root/rust-cryptopals

COPY src/ src/
COPY tests/ tests/
COPY Cargo.toml .
COPY README.md .

#Testing
RUN cargo test -- --nocapture

#Building documentation
RUN cargo doc --lib --release --no-deps --target-dir docs
#Building Rust library
# RUN cargo build --release
### ACTUALLY, building is unnecessary since this is a web server container that only needs the web assembly module.
### This could be interesting as part of a build CI if I made a rust crate out of this library.



#Installing wasm-pack to create a wasm module
RUN cargo install wasm-pack
RUN wasm-pack build --target web

FROM nginxinc/nginx-unprivileged:1.20.2-perl

COPY --from=test-build /root/rust-cryptopals/docs /usr/share/nginx/html/docs
COPY --from=test-build /root/rust-cryptopals/pkg /usr/share/nginx/html/pkg
COPY index.html /usr/share/nginx/html

