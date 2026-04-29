# Vesper — White Label Plan (v1.0)

**Product Name**: Vesper  
**Tagline**: "Your process. Elevated."  
**Date**: April 29, 2026  
**Timeline**: 6-hour ship window  
**Core Constraint**: **ZERO changes to any `warpui*` crates or UI layer**. Keep Vesper and Warp UI completely untouched to eliminate uncontrollable dependencies and bugs.

---

## 1. Project Overview

Vesper is a white-labeled, native macOS + Windows terminal built on the open-source Warp client.  
It delivers a premium agentic coding experience with **your exact coding process** pre-loaded, powered by intelligent multi-provider inference routing (Groq + Cloudflare Workers AI) and a smart free-tier → paid conversion model.

**Key Differentiators**:
- Your custom coding process baked into the agent from day one.
- Zero marginal inference cost after free tier (smart routing + your Cloudflare Workers).
- Multiple inference accounts with automatic smart delegation.
- Clean, professional branding that feels native and premium.

---

## 2. Non-Negotiable Constraints (Read This First)

- **DO NOT TOUCH** any file under `crates/warpui*`, `crates/ui_components`, or any UI rendering code.
- **DO NOT** modify layouts, themes, colors, icons (except app icon in `resources/`), or any visual elements that could introduce layout bugs.
- All changes must be **backend / settings / inference layer only**.
- Any new code must compile cleanly with existing `cargo check` and pass existing tests where possible.
- Documentation must be explicit so a new developer (or coding agent) can implement without introducing regressions.

---

## 3. Codebase Structure — Relevant Parts Only

```
vesper/  (fork of warp)
├── Cargo.toml
├── about.toml                  # ← CHANGE: name, author, description
├── about.hbs                   # ← CHANGE: about dialog text only
├── resources/                  # ← REPLACE: app icon (icns + ico) only — NO other assets
├── .agents/skills/             # ← POPULATE: your custom coding process skills
├── crates/
│   ├── ai/                     # ★ PRIMARY FOCUS — new providers + smart router
│   ├── settings*/              # ★ Add inference provider settings + credit display
│   ├── http_client/            # Re-use for all API calls
│   ├── persistence/            # Add usage tracking table (optional but recommended)
│   └── integration/            # Keep as-is for external CLI agents
├── script/                     # Use for builds (minor branding strings only)
└── README.md + FAQ.md          # Update for Vesper branding + "How inference works"
```

**Total safe change surface**: ~12–15 files.  
**UI surface touched**: Zero.

---

## 4. White-Label Changes — Exact & Minimal

### Phase 1 (First 2 hours — Branding & Foundation)

| File                          | Change                                                                 | Risk Level |
|-------------------------------|------------------------------------------------------------------------|------------|
| `Cargo.toml` (root)           | Update package name to `vesper`, authors, version                      | Low        |
| `about.toml`                  | Set name = "Vesper", description, copyright                            | Low        |
| `about.hbs`                   | Update text only (no layout changes)                                   | Low        |
| `resources/`                  | Replace only `icon.icns` and `icon.ico` with Vesper branding           | Low        |
| `README.md` + `FAQ.md`        | Rebrand + add "Inference & Credits" section                            | Low        |
| `.agents/skills/`             | Add 8–12 JSON skill files encoding **your exact coding process**       | Low        |

### Phase 2 (Hours 2–4 — Inference Layer)

- New module: `crates/ai/src/providers/vesper_router.rs`
- Extend `crates/ai/src/lib.rs` to register the smart router.
- Add settings in `crates/settings*/` for:
  - Inference provider selection (Groq / Cloudflare / "Vesper Hosted")
  - API key field (or "Use Vesper Credits")
  - Current credit balance display (read-only for free tier)

### Phase 3 (Hours 4–6 — Business Logic + Polish)

- Usage tracking + credit system in `persistence/`
- Smart routing logic (see section 5)
- Build & test release binaries for macOS + Windows

---

## 5. Inference Architecture — Smart Multi-Account Routing + Free → Paid Switch

### Design Goals
- **Free tier**: Generous limits via Groq (your 3–4 accounts rotated automatically).
- **When limits exhausted or user upgrades**: Seamlessly switch to **your Cloudflare Workers AI** instances (zero marginal cost to you).
- **Smart delegation**: Automatically choose the best available account/key with remaining quota.
- **Model mapping**: Different internal model names for branding (e.g., "Vesper Fast" = Groq Llama-3.1-70B, "Vesper Pro" = Cloudflare Llama-3.1-405B).

### Architecture Components

1. **Provider Registry** (`crates/ai/src/providers/mod.rs`)
   - `GroqProvider` (OpenAI-compatible)
   - `CloudflareWorkersProvider` (OpenAI-compatible)
   - `VesperSmartRouter` (new) — decides which backend to call

2. **Smart Router Logic** (in `vesper_router.rs`)
   ```rust
   // Pseudo-code for clarity
   match user_tier {
       Free => {
           // Rotate across your 3-4 Groq accounts
           // Track usage per key in local SQLite
           // If all Groq keys near limit → fallback to Cloudflare (your cost)
       }
       Paid => {
           // Always use Cloudflare Workers (your infrastructure)
           // Or premium Groq if you want higher speed for paying users
       }
   }
   ```

3. **Account Management**
   - Store your 3–4 Groq API keys + 1–2 Cloudflare account tokens securely in `managed_secrets/` or OS keychain.
   - Simple round-robin + least-used + remaining quota scoring.
   - Local SQLite table: `inference_usage(account_id, tokens_used, last_reset)`.

4. **Credit / Limit System**
   - Free tier: e.g., 50,000 tokens/day or 100 requests/day (configurable).
   - When limit hit → show "Upgrade to continue" with Stripe (or your payment) link.
   - After payment → switch routing to Cloudflare Workers automatically.

