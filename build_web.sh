#!/bin/bash

set -e

echo "Building Svelte frontend..."

cd web

if [ ! -d "node_modules" ]; then
  echo "Installing dependencies..."
  npm install
fi

echo "Building production bundle..."
npm run build

echo "✓ Frontend built successfully!"
echo "Build output: web/dist/"
