# Use the official Rust image as the base image
FROM rust:latest

# Set the working directory inside the container
WORKDIR /usr/src/myapp

# Install dependencies
RUN apt-get update && \
    apt-get install -y \
    libssl-dev libfl-dev libpq-dev pkg-config postgresql-client \
    && rm -rf /var/lib/apt/lists/*

# Copy everything
COPY . .

# Build the Rust application
RUN cargo build --release

# Run the application
CMD [ "./target/release/buddy_chat" ]