### Why This Is a Good Idea (Yes — Excellent)

**Advantages**:
- Near-zero inference cost after free tier (you only pay for Cloudflare when users are on paid plan or free tier is exhausted — and even then it's cheap and predictable).
- Users get a generous free experience (thanks to Groq's excellent free limits).
- You control the upgrade path and monetization.
- Multiple accounts give you redundancy and higher total free quota.
- Smart routing feels premium ("Vesper always finds the fastest available model").

**Risks & Mitigations**:
- Key rotation complexity → Keep router simple (round-robin + local usage counter first).
- Rate limit errors → Implement exponential backoff + automatic failover.
- User confusion about "which model" → Always show friendly name ("Vesper Fast", "Vesper Pro") and hide real provider.

---

## 6. Business Model Implementation (Minimal)

- **Free Tier**: Groq-powered with daily token/request limit. Your 3–4 accounts provide the quota.
- **Upgrade Trigger**: When limit reached → beautiful in-app prompt → payment (Stripe or Lemon Squeezy recommended for speed).
- **Paid Tier**: Unlimited (or high limit) on your Cloudflare Workers AI infrastructure. You pay the (very low) Cloudflare bill.
- **Model Picker**: Shows "Vesper Fast", "Vesper Balanced", "Vesper Pro" — maps internally to different providers/models.

This model lets you acquire users aggressively with a strong free tier while having near-zero variable cost on the paid side.

---

## 7. Exact Files to Modify (Safe List Only)

**Branding (do these first)**:
1. `Cargo.toml`
2. `about.toml`
3. `about.hbs`
4. `resources/icon.*`
5. `README.md`
6. `FAQ.md`

**Inference (core engineering)**:
7. `crates/ai/src/lib.rs` — register new providers + router
8. `crates/ai/src/providers/mod.rs` — add module declarations
9. `crates/ai/src/providers/vesper_router.rs` — **new file** (smart routing logic)
10. `crates/ai/src/providers/groq.rs` — new or extend
11. `crates/ai/src/providers/cloudflare.rs` — new or extend
12. `crates/settings*/src/inference_settings.rs` — add provider selector + credit display
13. `crates/persistence/src/schema.rs` + migration — add `inference_usage` table (if tracking locally)

**No other files.**

---

## 8. Risk Mitigation & Bug Prevention

- Every change must be accompanied by a one-line comment: `// Vesper white-label change — safe, no UI impact`.
- Run `cargo check --workspace` after every logical change.
- Do **not** touch any `warpui*` import or re-export.
- Test inference routing in isolation first (unit test in `vesper_router.rs`).
- Keep the router behind a feature flag initially (`#[cfg(feature = "vesper-inference")]`).

---

## 9. Ready-to-Use Prompts for the Team / Coding Agent

### Prompt 1 — Kickoff (Copy-Paste into Cursor / Claude / Windsurf)

```
You are implementing the Vesper white-label fork of Warp.

CRITICAL RULES:
- NEVER modify any file under crates/warpui*, ui_components, or any UI code.
- All changes must be backend, settings, or inference layer only.
- Follow the exact plan in Vesper-WhiteLabel-Plan.md

Current task: Create the smart inference router.

1. Create crates/ai/src/providers/vesper_router.rs
2. Implement VesperSmartRouter that:
   - For free tier users: rotates across 4 Groq API keys (stored in managed_secrets)
   - Tracks daily usage per key in local SQLite
   - When all Groq keys are near limit, falls back to Cloudflare Workers AI
   - For paid users: always uses Cloudflare Workers
3. Expose friendly model names: "Vesper Fast", "Vesper Pro"
4. Add clear comments: // Vesper white-label change — safe, no UI impact

Start by reading the existing ai/ crate structure and http_client usage.
```

### Prompt 2 — Settings UI (Safe Addition)

```
Add a new section in the Settings window called "Inference & Credits".

Fields (read-only for now):
- Current Provider: "Groq (Free Tier)" or "Vesper Pro (Cloudflare)"
- Daily Usage: 12,450 / 50,000 tokens
- Upgrade button that opens a URL

Do NOT change any existing layout or styling. Only append a new section.
Use existing settings components only.
```

### Prompt 3 — Skills Population

```
Populate .agents/skills/ with 10 JSON files that encode the following coding process:
[PASTE YOUR ACTUAL 5–10 STEP CODING PROCESS HERE]

Each skill must have: name, description, prompt_template, tools_allowed, verification_steps.
Make them production-ready and specific to how you actually code.
```

---

## 10. 6-Hour Shipping Timeline

| Hour | Milestone                                      | Owner      |
|------|------------------------------------------------|------------|
| 0–1  | Fork repo, apply branding changes (Phase 1)    | You + Agent|
| 1–3  | Implement Groq + Cloudflare providers + router | Agent      |
| 3–4  | Add settings + credit display + usage tracking | Agent      |
| 4–5  | Test routing logic + fallback + model mapping  | Agent      |
| 5–6  | Build release binaries (macOS + Windows), test, prepare GitHub Release | You + Agent |

**Success Definition at Hour 6**:
- Branded Vesper app launches on macOS and Windows.
- Agent uses your custom skills.
- Inference works with Groq (free) and falls back to Cloudflare.
- No UI regressions.
- Binaries are signed and ready for distribution.

---

**This plan is engineered for speed and safety.**

We are not changing the UI. We are surgically enhancing the inference layer and adding smart business logic on top of the excellent foundation Warp gave us.

**Next immediate action**: Confirm this plan, then I will generate the first ready-to-apply patch or start implementing the `vesper_router.rs` file.

Let's ship Vesper in 6 hours. Ready when you are.