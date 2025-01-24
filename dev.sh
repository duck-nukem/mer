function start_with_checks() {
  cargo run doctor || exit
  cargo run db migrate
  cargo run watch
}

which cargo-watch > /dev/null || cargo install cargo-watch
docker compose up -d --wait || exit
trap "docker compose down" SIGINT; start_with_checks
