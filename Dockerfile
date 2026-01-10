FROM rust:latest as builder

WORKDIR /usr/src/myapp
COPY . .

# Build the release binary
RUN cargo build --release

FROM debian:bookworm-slim

# Copy the binary from the builder stage
COPY --from=builder /usr/src/myapp/target/release/rust_db /usr/local/bin/dbms

WORKDIR /app
RUN mkdir data

# Command to run when container starts
CMD ["dbms"]