echo 'stopping joe.archbookd.service'
sudo systemctl stop joe.archbookd.service

echo 'Removing old archbookd binary'
sudo rm -rf /usr/local/bin/archbookd

echo 'Building new binary'
cargo build

echo 'Moving new binary into /usr/local/bin/archbookd'
sudo cp target/debug/archbookd /usr/local/bin/archbookd

echo 'Enabling joe.archbookd.service'
sudo systemctl enable joe.archbookd.service

echo 'Starting joe.archbookd.service'
sudo systemctl start joe.archbookd.service