# Stage 1: Build the Rust application
FROM rust:latest as rust-setup

# Set the working directory in the container
WORKDIR /usr/src/app

# Copy the Rust source code
COPY ./backend .

# Compile the Rust application
RUN cargo build --release

# Stage 2: Set up the Debian Slim environment with Java
FROM ubuntu:latest

# Install Java
# RUN apt-get update && apt-get install -y openjdk-11-jre-headless && rm -rf /var/lib/apt/lists/*
# Install OpenJDK 17
RUN apt-get update && \
    apt-get install -y software-properties-common && \
    add-apt-repository -y ppa:openjdk-r/ppa && \
    apt-get update && \
    apt-get install -y openjdk-17-jdk && \
    rm -rf /var/lib/apt/lists/*

# Verify the installation
RUN java -version

# Copy the compiled Rust binary from the builder stage
COPY --from=rust-setup /usr/src/app/target/release/backend /usr/src/server/backend

# Copy other necessary files
COPY server.jar /usr/src/server/server.jar
COPY eula.txt /usr/src/server/eula.txt
COPY server.properties.template /usr/src/server/server.properties.template

# Set the working directory
WORKDIR /usr/src/server

# Command to run the Rust application
CMD ["./backend"]

# Expose necessary ports
EXPOSE 8080 25565

