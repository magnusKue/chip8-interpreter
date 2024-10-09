#!/bin/bash

# Specify the directory containing the applications
APP_DIR="../../roms"

# Loop through each file in the directory
for app in "$APP_DIR"/*; do
	cargo run -- "$app" &
done
