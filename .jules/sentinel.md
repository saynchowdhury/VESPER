## 2024-05-22 - [Exposed Secrets in Repository]
**Vulnerability:** Critical API keys (Cloudflare, Groq) were committed to the repository in a `.ENV` file.
**Learning:** Hardcoded configuration files are a common source of data leaks during rapid prototyping or white-labeling.
**Prevention:** Always use a root-level `.gitignore` to exclude sensitive files like `.ENV` and provide a `.ENV.example` template for developers.
