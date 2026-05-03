## 2025-05-03 - [CRITICAL] Leaked API Keys in Version Control
**Vulnerability:** Actual Cloudflare and Groq API keys were committed to the repository in a `.ENV` file.
**Learning:** Even white-labeling projects can accidentally leak upstream or project-specific secrets if environment files are not explicitly ignored from the start.
**Prevention:** Always include `.env`, `.ENV`, and other sensitive extensions in the root `.gitignore` immediately upon project initialization. Provide `.env.example` templates instead.
