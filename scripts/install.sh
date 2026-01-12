#!/bin/bash
set -e

# LCN Installation Script
# This script installs the LCN service on Ubuntu Server

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_DIR="$(dirname "$SCRIPT_DIR")"

echo "=== LCN Installation Script ==="
echo

# Check if running as root
if [[ $EUID -ne 0 ]]; then
   echo "This script must be run as root (use sudo)"
   exit 1
fi

# Check if binaries exist
if [[ ! -f "$PROJECT_DIR/target/release/lcn-server" ]]; then
    echo "Error: lcn-server binary not found."
    echo "Please build the project first:"
    echo "  cargo build --release"
    exit 1
fi

if [[ ! -f "$PROJECT_DIR/target/release/lcn" ]]; then
    echo "Error: lcn binary not found."
    echo "Please build the project first:"
    echo "  cargo build --release"
    exit 1
fi

echo "Installing binaries to /usr/local/bin..."
cp "$PROJECT_DIR/target/release/lcn-server" /usr/local/bin/
cp "$PROJECT_DIR/target/release/lcn" /usr/local/bin/
chmod +x /usr/local/bin/lcn-server
chmod +x /usr/local/bin/lcn

echo "Installing systemd service..."
cp "$PROJECT_DIR/lcn.service" /etc/systemd/system/

echo "Reloading systemd daemon..."
systemctl daemon-reload

echo "Enabling LCN service to start on boot..."
systemctl enable lcn.service

echo "Starting LCN service..."
systemctl start lcn.service

echo
echo "=== Installation Complete ==="
echo
echo "LCN service is now running and will start automatically on boot."
echo
echo "Useful commands:"
echo "  lcn              - Scan network for LCN hosts"
echo "  systemctl status lcn   - Check service status"
echo "  journalctl -u lcn -f   - View service logs"
echo
echo "Test the service:"
echo "  curl http://localhost:7979/hostinfo"
