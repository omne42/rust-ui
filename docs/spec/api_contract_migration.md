# API Contract Migration (Naming / Control / State / DX / Composition)

## Scope

This migration gate currently focuses on migrated source-first components:

- `components/alert`
- `components/breadcrumb`
- `components/button`
- `components/image`
- `components/pagination`
- `components/action-bar`
- `components/item`
- `components/snippet`
- `components/underlay`
- `components/text`
- `components/icon`

## Contract Rules

1. Public API naming:
- boolean flags should use `is_*`
- callbacks should use `on_*`
- default values should use `default_*`

2. Controlled/uncontrolled axis must be triad:
- `value + on_value_change + default_value`

3. Default source must be centralized:
- default/fallback normalization should live in `logic.rs`
- `view.rs` should consume normalized output

4. State normalization centralized:
- `view.rs` should consume `logic::*` normalization/derivation

5. Discrete state typing:
- `variant/size/mode/status` should be enum-like inputs
- avoid string/bool free-form combinations for mutually exclusive state

6. State primitive source boundary:
- component layer should not bind business/global stores directly
- controllable/open-state primitive wiring should not sprawl in `view.rs`
- `logic.rs` should stay POJO-like (no reactive container bindings)

7. Async contract:
- if async exists, enforce `is_loading` + `aria-busy` + error/retry path + disabled mapping
- if no async path, `check2.md` must mark explicit N/A reason

8. DX paradox:
- do not require internal `state` objects as mandatory props
- docs-app must include minimal component entry path for focused components

9. Composition-first API:
- forbid parallel-array default APIs (`labels/titles/panels/...`)
- config input is allowed only when typed `ItemSpec`-style (no `Vec<String>` item API)

10. Macro/Micro duality:
- high-frequency drag loops should stay in local `view/motion` loop
- terminal convergence should flow back via terminal action (`DragEnd`-style)

11. Two-pass geometry rendering:
- geometry overlays should follow intent/measure/rectification
- include idempotence guard to avoid render/measure loops

12. Registration protocol:
- dynamic item sets should report register/unregister through context
- logical order should be explicit (`items_order`), not `HashSet` iteration order

13. Slot projection strategy:
- container projection policy should be explicit (`Lazy/KeepAlive/Eager`)
- `KeepAlive` hidden branches should expose hidden notification hook

14. Env stream discipline:
- raw env subscriptions (`Resize/Intersection/Theme`) should be sampled/debounced
- map to semantic actions before entering logic layer

15. Event light cone:
- large collections should use context bus + selector/state compression
- avoid O(N) prop drilling style fan-out by default

16. Causality bus:
- bus broadcast/dispatch paths should preserve `TraceId` chain

17. A11y + i18n/l10n:
- interactive contracts should expose verifiable role/aria semantics
- visible text should stay overrideable (props/app injection/fallback)
- `lang/dir` access path should be present
- avoid duplicating shared A11y helper names in component layer

18. Observability contract:
- key state/source markers should be exposed via stable `data-*`/`aria-*`
- avoid free-form marker text that cannot be contract-verified
- selectors should prefer semantic markers over DOM-depth assumptions

19. Style contract:
- state-driven styles should rely on stable state markers
- forbid fragile structural selectors like `:nth-child`
- inline style should be limited to necessary CSS variable plumbing

20. Semantic-test contract:
- semantic assertions (`role`/`aria`/`data-*`) are required
- snapshots are optional but must not replace semantic checks
- matrix coverage should include applicable controlled/disabled/keyboard/pointer/platform paths

21. Component file responsibilities:
- `mod.rs` keeps export boundary / feature gates only
- `logic.rs` keeps normalization/derivation/source markers, no DOM/view bindings
- `styles.rs` stays token-first static CSS only
- `view.rs` keeps rendering/mounting, avoids introducing hidden local type-system state
- `motion.rs` keeps motion contract attach path, no component rendering concern

22. `spec.rs` anti-sprawl:
- `spec.rs` is allowlist-scoped to complex components (currently button)
- when present, it must stay versioned and backed by contract tests

23. Token-first style system:
- `styles.rs` must consume `var(--ui-*)` tokens and avoid private token namespaces
- utility-first/CSS-in-Rust markers are forbidden in component-library source as default paradigm
- CSS registry/injection must stay centralized in `crates/ui/src/css.rs` and `UiRoot`

24. Visual baseline quality:
- docs-app must keep a baseline page for default theme quality
- baseline page must include Button/Input/Overlay
- e2e visual baseline spec must exist and keep screenshot assertions for key slices

## Compatibility Strategy

- Existing legacy names are temporarily allowed only when they are already in baseline debt.
- No new debt is allowed: CI blocks any baseline drift.
- Migration is monotonic:
1. add canonical prop name in `view.rs`
2. normalize legacy + canonical in `logic.rs` (canonical wins)
3. migrate callsites/docs
4. remove legacy alias and refresh baseline (debt count must decrease)

For newly-added rules (5-24), the same policy applies:
- baseline captures current debt on focused scope
- each migration reduces baseline
- no new violations are allowed in CI

## Enforcement

- Gate script: `scripts/check-api-contracts.sh`
- Debt baseline: `scripts/baseline/api_contract_violations.txt`
- Hygiene integration: `scripts/check-rust-hygiene.sh`

To intentionally refresh baseline after planned debt reduction:

```bash
UPDATE_API_CONTRACT_BASELINE=1 ./scripts/check-api-contracts.sh
```
