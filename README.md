SSL

openssl req -x509 -newkey rsa:4096 -keyout key.pem -out cert.pem -days 365 -sha256 -subj "/C=CN/ST=Manuel/L=Serpico/O=Salvi/OU=It/CN=rustapi.lxd"
openssl rsa -in key.pem -out nopass.pem



DOCKER BUILD & RUN

docker build -t rust-rpg-user .
docker run -d -p 8080:8080 rust-rpg-user