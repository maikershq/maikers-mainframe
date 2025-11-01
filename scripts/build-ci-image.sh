#!/bin/bash

# Build the CI Docker image locally
# Usage: ./scripts/build-ci-image.sh [tag]

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"
IMAGE_NAME="maikers-mainframe-ci"
TAG="${1:-latest}"

echo "🐳 Building CI Docker image..."
echo "Image: ${IMAGE_NAME}:${TAG}"
echo ""

cd "$PROJECT_ROOT"

docker build \
  -f Dockerfile.ci \
  -t "${IMAGE_NAME}:${TAG}" \
  --progress=plain \
  .

echo ""
echo "✅ Docker image built successfully!"
echo ""
echo "Image: ${IMAGE_NAME}:${TAG}"
echo ""
echo "To test the image:"
echo "  docker run --rm -it ${IMAGE_NAME}:${TAG} bash"
echo ""
echo "To use in CI, tag and push to GitHub Container Registry:"
echo "  docker tag ${IMAGE_NAME}:${TAG} ghcr.io/maikershq/maikers-mainframe/ci-builder:${TAG}"
echo "  docker push ghcr.io/maikershq/maikers-mainframe/ci-builder:${TAG}"

