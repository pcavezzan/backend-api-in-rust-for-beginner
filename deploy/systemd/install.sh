#!/usr/bin/env bash
cp ./sytemd.service /etc/systemd/system/backend-api-in-rust-for-beginner.service
# Start now the app
systemctl start backend-api-in-rust-for-beginner
# Make app starts whenever our OS starts
systemctl enable backend-api-in-rust-for-beginner