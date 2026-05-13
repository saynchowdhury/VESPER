## 2024-05-22 - [Sensitive Data Exposure in Version Control]
**Vulnerability:** API keys (Cloudflare, Groq) and Account IDs were hardcoded in code and stored in a tracked `.ENV` file.
**Learning:** Hardcoded strings in API base URLs (like Cloudflare Account IDs) are often overlooked as sensitive data. Versioning `.ENV` files is a common source of critical leaks.
**Prevention:** Always use environment variable lookups for any identifier that is unique to an environment or account. Ensure `.gitignore` explicitly blocks `.ENV` and provide a `.ENV.example` template.
