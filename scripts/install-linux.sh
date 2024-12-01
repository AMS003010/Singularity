#!/bin/bash

# Update and upgrade system packages
echo "Updating package lists..."
sudo apt update

echo "Upgrading installed packages..."
sudo apt upgrade -y

# Install development tools and essential libraries
echo "Installing build-essential package..."
sudo apt install -y build-essential

echo "Installing GCC and CMake..."
sudo apt install -y gcc cmake

echo "Installing SSL and package config libraries..."
sudo apt install -y libssl-dev pkg-config

echo "Installing D-Bus development library..."
sudo apt install -y libdbus-1-dev

# Install Rust's Cargo if not already installed
if ! command -v cargo &> /dev/null; then
  echo "Cargo is not installed. Installing Rust and Cargo..."
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
  source $HOME/.cargo/env
else
  echo "Cargo is already installed. Skipping Rust installation..."
fi

# Install Cargo packages
echo "Installing Cargo dependencies..."
cargo install --path .

echo "Singularity completed successfully."