## 2024-05-04 - [Exposed Secrets and Hardcoded Infrastructure IDs]
**Vulnerability:** API keys and infrastructure-specific IDs (Cloudflare Account ID) were tracked in the repository via a `.ENV` file or hardcoded as placeholders in the source code.
**Learning:** Initial white-labeling efforts often use placeholders or local environment files for quick setup, but these can easily be committed to version control, exposing sensitive credentials and leaking internal infrastructure details.
**Prevention:** Always use a root-level `.gitignore` to exclude `.env` files from the start. Provide a `.ENV.example` template with redacted values. Ensure infrastructure IDs are loaded from environment variables rather than hardcoded in URL strings.
