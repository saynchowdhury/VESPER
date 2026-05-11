## 2025-05-11 - [CRITICAL] Hardcoded Secrets in Git-Tracked .ENV File
**Vulnerability:** The repository contained a `.ENV` file with live Cloudflare and Groq API keys that was tracked by Git.
**Learning:** Without a root-level `.gitignore` and a clear "Secrets Management" policy, sensitive environment configuration files can easily be accidentally committed, exposing credentials to anyone with access to the source code.
**Prevention:** Always maintain a root-level `.gitignore` that explicitly excludes environment files like `.ENV` and `.env`. Provide a `.ENV.example` template with redacted placeholders to guide setup without risking secret exposure.
