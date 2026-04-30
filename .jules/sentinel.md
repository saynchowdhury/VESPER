## 2024-05-24 - [CRITICAL] Sensitive Configuration and Hardcoded Placeholders
**Vulnerability:** The `.ENV` file containing active Cloudflare and Groq API keys was tracked by Git. Additionally, the Cloudflare Account ID was hardcoded as a placeholder string "VESPER_CF_ID" in the Rust backend code, which would lead to runtime failures if not manually replaced, and encourages insecure practices.
**Learning:** Initial white-labeling efforts often result in "quick and dirty" configuration management that can accidentally expose secrets if standard `.gitignore` practices aren't followed from the start.
**Prevention:** Always initialize a `.gitignore` that excludes environment files and use environment variables for all infrastructure-specific identifiers.
