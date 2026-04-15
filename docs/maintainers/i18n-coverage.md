# ZeroClaw i18n Coverage and Structure

This document defines the localization structure for ZeroClaw docs and tracks current coverage.

Last refreshed: **February 21, 2026**.

## Canonical Layout

Use these i18n paths:

- Root project overview: `README.md` (English only)
- Full localized docs tree: `docs/i18n/<locale>/...`
- Optional compatibility shims at docs root:
  - `docs/commands-reference.<locale>.md`
  - `docs/config-reference.<locale>.md`
  - `docs/troubleshooting.<locale>.md`

## Locale Coverage Matrix

| Locale | Root README | Canonical Docs Hub | Commands Ref | Config Ref | Troubleshooting | Status |
|---|---|---|---|---|---|---|
| `en` | `README.md` | `docs/README.md` | `docs/commands-reference.md` | `docs/config-reference.md` | `docs/troubleshooting.md` | Source of truth |
| `zh-CN` | `README.md` | `docs/README.md` | - | - | - | Partial articles under `docs/i18n/zh-CN/` |
| `ja` | `README.md` | `docs/README.md` | - | - | - | Partial articles under `docs/i18n/ja/` |
| `ru` | `README.md` | `docs/README.md` | - | - | - | Partial articles under `docs/i18n/ru/` |
| `fr` | `README.md` | `docs/README.md` | - | - | - | Partial articles under `docs/i18n/fr/` |
| `vi` | `README.md` | `docs/i18n/vi/README.md` | `docs/i18n/vi/commands-reference.md` | `docs/i18n/vi/config-reference.md` | `docs/i18n/vi/troubleshooting.md` | Full tree localized |

## Root README Completeness

The repository ships a single root `README.md` in English. Locale-specific marketing or hub-style root READMEs are not maintained.

## Collection Index i18n

Localized `README.md` files under collection directories (`docs/getting-started/`, `docs/reference/`, `docs/operations/`, `docs/security/`, `docs/hardware/`, `docs/contributing/`, `docs/project/`) currently exist only for English and Vietnamese. Collection index localization for other locales is deferred.

## Localization Rules

- Keep technical identifiers in English:
  - CLI command names
  - config keys
  - API paths
  - trait/type identifiers
- Prefer concise, operator-oriented localization over literal translation.
- Update "Last refreshed" / "Last synchronized" dates when localized pages change.
- Full-tree locales (for example Vietnamese) should keep an "Other languages" section on their hub `README.md`.

## Adding a New Locale

1. Create canonical docs tree under `docs/i18n/<locale>/` (at least `README.md`, `commands-reference.md`, `config-reference.md`, `troubleshooting.md` when doing a full tree).
2. Add locale links to:
   - "Other languages" sections in localized hubs where they exist
   - language entry section in `docs/SUMMARY.md`
3. Optionally add docs-root shim files for backward compatibility.
4. Update this file (`docs/maintainers/i18n-coverage.md`) and run link validation.

## Review Checklist

- Links resolve for all localized entry files.
- No locale references stale filenames (for example `README.vn.md`).
- TOC (`docs/SUMMARY.md`) and docs hub (`docs/README.md`) stay consistent with active locales.
