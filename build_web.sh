#!/bin/bash

set -e

echo "Building Svelte + TypeScript frontend..."

cd web

if [ ! -d "node_modules" ]; then
  echo "Installing dependencies..."
  npm install
fi

echo "Type checking..."
npm run check || echo "Warning: Type check had issues, continuing with build..."

echo "Building production bundle..."
npm run build

echo "✓ Frontend built successfully!"
echo "Build output: web/dist/"
