#!/usr/bin/env bash
./target/release/diesel migration run
apt install npm
cd public/
npm run build