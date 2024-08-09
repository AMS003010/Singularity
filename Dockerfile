# Use a Debian-based image for broader library support
FROM rust:1.75-buster AS build
ARG APP_NAME=singularity
WORKDIR /app

# Install build dependencies
RUN apt-get update && apt-get install -y \
    clang \
    lld \
    musl-tools \
    gcc \
    libc-dev \
    git \
    && rm -rf /var/lib/apt/lists/*

# Build the application
COPY . .
RUN cargo build --release && \
    cp ./target/release/$APP_NAME /bin/server

# Use a minimal runtime image
FROM debian:buster-slim AS final

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    libssl-dev \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Create a non-privileged user
ARG UID=10001
RUN useradd -u $UID -m appuser
USER appuser

# Copy the executable and assets
COPY --from=build /bin/server /bin/server
COPY singularity.yaml .
COPY src/ src/ 

# Expose the port and set the CMD
EXPOSE 8080
CMD ["/bin/server"]