echo 'Building new binary'
cargo build

echo 'Running new build as super user'
sudo ./target/debug/archbookd