## 2026-04-29 - [CRITICAL] Hardcoded API Keys in tracked .ENV file
**Vulnerability:** Hardcoded Cloudflare and Groq API keys were stored in a `.ENV` file that was tracked by Git.
**Learning:** Even if using a `.ENV` file, if it is not explicitly ignored in `.gitignore`, it will be committed to the repository, exposing secrets.
**Prevention:** Always ensure `.ENV` and similar secret-containing files are added to `.gitignore` from the project's inception. Provide a `.ENV.example` file with placeholders instead.
