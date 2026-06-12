{
  "id": "58cfbcaf",
  "title": "Remove next-edit-prediction UI, keep simple Copilot ghost text",
  "tags": [
    "zed-custom",
    "edit-prediction"
  ],
  "status": "closed",
  "created_at": "2026-06-12T23:00:17.286Z"
}

# DONE — Remove next-edit-prediction UI, keep simple Copilot ghost text

## Final commit range
`487022c9f4..3abe377aa2` (plus registry change swept into `cad5bd3b5c` by the parallel text-thread agent's commit — see note below):

- (in `cad5bd3b5c`, TODO-a28ec443 commit) `crates/zed/src/zed/edit_prediction_registry.rs`: unwired Zed/Zeta/Ollama/OpenAI-compatible/Mercury providers — `EditPredictionProviderConfig` is now just `Copilot | Codestral`; settings values for the removed providers map to `None` (disabled). Copilot arm + `EditPredictionStore::start_copilot_for_project` preserved as required by TODO-a28ec443 Phase 2. *Note: my staged registry change was accidentally included in the other agent's commit because we share a worktree; content is exactly as intended.*
- `487022c9f4` — removed `rate_prediction_modal.rs` (1415 lines), `edit_prediction_context_view.rs` (417 lines); `edit_prediction_ui.rs` init reduced to no-op; button pruned: removed Zed/Mercury/Ollama/OpenAI-compat render arms (now `div().hidden()`), `build_edit_prediction_context_menu`, Zeta upsell/sign-in UI, `render_zeta_tab_animation`, data-collection menu, staff experiments submenu, Rate Predictions action, Mercury/Ollama/OpenAI token loading; `get_available_providers` now only Copilot/Codestral; `user_store` field dropped from `EditPredictionButton` (caller in `zed.rs` updated).
- `0def5947da` — removed `onboarding_modal.rs` (`ZedPredictModal`), `ResetOnboarding` action, `should_show_upsell_modal`/`ZedPredictUpsell` from `crates/edit_prediction`; `edit_prediction::init` now only registers Copilot SignIn/Reinstall/SignOut actions; removed `RatePredictionsModal`/`ZedPredictModal`/`zeta::*`/`edit_prediction::RatePredictions` bindings from all three default keymaps.
- `7e6f7bcda6` — `assets/settings/default.json`: default `edit_predictions.provider` = `"copilot"`.
- `b6b95812f0` — trimmed 14 unused deps from `edit_prediction_ui/Cargo.toml`.
- `1e39533846` — removed 4 upsell-modal tests from `edit_prediction_tests.rs`.
- `3abe377aa2` — default `copilot.enable_next_edit_suggestions` = `false` (Copilot NES is the one Copilot feature that produces away-from-cursor edits → jump/diff popover UI; disabling it guarantees ghost-text-only by default; user can still opt in via the status-bar Copilot menu toggle, which was kept).

## What was kept (deliberately)
- `EditPredictionDelegate` abstraction, ghost-text inlay path, accept/dismiss actions, eager/subtle modes.
- `crates/edit_prediction` engine crate (`EditPredictionStore` is required by Copilot startup post-ca23fa7c7c). Zeta modules left in place but unreachable from settings (registry never constructs `ZedEditPredictionDelegate`).
- Status-bar `EditPredictionButton` (it's the Copilot toggle) with Copilot/Codestral menus, provider switching, "Configure Providers", excluded-files settings.
- `EditPredictionProvider` settings enum variants (deleting them would break settings deserialization).
- Editor popover code in `crates/editor` (minimal approach per plan): with Zeta unwired and NES off, the diff-popover/jump paths are dormant. Phase-2 editor cleanup deemed unnecessary — no visible UI artifacts.

## Verification
- `cargo check -p edit_prediction -p edit_prediction_ui -p settings_ui -p editor -p language -p copilot` clean (only pre-existing `zed_actions` PathBuf warning from another agent's work).
- `./script/clippy -p edit_prediction_ui -p edit_prediction -p settings_ui` clean.
- `cargo test -p edit_prediction_ui` — 3/3 pass.
- Full `-p zed` check currently blocked by parallel TODO-a28ec443 work (`agent` crate `MentionUri::TextThread` non-exhaustive match) — not caused by this todo.

## Note for follow-up
- `crates/edit_prediction_cli`, `edit_prediction_metrics`, `edit_prediction_context`, `zeta_prompt` left in workspace (unused-by-default Zeta tooling) — droppable later if a leaner build is wanted.
- Manual smoke test still recommended: Copilot sign-in → ghost text appears → tab accepts; provider `zed` in settings does nothing.
