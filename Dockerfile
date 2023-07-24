# Use an existing Rust image as the base
FROM rust:latest

# Set the working directory
WORKDIR /usr/src/app

# Copy the application files into the image
COPY . .

# Build the application in release mode
RUN cargo build --release

RUN apt-get update && apt-get install -y \
  imagemagick \
  pngquant \
  jpegoptim

# Set the command to run the binary
CMD ["./target/release/something"]