## 2025-05-16 - [CRITICAL] Exposed secrets in version control
**Vulnerability:** Real API keys for Groq and Cloudflare were committed in a `.ENV` file, and no `.gitignore` existed to prevent this.
**Learning:** Initial project setup lacked basic security hygiene (gitignore), and the `.ENV` file was explicitly tracked.
**Prevention:** Always use a `.gitignore` to exclude environment files and provide a `.ENV.example` template. Never commit real secrets.
