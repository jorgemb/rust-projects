FROM rust:bullseye

# Copy all projects
WORKDIR /app
COPY . .

# Build and test
RUN cargo build && cargo test
