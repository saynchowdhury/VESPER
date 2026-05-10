## 2024-05-20 - [Hardcoded Secrets and Insecure Configuration]
**Vulnerability:** Live Cloudflare and Groq API keys were hardcoded in a tracked `.ENV` file, and a Cloudflare Account ID placeholder was hardcoded in the source code.
**Learning:** Initial bootstrapping of white-label features sometimes leads to "temporary" hardcoded values and untracked config files that accidentally get committed.
**Prevention:** Always use environment variables for secrets from the start, provide `.ENV.example` templates, and ensure `.gitignore` is correctly configured at the root to ignore all sensitive patterns.
