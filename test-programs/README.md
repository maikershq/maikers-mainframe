# Test Programs Directory

This directory stores cloned programs for local testing.

Programs are automatically cloned from mainnet when you run `anchor test`.

## Contents

- `mpl_token_metadata.so` - Metaplex Token Metadata program (if manually dumped)

## Automatic vs Manual

**Automatic (Recommended)**: Just run `anchor test` - programs are cloned automatically via Anchor.toml configuration.

**Manual**: Run `./scripts/dump-programs.sh` to dump programs to this directory for offline use.

Configuration is in `../Anchor.toml` under the `[[test.validator.clone]]` section.

