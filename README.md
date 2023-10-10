# SSL (usage of nokey, not for production)
openssl req -x509 -newkey rsa:4096 -keyout key.pem -out cert.pem -days 365 -sha256 -subj "/C=CN/ST=Manuel/L=Serpico/O=Salvi/OU=It/CN=rustapi.lxd"
openssl rsa -in key.pem -out nopass.pem

# Add cargo command to simplify usage
cargo install cargo-swagg

# Add support library to your project (via cargo-edit or manual)
cargo add actix-swagger

# Generate your code with cargo subcommand
cargo swagg ./openapi.yaml --out-file ./src/api.rs

# Format file after
rustfmt ./src/api.rs

# DOCKER BUILD & RUN
docker build -t rust-rpg-user .
docker run -d -p 8080:8080 rust-rpg-user