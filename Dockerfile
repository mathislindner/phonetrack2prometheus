# Use the official Rust image from the Docker Hub
FROM rust:latest

# Set the working directory in the container
WORKDIR /app

# Copy the Cargo.toml and Cargo.lock files into the container
COPY phonetrackrust/Cargo.toml phonetrackrust/Cargo.lock ./

# Copy the source code into the container
COPY phonetrackrust/src ./src

# Build the application
RUN cargo build --release

# Copy the environment file into the container
COPY .env .env

# Expose the port the app runs on
EXPOSE 5000

# Run the application
CMD ["./target/release/phonetrackrust"]