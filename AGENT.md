# AGENT.md — Vesper Development Guide (Strict Rules)

**Project**: Vesper — White-labeled native terminal (macOS + Windows)  
**Core Rule**: **NEVER touch any `warpui*` crate, UI rendering code, layouts, or visual elements.**  
This is non-negotiable to avoid uncontrollable dependencies and bugs.

---

## 1. Current State (April 29, 2026)

- Fork of Warp client (AGPL + MIT UI).
- Name changed to **Vesper**.
- Goal: Ship in 6 hours with custom coding process + smart inference routing.
- Inference: Groq (free tier, multi-account) → automatic fallback to Cloudflare Workers AI (paid / zero marginal cost).

---

## 2. What You Must Do (In Order)

### Phase 1 — Branding (Safe, Low Risk)
1. Update `Cargo.toml` (package name, authors, version).
2. Update `about.toml` and `about.hbs` (text only).
3. Replace only the app icons in `resources/` (`icon.icns` + `icon.ico`).
4. Update `README.md` and `FAQ.md` with Vesper branding + new "Inference & Credits" section.
5. Populate `.agents/skills/` with 8–12 JSON files describing the user's exact coding process.

### Phase 2 — Inference Layer (Core Engineering)
6. Create `crates/ai/src/providers/vesper_router.rs` (see implementation below).
7. Register the router in `crates/ai/src/lib.rs` and `crates/ai/src/providers/mod.rs`.
8. Add `groq.rs` and `cloudflare.rs` provider implementations (OpenAI-compatible).
9. Extend settings in `crates/settings*/` to show provider, usage, and upgrade button (append only — no layout changes).
10. Add minimal usage tracking table in `persistence/` (optional but recommended).

### Phase 3 — Build & Ship
11. Run `cargo check --workspace` after every change.
12. Build release binaries for macOS (Intel + Apple Silicon) and Windows.
13. Test inference routing in isolation first.
14. Prepare GitHub Release with signed installers.

---

## 3. Critical Constraints

- **No UI changes whatsoever** — even small ones can break layout or introduce bugs.
- Every new file or change must include this comment:  
  `// Vesper white-label change — safe, no UI impact`
- Use only existing crates (`http_client`, `persistence`, `managed_secrets`, etc.).
- Keep the smart router simple and testable.

---

## 4. Ready-to-Use Implementation Prompts

**Prompt for vesper_router.rs** (already generated — use the file in artifacts):

```
Implement the smart router exactly as specified in Vesper-WhiteLabel-Plan.md and the vesper_router.rs file provided.
Use existing http_client patterns. Make it compile with cargo check.
Add clear comments for every Vesper-specific change.
```

**Prompt for settings**:

```
Append a new "Inference & Credits" section to the Settings window.
Show: Current provider, Daily usage (e.g. 12,450 / 50,000 tokens), Upgrade button.
Do not modify any existing UI layout, colors, or components.
Use only existing settings UI primitives.
```

---

## 5. How to Test

- `cargo check --workspace`
- Run the app: `cargo run`
- In Settings → verify new Inference section appears.
- Test router: Switch between "Vesper Fast" (Groq) and "Vesper Pro" (Cloudflare).
- Simulate limit exhaustion → confirm fallback.

---

## 6. Shipping & Updates (Important)

**This is a native desktop app, not web-based.**

- You build binaries locally or in GitHub Actions.
- Distribute via GitHub Releases (`.dmg`, `.msi`, `.exe`).
- For auto-updates later: Add a simple updater crate (e.g. `self_update` or keep Warp's mechanism).
- Developers get updates by downloading the new release manually for v1.0.

Do **not** treat this like a web app. No "upload to server". Build → Release.

---

**Follow this guide strictly. Ship Vesper in 6 hours.**