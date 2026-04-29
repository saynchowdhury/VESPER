<!-- Vesper white-label change — safe, no UI impact -->

# Vesper

**Your process. Elevated.**

Vesper is a premium native terminal for macOS and Windows, built on the open-source Warp client. It delivers an agentic coding experience with your exact coding process pre-loaded, powered by intelligent multi-provider inference routing.

## Key Features

- **Smart Inference Routing** — Automatic model selection across Groq and Cloudflare Workers AI for optimal speed and cost.
- **Your Coding Process Built-In** — Custom agent skills that encode your exact development workflow from day one.
- **Generous Free Tier** — Multi-account Groq rotation provides substantial free usage with automatic fallback.
- **Premium Upgrade Path** — Seamless transition to Cloudflare Workers AI for unlimited, low-cost inference.

## Inference & Credits

Vesper uses a smart multi-provider routing system:

| Tier | Provider | Model Names | Limits |
|------|----------|-------------|--------|
| Free | Groq (multi-account rotation) | Vesper Fast | 50,000 tokens/day |
| Paid | Cloudflare Workers AI | Vesper Pro | Unlimited |

When free tier limits are reached, Vesper automatically falls back to Cloudflare Workers AI. Upgrade at any time for unlimited access.

## Installation

Download the latest release from the [Releases](https://github.com/vesper-dev/vesper/releases) page.

- **macOS**: Download `.dmg` (Intel or Apple Silicon)
- **Windows**: Download `.msi` or `.exe`

## Building from Source

```bash
./script/bootstrap   # platform-specific setup
cargo run            # build and run Vesper
cargo check          # verify compilation
```

## Licensing

Vesper's UI framework (the `warpui_core` and `warpui` crates) are licensed under the [MIT license](LICENSE-MIT).

The rest of the code in this repository is licensed under the [AGPL v3](LICENSE-AGPL).

## Open Source Foundation

Vesper is built on top of the excellent [Warp](https://github.com/warpdotdev/Warp) open-source terminal. We are grateful to the Warp team for making their client codebase available.

## Open Source Dependencies

We'd like to call out a few of the [open source dependencies](https://docs.warp.dev/help/licenses) that have helped get off the ground:

* [Tokio](https://github.com/tokio-rs/tokio)
* [NuShell](https://github.com/nushell/nushell)
* [Fig Completion Specs](https://github.com/withfig/autocomplete)
* [Alacritty](https://github.com/alacritty/alacritty)
* [Hyper HTTP library](https://github.com/hyperium/hyper)
* [FontKit](https://github.com/servo/font-kit)
* [Core-foundation](https://github.com/servo/core-foundation-rs)
* [Smol](https://github.com/smol-rs/smol)
