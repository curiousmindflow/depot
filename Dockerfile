FROM node:22-alpine AS frontend
WORKDIR /app
COPY depot/package.json depot/package-lock.json ./
RUN npm ci
COPY depot/ .
RUN npm run build

FROM rust:1.93-slim AS backend
WORKDIR /app
COPY Cargo.toml Cargo.lock ./
COPY src/ src/
COPY --from=frontend /app/dist depot/dist
RUN cargo build --release

FROM debian:bookworm-slim
WORKDIR /app
COPY --from=frontend /app/dist ./static
COPY --from=backend /app/target/release/depot ./server
ENV DIST_PATH=/app/static
EXPOSE 3000
CMD ["./server"]