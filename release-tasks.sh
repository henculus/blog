#!/usr/bin/env bash
./target/release/diesel migration run
cd public/
npm run build