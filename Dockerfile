# Build stage
FROM rust:alpine as builder
WORKDIR /usr/src/app

# Install build dependencies
RUN apk add --no-cache musl-dev

# Copy project files
COPY src src
COPY Cargo.* .

# Build release binary
RUN cargo build --release

# Production stage
FROM alpine:latest
WORKDIR /app

RUN apk add --no-cache ca-certificates

# Copy only the binary from builder stage
COPY --from=builder /usr/src/app/target/release/cluster .

# Create a non-root user for security
RUN addgroup -S appgroup && adduser -S appuser -G appgroup \
    && chown -R appuser:appgroup /app

USER appuser

# Set the binary as the entrypoint
ENTRYPOINT ["./cluster"]
