# Use a lightweight Linux distribution as the base image
FROM debian:buster-slim

# Install the necessary dependencies
RUN apt-get update && apt-get install -y nginx

# Copy the Wasm binary to the appropriate location
COPY ./target/wasm32-unknown-unknown/release/your_app_name.wasm /usr/share/nginx/html/

# Expose port 80 for the nginx server
EXPOSE 80

# Start the nginx server
CMD ["nginx", "-g", "daemon off;"]
