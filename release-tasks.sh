#!/usr/bin/env bash
./target/release/diesel migration run
curl -sL https://deb.nodesource.com/setup_10.x | bash -
apt install npm
cd public/
npm run build