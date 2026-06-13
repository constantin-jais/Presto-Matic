# presto-ui

Mobile-first Dioxus design primitives for Presto-Matic.

## Scope

`presto-ui` renders reusable UI primitives only. Product state, API contracts, and
protocol transitions stay in `presto-core`; apps must not depend on
`presto-server` as a Rust library.

## Components

- `ThemeStyles` — injects token + component CSS.
- `AppSurface` — mobile safe-area surface.
- `Button`
- `TextInput`
- `Card`
- `Dialog`
- `Toast`
- `SourceCard`
- `BottomNav`
- `MobileDemo` — compact demo fragment for smoke/snapshot rendering.

## Mobile/a11y constraints

- Touch targets use `--presto-touch-target: 44px`.
- Focus uses `:focus-visible` with a tokenized focus color.
- Dialogs render `role="dialog"`, `aria-modal`, and `aria-labelledby`.
- Toasts render as polite status regions.
- No remote fonts, CDN, or component SaaS.
- Component CSS must use token variables for colors; raw color values live only in
  `tokens.css`.
