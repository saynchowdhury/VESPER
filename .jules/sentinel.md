## 2025-05-15 - Exposed Secrets in Repository Root
**Vulnerability:** A `.ENV` file containing active Cloudflare and Groq API keys was committed and tracked in the repository root.
**Learning:** Development environments often lack a root-level `.gitignore` when using subdirectories for different components (frontend/backend), leading to accidental commits of sensitive configuration files.
**Prevention:** Always implement a root-level `.gitignore` that explicitly excludes `.ENV` and other sensitive files, and provide a `.ENV.example` template with redacted placeholders.
