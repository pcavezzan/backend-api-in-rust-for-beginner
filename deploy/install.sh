#!/usr/bin/env bash
cp ./sytemd.service /etc/systemd/system/backend-api-in-rust-for-beginner.service
# Start now the app
systemctl start backend-api-in-rust-for-beginner
# Make app starts whenever our OS starts
systemctl enable backend-api-in-rust-for-beginner
# Enable nginx site for backend-api-in-rust-for-beginner
cp ./nginx.conf /etc/nginx/sites-available/backend-api-in-rust-for-beginner
# Make nginx read the new configuration for our app to make it available
systemctl reload nginx