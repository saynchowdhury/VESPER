## 2026-05-07 - Exposed Secrets in Git and Hardcoded IDs
**Vulnerability:** Sensitive API keys (Cloudflare, Groq) were tracked in a `.ENV` file in the Git repository. Additionally, a Cloudflare Account ID was hardcoded in the backend source code.
**Learning:** Initial project setup or "white-labeling" forks often overlook basic security hygiene like `.gitignore` configuration for environment files, leading to credential leakage.
**Prevention:** Always initialize a repository with a comprehensive `.gitignore` and use environment variables for all infrastructure identifiers and credentials from day one. Use `.ENV.example` to document required variables without leaking values.
