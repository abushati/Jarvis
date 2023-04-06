FROM ubuntu:latest

# # Install dependencies (including the SQLite3 library)
# RUN apt-get update && apt-get install -y \
#     sqlite3 \
#     libsqlite3-dev

# Create a new directory in the container
WORKDIR /usr/local/bin
# Copy the Rust binary into the container
# Copy all Rust binaries from the target directory
COPY target/debug/* .

# Set the command to run when the container starts
CMD ["./server"]