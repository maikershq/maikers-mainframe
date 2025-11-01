# Docker CI Image for Maikers Mainframe

This directory contains Docker configuration for building a containerized CI/CD environment with all required tools pre-installed.

## 📦 What's Included

The CI Docker image (`Dockerfile.ci`) includes:

- **Rust** 1.88.0 with rustfmt, clippy
- **Solana CLI** 1.18.26
- **Anchor Framework** 0.31.1 (via AVM)
- **Node.js** 20.x with Yarn
- **Build tools**: cargo-audit, git, pkg-config, etc.

## 🚀 Quick Start

### Building Locally

```bash
# Build the image
./scripts/build-ci-image.sh

# Or with a custom tag
./scripts/build-ci-image.sh v1.0.0

# Test the image
docker run --rm -it maikers-mainframe-ci:latest bash
```

### Using in GitHub Actions

The repository includes two CI workflow options:

1. **Standard CI** (`.github/workflows/ci.yml`) - Installs tools on each run
2. **Docker CI** (`.github/workflows/ci-docker.yml`) - Uses pre-built Docker image

To use the Docker-based CI:

```yaml
jobs:
  build:
    runs-on: ubuntu-latest
    container:
      image: ghcr.io/${{ github.repository }}/ci-builder:latest
      credentials:
        username: ${{ github.actor }}
        password: ${{ secrets.GITHUB_TOKEN }}
    steps:
      - uses: actions/checkout@v4
      - run: anchor build
```

## 🔄 Building and Publishing the Image

### Automatic Build (Recommended)

The image is automatically built and pushed to GitHub Container Registry when:
- `Dockerfile.ci` is modified
- `.github/workflows/docker-build.yml` is modified
- Manually triggered via GitHub Actions UI

### Manual Build and Push

```bash
# 1. Build the image
./scripts/build-ci-image.sh

# 2. Tag for GitHub Container Registry
docker tag maikers-mainframe-ci:latest \
  ghcr.io/maikershq/maikers-mainframe/ci-builder:latest

# 3. Login to GitHub Container Registry
echo $GITHUB_TOKEN | docker login ghcr.io -u USERNAME --password-stdin

# 4. Push to registry
docker push ghcr.io/maikershq/maikers-mainframe/ci-builder:latest
```

## 🔧 Customizing the Image

### Updating Tool Versions

Edit `Dockerfile.ci` and update the environment variables:

```dockerfile
ENV RUST_VERSION=1.88.0
ENV SOLANA_VERSION=1.18.26
ENV ANCHOR_VERSION=0.31.1
ENV NODE_VERSION=20
```

### Adding New Tools

Add installation commands in the Dockerfile:

```dockerfile
# Install additional tools
RUN cargo install cargo-expand
RUN npm install -g some-tool
```

## 📊 Benefits

### Speed
- ⚡ **60-80% faster CI builds** - No tool installation overhead
- 🚀 **Parallel job execution** - All tools pre-installed

### Reliability
- ✅ **Consistent environment** - Same image across all jobs
- 🔒 **Version locked** - No unexpected tool updates
- 🛡️ **Reproducible builds** - Exact same tools every time

### Cost
- 💰 **Reduced CI minutes** - Faster builds = lower costs
- 📉 **Better resource usage** - Cached layers = less download

## 🧪 Testing Locally

Run the full build locally using the Docker image:

```bash
# Build the image
./scripts/build-ci-image.sh

# Run build in container
docker run --rm -v $(pwd):/workspace \
  maikers-mainframe-ci:latest \
  bash -c "cd /workspace && yarn install && anchor build"

# Run tests
docker run --rm -v $(pwd):/workspace \
  maikers-mainframe-ci:latest \
  bash -c "cd /workspace && yarn install && yarn test"
```

## 📝 Image Sizes

- **Uncompressed**: ~2.5 GB
- **Compressed**: ~900 MB
- **Pull time**: ~2-3 minutes (first time), then cached

## 🔐 Security

The Docker image:
- Uses official Ubuntu 22.04 base
- Installs tools from official sources only
- Runs as root (required for CI containers)
- Published to GitHub Container Registry (private by default)

## 🆘 Troubleshooting

### Image Pull Fails

```bash
# Ensure you're authenticated
echo $GITHUB_TOKEN | docker login ghcr.io -u USERNAME --password-stdin
```

### Build Fails

```bash
# Clear Docker cache
docker system prune -af

# Rebuild without cache
docker build --no-cache -f Dockerfile.ci -t maikers-mainframe-ci:latest .
```

### Tools Not Found in CI

Ensure the workflow uses the correct image:

```yaml
container:
  image: ghcr.io/${{ github.repository }}/ci-builder:latest
```

## 📚 References

- [Docker in GitHub Actions](https://docs.github.com/en/actions/using-containerized-services)
- [GitHub Container Registry](https://docs.github.com/en/packages/working-with-a-github-packages-registry/working-with-the-container-registry)
- [Solana CLI](https://docs.solana.com/cli/install-solana-cli-tools)
- [Anchor Framework](https://www.anchor-lang.com/docs/installation)

