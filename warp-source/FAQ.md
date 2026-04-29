<!-- Vesper white-label change — safe, no UI impact -->

# Frequently Asked Questions

This FAQ covers common questions about Vesper, its inference system, and how it builds on the open-source Warp client.

## General

### What is Vesper?

Vesper is a white-labeled, native macOS + Windows terminal built on the open-source Warp client. It delivers a premium agentic coding experience with your exact coding process pre-loaded, powered by intelligent multi-provider inference routing.

### How is Vesper different from Warp?

Vesper adds:
- **Smart inference routing** across multiple providers (Groq + Cloudflare Workers AI).
- **Your custom coding process** baked into the agent from day one.
- **A free-to-paid conversion model** with generous free-tier limits.
- Branded model names ("Vesper Fast", "Vesper Pro") that hide underlying provider details.

### What platforms does Vesper support?

macOS (Intel and Apple Silicon) and Windows.

## Inference & Credits

### How does Vesper's inference work?

Vesper uses a smart multi-provider routing system:

1. **Free tier**: Your requests are routed to Groq's API using a round-robin rotation across multiple accounts. This provides generous daily limits at zero cost.
2. **Automatic fallback**: When all Groq accounts approach their daily limits, Vesper automatically falls back to Cloudflare Workers AI.
3. **Paid tier**: Paid users always use Cloudflare Workers AI for consistent, unlimited access.

### What are the model names?

| Friendly Name | Real Provider | Real Model |
|---------------|--------------|------------|
| Vesper Fast | Groq | llama-3.1-70b-versatile |
| Vesper Pro | Cloudflare Workers AI | llama-3.1-405b-instruct |

### What are the free tier limits?

Free tier provides approximately 50,000 tokens/day or 100 requests/day (configurable). When limits are reached, you'll be prompted to upgrade.

### How do I upgrade to paid?

When your free tier is exhausted, Vesper shows an upgrade prompt with a link to the payment page. After payment, your routing automatically switches to Cloudflare Workers AI for unlimited access.

### Is my data sent to third parties?

Your prompts are sent to the inference provider (Groq or Cloudflare Workers AI) for processing. Vesper does not store or use your data for model training.

## Building & Contributing

### How do I build Vesper from source?

```bash
./script/bootstrap   # platform-specific setup
cargo run            # build and run Vesper
cargo check          # verify compilation
```

### Can I use my own coding agent?

Yes. Vesper supports Warp's built-in agent as well as external CLI agents (Claude Code, Codex, Gemini CLI, and others). Agent-readable skills are located under `.agents/skills/`.

## Licensing

### What license is Vesper under?

Vesper's UI framework (`warpui_core` and `warpui` crates) are licensed under [MIT](LICENSE-MIT). The rest of the code is licensed under [AGPL v3](LICENSE-AGPL).

### Can someone fork Vesper?

Yes — that's what AGPL is for. Open derivatives are welcome; fully-proprietary relaunches are not permitted under the license.

## Help

### Where do I get help?

- Check this FAQ and the [README](README.md).
- Open a [GitHub Issue](https://github.com/vesper-dev/vesper/issues) for bug reports and feature requests.

### How do I report a security vulnerability?

Please don't open a public GitHub issue. See [SECURITY.md](SECURITY.md) for private reporting instructions.
