# Use the Ubuntu 22.04 as the base image
FROM ubuntu:22.04

# Set environment variables for interactive installation
ENV DEBIAN_FRONTEND=noninteractive

# Install essential packages
RUN apt-get update && apt-get install -y \
    build-essential \
    curl \
    file \
    ruby-full

# Install Rust
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"

# Print version information for Rust
RUN rustc --version && cargo --version

# Set the working directory
WORKDIR /tmp

# Copy your Rust project files into the container
COPY . .

# Build your Rust project
# RUN cargo build

# Set the entry point for the container
CMD ["cargo", "run"]
