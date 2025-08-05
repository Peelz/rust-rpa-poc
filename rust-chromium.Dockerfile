# ---- Builder Stage ----
ARG SERVICE_NAME
FROM rust:1.88 AS builder

# Set working directory
WORKDIR /usr/src/app

# Create a new empty project to cache dependencies.
# This is faster than copying the whole project.
RUN USER=root cargo new --bin SERVICE_NAME
WORKDIR /usr/src/app/

# Copy the dependencies manifest files
COPY ./ ./

# Build dependencies. This will be cached if the manifests don't change.
RUN cargo build --release

# Build the application
RUN rm ./target/release/deps/${SERVICE_NAME}* # remove dummy build artifacts
RUN cargo build --release

# ---- Final Stage ----
FROM debian:12-slim

# Set working directory
WORKDIR /usr/local/bin

RUN apt update && apt install chromium -y

# Copy the built binary from the builder stage
COPY --from=builder /usr/src/app/session_man/target/release/session_man .

# Set the entrypoint
CMD ["./${SERVICE_NAME}"]
