## 2025-05-16 - [Exposed Secrets in Version Control]
**Vulnerability:** Sensitive API keys (Cloudflare, Groq) were committed to the repository in a `.ENV` file without a `.gitignore` to protect them.
**Learning:** Initial project setup for white-labeling often involves rapid iteration where environment files might be created and accidentally tracked before a proper `.gitignore` is established.
**Prevention:** Always initialize a repository with a comprehensive `.gitignore` that includes common environment file patterns (`.env`, `.ENV`, `*.local`) before adding any configuration.
