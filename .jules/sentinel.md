## 2024-05-06 - [CRITICAL] Leaked API Keys in Git History
**Vulnerability:** Sensitive API keys for Cloudflare and Groq were committed to the repository in a `.ENV` file.
**Learning:** Initial bootstrapping or white-labeling processes may accidentally include sensitive configuration files in the base repository state if a root `.gitignore` is missing.
**Prevention:** Always establish a root-level `.gitignore` that excludes environment files (`.ENV`, `.env`) and provide a redacted `.ENV.example` template at the very start of a project.
