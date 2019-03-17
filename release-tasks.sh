#!/usr/bin/env bash
./target/release/diesel migration run
sudo apt install npm
cd public/
npm run build