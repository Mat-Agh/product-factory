# 03 — UI and design system

**Status:** Active
**Last updated:** {{DATE}}
**Source:** Product Factory knowledge pack (typescript-web)

## Tokens and components

- Colour, spacing, radius, typography and motion are tokens in one global
  stylesheet (CSS custom properties). Components never hard-code values.
- Dark and light themes are paired; every token has both values.
- Shared components live in a package; features compose them.

## States

Every view designs loading, empty, success, error with retry, unauthorized,
forbidden and destructive-confirmation states deliberately.

## Accessibility

- Semantic elements first (`button`, `label`, `table`, landmarks).
- Every input has a visible label; errors are linked with `aria-describedby`.
- Live regions announce async results.
- Visible focus; full keyboard operation; logical tab order.
- `prefers-reduced-motion` respected.
- Contrast meets WCAG AA.

## Localization and RTL

- Copy comes from message files per language, never inline strings.
- Layout uses logical properties (`margin-inline-start`) so RTL works by setting
  `dir`.
- Isolate LTR values (emails, URLs, codes) inside RTL text with `dir="ltr"` or
  `<bdi>`.
