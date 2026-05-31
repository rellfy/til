# syntax=docker/dockerfile:1.7

# Stage 1: build wasm bindings.
FROM rust:1-bookworm AS wasm
RUN cargo install wasm-pack --locked
WORKDIR /src
COPY Cargo.toml Cargo.lock ./
COPY src ./src
COPY crates ./crates
RUN wasm-pack build crates/til-wasm --target web --out-dir /out/wasm

# Stage 2: build the FE bundle.
FROM node:22-alpine AS fe
RUN corepack enable
WORKDIR /app
COPY fe/package.json fe/pnpm-lock.yaml ./
RUN pnpm install --frozen-lockfile
COPY fe ./
COPY --from=wasm /out/wasm ./src/wasm
RUN pnpm build

# Stage 3: serve the static bundle.
FROM nginx:alpine
COPY fe/nginx.conf /etc/nginx/conf.d/default.conf
COPY --from=fe /app/dist /usr/share/nginx/html
EXPOSE 80
