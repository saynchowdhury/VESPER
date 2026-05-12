## 2026-04-29 - [Exposed Secrets in .ENV]
**Vulnerability:** A `.ENV` file containing live API keys for Cloudflare and Groq was tracked by Git and pushed to the repository. Additionally, a Cloudflare Account ID was hardcoded in the backend.
**Learning:** Initial project setup for white-label forks can sometimes involve committing development environment files that should be ignored. Relying on hardcoded placeholders for environment-specific URLs is also a common pitfall.
**Prevention:** Always include a root-level `.gitignore` that excludes `.ENV` and other secret files from the very first commit. Use `.ENV.example` templates and dynamic environment variable lookups in the backend code.
