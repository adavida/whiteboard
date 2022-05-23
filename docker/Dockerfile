FROM rust:1.61

RUN cargo install cargo-watch \
    && cargo install --locked trunk\
    && curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh \
    && rustup update \
    && rustup target add wasm32-unknown-unknown
