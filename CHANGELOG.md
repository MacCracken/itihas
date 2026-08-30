# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [2.4.3] - 2026-08-30

Two 2.4.x hardening-arc items — checked allocation and query tracing — plus the
documentation and repository housekeeping that preceded them. **299 tests**
(was 285), coverage 94%, docs/fmt/dist gates clean.

### Added

- **`xalloc(n)` — checked allocation, closing CWE-690.** `alloc()` returns `0`
  on exhaustion rather than aborting, and **all 21** allocation sites in itihas
  wrote into the result immediately: a struct constructor `store64`s its fields,
  a buffer builder `store8`s its bytes. Every one was therefore a write through
  a near-NULL pointer under memory pressure, faulting at an address that says
  nothing about where it came from.

  17 sites now use `xalloc`, which aborts with a diagnostic on stderr and exit
  134 (128 + SIGABRT) — the AGNOS policy avatara set in its ADR-009, matching
  Rust and Go, whose allocators abort rather than return null because a library
  cannot invent a recovery its caller did not ask for. The remaining 4 are
  hoosh's network buffers, where the caller already handles a `0` return and
  logs it, so those check `alloc` explicitly and degrade instead.

  `xalloc` reports a **non-positive size separately** from exhaustion. The
  stdlib `alloc(0)` returns 0, so a zero-size request would otherwise abort
  claiming "out of memory" and send the reader hunting the wrong problem. No
  itihas call site can reach it — every size is a positive struct constant or
  `len + 1` — but the diagnostic is honest if one ever does.

- **Query tracing restored — 45 of the Rust original's 52 sites.** The port kept
  `logging.cyr` but never called it from anything, so `itihas_log_init` set a
  level nothing honoured and an operator had no way to see what a consumer was
  asking for. Every query entry now traces its lookup argument, matching the
  Rust `tracing::debug!` sites one for one: 42 across the ten data modules, plus
  three in hoosh (`answer_from_data`, tool selection, and `llm_answer` at
  **info** rather than debug, since that is the one path that leaves the process
  and spends a consumer's tokens). The remaining seven live in `mcp.rs`, which
  is still unported — they come with 2.5.0.

  Four helpers in `util.cyr` (`itrace_i`, `itrace_s`, `itrace_range`,
  `itrace_pair`) emit through sakshi's structured `sakshi_log_kv`, so a trace
  reads `looking up eras containing year year=500` rather than a flat string.
  Each checks the level **first** and returns before touching its arguments, so
  a build with logging off pays one call and one compare — the argument is an
  integer or an already-materialised `Str`, never a construction. They live in
  `util.cyr` rather than `logging.cyr` because Cyrius resolves forward
  references single-pass and `logging.cyr` is included after the ten data
  modules that call them.

### Removed

- **`rust-old/`** — the preserved Rust v1.5.0 tree (29 files, 10,842 lines).
  A per-function audit confirmed the port is complete: every `pub fn` in the 14
  Rust modules has a Cyrius counterpart, and the Cyrius modules carry *more*
  functions than their originals because the port split accessors out. The two
  apparent gaps were naming, not omissions — `civilizations_at` survives as the
  hoosh tool name (resolving through `civs_active_at`), and `init_with_level`
  became `itihas_log_init_level`. Everything the Rust tests, examples and
  benchmarks exercised maps to a Cyrius equivalent.

  `mcp.rs` was the one module never ported, so its reference value is preserved
  as **`docs/development/mcp-port-reference.md`** — public surface, all five tool
  schemas, handler-to-query mappings, and porting notes — rather than as 714
  lines of un-buildable Rust. The full tree remains in git history.

- **Rust-era `.gitignore` entries** — `**/*.rs.bk`, `Cargo.lock`, `/target/`,
  `/target/criterion/`, and the Rust/LLVM coverage artifacts (`*.profraw`,
  `*.profdata`, `lcov.info`, `tarpaulin-report.*`). None can be produced by this
  repository any more.

### Changed

- **`docs/development/roadmap.md` rewritten** (157 → 116 lines). It now tracks
  **open work only** — every "Completed in X" section was duplicating the
  CHANGELOG, which the file's own header said it would not do. Open items are
  organised into arcs: **2.4.x** hardening (the 2.4.2 audit's un-batched
  findings), **2.5.0** tool integration, **2.6.x** performance, **2.7.x**
  structural, plus demand-gated data work.

- **bote is no longer a blocker.** The roadmap recorded v2.5.0 as "blocked on
  bote toolchain bring-up". That is stale: bote **3.3.7** is pinned to cyrius
  6.5.35, builds clean, and `dist/bote.cyr` exports the whole surface itihas
  needs. The roadmap now carries a verified API mapping (`tool_def_new`,
  `schema_new`, `ann_read_only`, `registry_register`, `dispatcher_handle`,
  `host_registry_*`) and points at `bote/src/libro_tools.cyr ::
  libro_tools_register()` as a working reference for the same pattern. The mcp
  and daimon modules are now ordinary scheduled work.

- **`benchmarks-rust-v-cyrius.md`** — the "Not Yet Ported" table was a
  2026-04-12 snapshot in which six of eight rows had since shipped (JSON
  serialization, case-insensitive lookups, description fields, the hoosh module,
  the `.bcyr` harness) or become unblocked (mcp, daimon). Each row now carries
  its current status. Its `interactions_for_rome` note also still said 22
  interactions; the table has held 21 since 2.4.2.

- **Data tables exempted from the line-length rule.** `cyrius lint` reports
  `#skip-lint` **per line**, so 352 data rows carry the marker directly, along
  with 8 single-line offset/type enums and hoosh's two irreducible string
  constants (the system prompt and the tool-definitions JSON). The two genuinely
  over-long *code* lines — the belligerent test in `campaigns_by_civilization`
  and the bidirectional match in `interactions_between`, both introduced by
  2.4.2 — were **wrapped** rather than exempted, since those were real style
  problems. `src/` goes from **364 lint warnings to 0**, which makes the count a
  usable gate again instead of a standing backlog.

- **Civilization-name fields are documented as free text.** The `civilization`
  field on figures, sites, routes and events, and a campaign's belligerents, are
  free-text labels rather than keys into `all_civilizations()`. This is now
  stated at each constructor and in
  [usage.md](docs/guides/usage.md), because the alternative reading — that they
  are foreign keys — would make 72 of the 223 references (32%) look like bugs
  when they are correct.

  Those 72 split two ways, and both are legitimate: real polities outside the
  curated 53-entry civilization table (`Maurya Empire`, `Roman Republic`,
  `Carthage`, `Gupta Empire`, `Kingdom of England`), and labels that name no
  state at all (`Pre-Pottery Neolithic`, `Neolithic Anatolia`, `Italian
  city-states`) because that is often the only honest attribution. Consequently
  **no referential-integrity test exists** — one would fail on correct data — and
  a name absent from `all_civilizations()` remains a valid query. A reference
  whose dates contradict the record pointing at it is still a data bug; two such
  were fixed in 2.4.2.

### Fixed — stale documentation

- **Battle count corrected 40+ → 35** in `README.md`, `src/lib.cyr`,
  `src/campaign.cyr` and `docs/guides/usage.md`. Counted directly from the
  `bat_add` rows. All ten entity counts were re-verified against the data and
  are correct at 338 total.
- **`docs/architecture/overview.md`** — dropped the `rust-old/` row from the
  source tree and added the `dist/itihas.deps` sidecar that 2.4.1 introduced.
- **`SECURITY.md`** — the 1.x row pointed at a directory that no longer exists.
- **`CLAUDE.md`** — the documented docs layout omitted `docs/sources/` (nine
  files of data provenance) and described `roadmap.md` as holding completed
  items, which is the opposite of the convention now in force.

### Performance

Interleaved A/B against the 2.4.2 release (`c835ff1`), **6 rounds each**, same
machine and session. **Aggregate +1.4%** across all 28 benchmarks
(9,877 → 10,018 ns summed medians), with four regressing beyond their own
run-to-run spread:

| Benchmark | 2.4.2 | 2.4.3 | Delta | Spread |
|-----------|-------|-------|-------|--------|
| `causes_of_french_revolution` | 238 ns | 248 ns | +4.2% (+10 ns) | 2.4% |
| `chain_writing_depth3` | 387 ns | 403 ns | +4.1% (+16 ns) | 3.0% |
| `campaigns_between_500bce_500ce` | 198 ns | 206 ns | +4.0% (+8 ns) | 2.4% |
| `calendar_by_name_gregorian` | 124 ns | 129 ns | +3.6% (+4 ns) | 2.4% |

**These are accepted, not incidental.** They are the measured price of the two
things this release exists to add: `xalloc` puts a call and a null check on every
allocation (which is why the causality paths, allocating a `CH_SIZE` entry per
chain node, move most), and every query now carries a level-gated trace call.
Nothing here is a hot-path algorithm change; no benchmark improved, and none was
expected to.

An earlier 4-round pass reported **no** regressions at +0.9%. That measurement
was wrong — its run-to-run spread was wide enough to swallow deltas this size.
Six rounds tightened the spreads to 2.4-3.0% and the same four benchmarks
separated cleanly and reproducibly. The six-round figures above are the ones to
trust; the earlier number is recorded here only so the discrepancy is not
mistaken for drift.

A variant that avoided the `sakshi_get_level()` call by reading sakshi's level
global directly was measured and **rejected** — it landed within noise of the
public-API version, so it bought nothing for the layering it broke.

## [2.4.2] - 2026-08-29

P(-1) scaffold-hardening release — a full audit/refactor/security sweep with no
new features. Ten review dimensions with adversarial verification produced 49
confirmed findings; this release repairs them. Test suite 153 → 285 assertions,
reference coverage 54% → 96%, and `cyrius audit` fmt/docs gates go from failing
to clean. Every defect below was reproduced before it was fixed.

### Fixed

- **hoosh: stringified tool `arguments` were never decoded, disabling the entire structured-data path.** `_extract_tool_call` returned the raw wire span for the OpenAI `/v1/chat/completions` shape — the only shape hoosh's own emitter produces — so `arguments` arrived still JSON-encoded (`{\"event\": \"French Revolution\"}`). `json_parse` yielded **zero pairs**, `parse_tool_call` returned 0, and `llm_answer` silently fell through to ungrounded LLM prose with an empty `data_json` for every one of the six tools. Now decoded via the new `json_unescape`; the object form is untouched, so it cannot double-unescape.
- **hoosh: out-of-bounds reads and a negative-length `str_new` on hostile responses.** In `_extract_tool_call`, a body with `"arguments"` but no following `{` or `"` left the scan index at `slen`, and the two following `load8(data + i)` calls read one byte past the buffer. An unterminated tool name made `name_start` exceed `i`, handing `str_new` a **negative length**. `_extract_content` repeated the same pattern. All three paths now bounds-check and return empty.
- **hoosh: the escaped-quote terminator ignored backslash parity.** `load8(data+i-1) != 92` treats a quote preceded by *any* backslash as escaped, so a value ending in a literal `\\` ran past its closing quote into the rest of the body. Replaced with a parity count at all three sites (`_bs_parity_even`).
- **hoosh: `Content-Type` header truncated by one byte.** `memcpy(buf + pos, "Content-Type: application/json", 29)` — the literal is **30** bytes, so every request sent `Content-Type: application/jso`. The other five header literals were verified correct.
- **hoosh: argument parsing fell through from the object branch into the string branch.** A bare second `if` (now `else if`) could overwrite `args_str` after the object form had already been parsed.
- **hoosh: `_extract_content` returned raw escaped bytes**, so `\n` and `\"` reached callers as literal backslash sequences through `resp_content()`.
- **hoosh: `hoosh_post` conflated five distinct failures into `0`, blocked forever, and closed the wrong syscall.** The socket close used a bare `syscall(3, fd)` — correct only on x86_64 Linux, where `3` is `close`; on aarch64 that number is `io_cancel` (close is 57), leaking the fd and issuing a wild syscall. There were no send/recv timeouts, so a process squatting the port hung the caller with no cancellation path. Both `SYS_WRITE` calls discarded their return, so a short write sent a partial body. The read loop ended identically on clean EOF, on error, and on filling the buffer — the last case then parsed a response cut mid-JSON. Now: `SYS_CLOSE`, 30s `SO_RCVTIMEO`/`SO_SNDTIMEO` via `net`'s helpers, a looping `_write_all`, truncated responses refused rather than parsed, and each failure logged through `sakshi_warn`.
- **serial + hoosh: incomplete JSON escaping, in three places.** `_json_str` and the request builder each carried their own copy escaping only `"`, `\` and LF; TAB, CR, BS, FF and every other C0 control emitted raw, producing invalid JSON. The `model` argument was interpolated with **no** escaping at all. The rule of three is satisfied, so the logic is extracted to `json_escape_into` in `util.cyr` with full RFC 8259 coverage including `\u00XX`. The shipped dataset is pure printable ASCII, so no existing output was malformed — the exposure was caller-supplied strings and `llm_answer(question)`.
- **Substring matching on `;`-delimited fields returned the wrong records.** `campaigns_by_civilization("Roman Empire")` returned exactly one campaign — the **Thirty Years' War**, matched inside `"Holy Roman Empire;Spanish Empire"` — and none of the actual Roman campaigns. `campaigns_by_commander("Darius I")` also returned Alexander's campaigns via the `"Darius III"` prefix, 152 years apart. Replaced with token-aware matching (`str_list_has`) on the belligerent, commodity and civilization fields. Commander lookup uses `str_list_has_name`, which additionally accepts a leading whole-word prefix so surname search (`"Napoleon"` → `"Napoleon Bonaparte"`) still resolves while `"Darius I"` no longer matches `"Darius III"`. The region fields are deliberately left as substring matches — they are comma-joined hierarchical regions where that is the intended semantic.
- **Causality and interaction lookups were case-sensitive** while every `*_by_name` was not, so `causes_of("french revolution")` returned 0 against 2 for the cased spelling — a silent wrong answer, and a real hazard on the hoosh path where an LLM chooses the casing. `causes_of`, `effects_of`, `interactions_for`, `interactions_between` and `interaction_neighbors` now fold case. Internal data-to-data comparisons deliberately still use `str_eq`.
- **`causal_chain` ignored `max_depth` below 1.** The seed loop stored direct effects at depth 1 before `max_depth` was consulted, so `causal_chain(event, 0)` returned 2 entries and broke the `depth <= max_depth` invariant the frontier loop enforces.
- **An empty region needle matched every record.** `str_contains` reports a match for a zero-length needle, so `civs_by_region("")` returned all 53 civilizations and `eras_by_region("")` all 25. Both now return empty.
- **Ongoing-era sentinel leaked into user-facing text.** `resolve_era_lookup` rendered `"Information Age (1970 - 2147483647)"` as its data-grounded answer. The stored value is unchanged (range predicates depend on it) and still appears in JSON; only the prose renders `"present"`, via the new `ERA_ONGOING` constant and `era_is_ongoing()`.
- **Historical data corrections**, each contradicted by the repo's own other tables; no entity count changes:
  - `interaction.cyr` — the Mongol "Sack of Baghdad" was recorded against **Persian Empire** (the Achaemenid record, which ends −330) dated 1219–1258, i.e. 1,549 years after that civilization's own end, and conflated the 1219 Khwarezmian campaign with the 1258 sack. Retargeted to **Arab Caliphates** (632–1517) at 1258.
  - `interaction.cyr` — the **Punic Wars** were recorded as a Roman conquest of **Phoenicia** in a window (−264..−146) entirely after Phoenicia's −300 end. Rome fought **Carthage**, which `campaign.cyr` already names as the belligerent.
  - `trade.cyr` — `"Viking Civilization"` → `"Viking/Norse"`, the name every other module uses; `routes_by_civilization("Viking/Norse")` previously returned nothing.
  - `site.cyr` — Great Zimbabwe's civilization `"Kingdom of Zimbabwe"` → `"Great Zimbabwe"`; `sites_by_civilization("Great Zimbabwe")` previously returned nothing.
  - `calendar.cyr` — Hebrew epoch −3760 → −3761. The Anno Mundi epoch is 3761 BCE; −3760 is the astronomical year number, mixing conventions with the seven other rows. The year convention is now documented on the enum.
  - `calendar.cyr` — the Maya **Long Count** was described with the Haab' month structure; replaced with the positional day count (18 uinal × 20 kin = 360-day tun).
  - `civilization.cyr` — Chola Dynasty founding 300 → **848**. 300 CE falls in the Chola interregnum, while the record's own end (1279), its traits and its only Chola event (1025) are all Imperial Chola, founded by Vijayalaya in 848.

### Added

- **`src/util.cyr`** — `json_escape_into` (RFC 8259 escaping into a `str_builder`), `json_unescape` (inverse, decoding `\u` escapes to UTF-8 and mapping lone surrogates to U+FFFD), `str_list_has` / `str_list_has_name` (token-aware membership on delimited fields), and the allocation-free `_ci_eq_bytes` comparison they share.
- **`src/era.cyr`** — `ERA_ONGOING` and `#must_use fn era_is_ongoing(p)`.
- **`src/hoosh.cyr`** — `_write_all`, `_bs_parity_even`, `_hoosh_warn`, and a `HooshWire` enum naming the previously bare 2048 / 131072 / timeout wire limits. `_build_post_header` is now bounds-checked against its buffer and returns −1 rather than overrunning.
- **132 new test assertions (153 → 285)** covering every repair above plus the previously untested surface: the whole hoosh wire path (`_build_post_header`, `_extract_tool_call` in both argument forms, `_extract_content`, `_build_inference_request`, and the offline `hoosh_post`/`llm_answer` connect-refused paths, none of which need a server), serializer escaping and round-tripping through `json_parse`, the three `*_to_json` functions that had no assertions, `resp_data` (which had zero call sites in the repo), and one assertion for every public accessor and constructor.

### Changed

- **Diagnostics on the hoosh failure paths.** `logging.cyr` set a level and nothing in the library ever emitted; the five failures that collapsed to `0` in `hoosh_post` are now distinguishable via `sakshi_warn`. Full tracing restoration across all modules remains deferred.
- **Documentation corrected against the code**: `docs/architecture/overview.md` claimed 297 entities (the stale 2.0.0 figure — it is 338, as the same file's own table says) and a 117KB binary that appears nowhere else in the repo; the real `CYRIUS_DCE=1` build is 674KB, and DCE NOPs unreachable code rather than stripping it, so it does not shrink the file. The "zero external deps / vendored stdlib" line now describes the regenerated-from-pin model adopted in 2.3.3. `docs/development/roadmap.md`'s Port Coverage table listed `hoosh.rs` as blocked (it shipped in 2.2.0) and credited `main.cyr` with 3 functions (it has none since 2.3.1). `README.md`'s 141KB figure is now scoped to the 2.0.0 release that measured it. `serial.cyr`'s header referenced a `json.cyr` and symbols that do not exist; `interaction.cyr`'s header said 22 rows for a 21-row table.
- **All 14 previously undocumented public functions now carry doc comments** (`cyrius audit` docs gate: 14 undocumented → clean). `battle_to_json` gained the `#must_use` the 2.3.4 sweep missed, as did the three pure `util.cyr` helpers.
- **`cyrius fmt` gate restored** — `serial.cyr`, `hoosh.cyr`, `main.cyr` and `util.cyr` reformatted.

### Performance

Measured by interleaved A/B: two bench binaries (2.4.1 at commit `68313be` vs this
working tree, both compiled by cycc 6.5.36), alternated for 3 rounds on the same
machine and session, comparing medians against each benchmark's own run-to-run
spread. **Net across all 28 benchmarks: 10,277 → 9,812 ns, −4.5%.** 17 flat.

- **Wins.** `events_between_500bce_500ce` 2622 → 1891 ns (**−28%**), `civs_active_at_500bce` 887 → 710 ns (**−20%**), `eras_containing_500bce` 415 → 352 ns (**−15%**), `campaigns_between_500bce_500ce` 229 → 197 ns (**−14%**). From two changes: `vec_sort` now indexes the vec's backing store directly instead of paying a bounds-checked `vec_get`/`vec_set` per comparison (it stays an insertion sort deliberately — `lib/vec.cyr`'s `vec_sort_by` is introsort and **not stable**, which would reorder records sharing a year), and `influence_score` scans the interaction table in place rather than materializing the vec `interactions_between` builds only to sum over it.
- **Regressions, and why they are accepted.** `interactions_for_rome` 396 → 583 ns (**+47%**), `influence_score_egypt_hittite` 411 → 545 ns (+33%), `causes_of_french_revolution` 180 → 238 ns (+32%), `chain_writing_depth3` 321 → 381 ns (+19%), `routes_by_commodity_silk` 268 → 311 ns (+16%), `campaigns_by_commander_napoleon` 266 → 291 ns (+9%). These are the direct cost of the case-insensitivity and token-membership correctness fixes: `str_eq_lower` and `str_list_has` are Cyrius-level loops where `str_eq`/`str_contains` were single calls into optimized primitives. Two alternatives were measured and both failed to recover it — an exact-match `str_eq`/`memeq` fast path, and a fused compare that folds only on a differing byte — so the extra call layer, not the folding arithmetic, is the cost. The absolute penalty is 25–187 ns on sub-microsecond queries, in exchange for lookups that previously returned silently wrong answers. An earlier form of `str_list_has` cost **8×** (silk 2104 ns, Napoleon 2775 ns) by scanning byte-by-byte with a per-byte branch; tightening the separator scan brought it to the numbers above.
- `all_civilizations` 4 → 5 ns is 1 ns of quantization on an O(1) accessor, not a real change.

## [2.4.1] - 2026-08-29

Toolchain modernization release — the cyrius 6.4.69 → 6.5.36 bump and the
vendored-`lib/` resync it requires. Source behavior unchanged; 153 tests pass.
The build is now warning-free, and the bump is performance-neutral.

### Changed

- **Toolchain pin bumped 6.4.69 → 6.5.36** (`cyrius.cyml` `[package].cyrius`). Resyncs the local `lib/` snapshot to the 6.5.36 stdlib — 25 declared `[deps].stdlib` modules re-vendored via `cyrius lib sync`, verified byte-identical to the pinned snapshot — clearing the toolchain-drift build warning. `[deps].stdlib` is unchanged; `net` + `http` remain load-bearing (hoosh's self-contained HTTP client). `dist/itihas.cyr` regenerated with the v2.4.1 header.

### Added

- **`dist/itihas.deps`** — new dep sidecar emitted by 6.5.36's `cyrius distlib` alongside `dist/itihas.cyr`, listing the 16 `[deps].stdlib` modules the bundle needs in scope. Consumed by `cyrius deps` on the consumer side (sankhya, avatara, kiran, joshua, jnana, lipi, vidya); ship it with the bundle.

### Fixed

- **Build is warning-free.** The three `lib/bayan.cyr:1439/1443/1448` "assigning non-pointer to typed pointer" warnings carried by the 6.4.69 snapshot are fixed upstream in the 6.5.36 bayan. Only the informational unreachable-fn note remains (eliminated by the `CYRIUS_DCE=1` build used in CI and releases).

### Performance

- **Performance-neutral toolchain bump — no regressions, no wins.** Measured by an interleaved A/B: two bench binaries (pre-bump 6.4.69 `lib/` vs post-bump 6.5.36 `lib/`, both compiled by cycc 6.5.36), run in alternation for 3 rounds each on the same machine and session. All 28 benchmarks land within **±2% of median**, and every delta is smaller than that benchmark's own run-to-run spread: `civs_active_at_500bce` 902 → 884 ns (-2.0%, spread 3.1%), `events_at_year_476` 807 → 791 ns (-2.0%, spread 2.5%), `civs_by_region_mediterranean` 853 → 840 ns (-1.5%), `causes_of_french_revolution` 183 → 181 ns (-1.1%), `events_by_category_war` 948 → 957 ns (+0.9%), `sites_active_at_500bce` 303 → 306 ns (+1.0%); `chain_writing_depth3` 324 → 325 ns, `interactions_for_rome` 402 → 401 ns, `eras_containing_500bce` 420 → 419 ns, and the O(1) `all_*` accessors all hold flat at 5 ns. Unlike 2.3.5 (creep) and 2.4.0 (recovery), the 6.5.36 stdlib neither costs nor gains itihas anything. Full run recorded in `bench-history.csv` / `benchmarks.md`.
- **Benchmark methodology note (affects cross-release CSV comparisons).** 6.5.36's `lib/bench.cyr` adds timer-floor calibration (v6.5.19): it measures what one `now_ns()` read costs on the host and subtracts it from every sample, which 6.4.69's harness did not do. itihas calls `bench_run_batch` with `BATCH=10000`, so the subtraction removes ~0.13 ns/iteration — under 0.1% of the query benchmarks, and the only reason the `all_*` rows can read 4 ns rather than 5 ns. Separately, absolute numbers in this run are well below the 2.4.0 release row for the same benchmarks (e.g. `chain_writing_depth3` 546 → 325 ns); that gap is machine/session difference, **not** attributable to this bump — the pre-bump binary measured the same faster numbers in this session. Only the interleaved A/B above is attributable.

## [2.4.0] - 2026-07-21

Toolchain modernization release — the cyrius 6.2.11 → 6.4.69 bump and the
vendored-`lib/` resync it requires. Source behavior unchanged; 153 tests pass.
The 6.4.x stdlib codegen recovers the 6.2.x creep and improves on it.

### Changed

- **Toolchain pin bumped 6.2.11 → 6.4.69** (`cyrius.cyml` `[package].cyrius`). Resyncs the local `lib/` snapshot to the 6.4.69 stdlib — 25 declared `[deps].stdlib` modules re-vendored via `cyrius lib sync` — clearing the toolchain-drift build warning (the local snapshot was a stale 6.2.11 copy). `[deps].stdlib` is unchanged; `net` + `http` remain load-bearing (hoosh's self-contained HTTP client). `dist/itihas.cyr` regenerated with the v2.4.0 header.

### Performance

- **Toolchain-codegen recovery (no source change), attributable to the 6.4.69 stdlib snapshot.** Same-machine, same-session comparison (pre-bump 6.2.11 `lib/` vs post-bump 6.4.69 `lib/`, both under cycc 6.4.69) shows the vec/str-heavy query paths **10–36% faster** while the O(1) collection accessors (`all_*`) hold flat at 4–5 ns: `chain_writing_depth3` 853 → 546 ns (**-36%**, essentially back to its pre-6.2.x baseline), `interactions_for_rome` 668 → 453 ns (**-32%**), `causes_of_french_revolution` 311 → 237 ns (**-24%**), `campaigns_between_500bce_500ce` 367 → 285 ns (**-22%**), `campaigns_by_commander_napoleon` 387 → 309 ns (**-20%**), `routes_by_region_east_asia` 364 → 291 ns (**-20%**), `routes_by_commodity_silk` 401 → 325 ns (**-19%**), `sites_active_at_500bce` 433 → 360 ns (**-17%**), `eras_containing_500bce` 545 → 475 ns (**-13%**), `influence_score_egypt_hittite` 544 → 471 ns (**-13%**), `figures_by_domain_scientist` 631 → 564 ns (**-11%**), `sites_by_region_near_east` 649 → 583 ns (**-10%**). This reverses the codegen creep itihas recorded across the 6.0.50 → 6.2.11 bump in 2.3.5; no regressions. Full run recorded in `bench-history.csv` / `benchmarks.md`.

## [2.3.5] - 2026-06-15

Toolchain modernization release — the cyrius 6.0.50 → 6.2.11 bump and the
stdlib data-format carve it requires. Source behavior unchanged; 153 tests pass.

### Changed

- **Toolchain pin bumped 6.0.50 → 6.2.11** (`cyrius.cyml` `[package].cyrius`). Resyncs the vendored `lib/` snapshot to the 6.2.x stdlib, clearing the toolchain-drift and `./lib/ shadows version-pinned lib/` build warnings (the local snapshot was a stale 6.0.50 copy).
- **`cyrius.cyml` `[deps] stdlib`: `json` → `bayan` (+ `result`, `io`).** Cyrius 6.1.25 carved the data-format modules (json/toml/csv/base64/bigint/u128) out of the stdlib into the **bayan** sibling, folded back as `lib/bayan.cyr`; standalone `json` no longer exists and was dropped from the auto-prepend list. hoosh's tool-call parsing (`json_parse`/`json_get`) is unchanged — bayan keeps those as compat aliases for `bayan_json_*`. `result` + `io` are bayan's internal prereqs. Mirrors avatara's 2.7.1 `json`-drop.

### Performance

- **Toolchain-codegen creep (no source change), attributable to the 6.2.x stdlib snapshot.** Same-binary, same-machine comparison (pre-bump 6.0.50 `lib/` vs post-bump 6.2.11 `lib/`, both under cycc 6.2.11) shows the vec/str-heavy query paths slower while the O(1) collection accessors (`all_*`) hold flat at 5–6 ns: `chain_writing_depth3` 544 → 948 ns, `eras_containing_500bce` 490 → 620 ns, `causes_of_french_revolution` 244 → 346 ns, `figures_by_domain_scientist` 592 → 695 ns, `sites_active_at_500bce` 373 → 472 ns, `campaigns_between_500bce_500ce` 293 → 399 ns. This matches the codegen creep avatara recorded across its 6.1.34/6.2.11 bumps; it is the cost of the toolchain bump, not a regression in itihas source. Full run recorded in `bench-history.csv` / `benchmarks.md`.

## [2.3.4] - 2026-06-03

### Added

- **`#must_use`** — Applied the real, compiler-enforced `#must_use` attribute to the pure public API across all 10 data modules plus `serial` (`*_to_json`) and `hoosh` (query builders, response accessors, tool defs). Discarding a query/lookup/serialization result now warns at compile time.
- **`defer`** — `hoosh_post` closes its socket via `defer { syscall(3, fd); }`, replacing per-path manual closes and guaranteeing cleanup on every exit.

### Changed

- **CLAUDE.md** — Corrected the attribute principles: Cyrius uses bare `#attr` syntax, not Rust's `#[attr]` (which is just a comment — a silent no-op). Replaced the Rust-holdover `#[non_exhaustive]`/`#[must_use]`/`#[inline]` with the real Cyrius set (`#must_use`, `#regalloc`, `#deprecated`, `#derive`); `#non_exhaustive` and `#inline` are not Cyrius attributes.
- **tests/itihas.bcyr** — Bench wrappers now `return` their query result and the warm-up assigns it, so the new `#must_use` attributes don't flag the harness; per-iteration measurements are unchanged.

## [2.3.3] - 2026-06-03

### Removed

- **cyrius.cyml** — Dropped 4 unused stdlib deps (`io`, `args`, `toml`, `hashmap`) after a usage audit; itihas's own code never calls them and no retained stdlib module needs them transitively. Verified by rebuilding the binary, test suite, benchmark suite, and consumer bundle.
- **lib/** — Stopped vendoring the Cyrius stdlib snapshot in-repo (18 files, now gitignored). `cyrius` regenerates `./lib/` from the version-pinned snapshot (`[package].cyrius`) on build/deps, so the committed copy only went stale and shadowed the matched snapshot. Resolves the `./lib/ shadows version-pinned lib/` build warning.

### Changed

- **cyrius.cyml** — Documented why `net` + `http` are retained: hoosh ships a **self-contained HTTP client** (raw syscalls + `sockaddr_in` from `net`) so itihas works independently and as a library; and `tagged` is kept for the `Result` type used transitively by `sakshi` and `net`.
- **Build hygiene** — Unreachable-fn notes are eliminated by the `CYRIUS_DCE=1` build used in CI and releases. (The large static-data `.bss` footprint — historical description strings — is left as a future optimization; restructuring it touches all 10 data modules.)

## [2.3.2] - 2026-06-03

### Added

- **dist/itihas.cyr** — Single-file distribution bundle generated by `cyrius distlib`, letting consumers (sankhya, avatara, kiran, joshua, jnana, lipi, vidya) depend on itihas without vendoring its source tree.
- **src/lib.cyr** — Library aggregator (public API surface): a doc-commented include of all 10 data modules + serial/logging/hoosh, in build order. Drives the `[lib].modules` bundle.
- **CI** — `Verify dist bundle is current` gate: regenerates the bundle and fails if `dist/itihas.cyr` is stale (out of sync with source).
- **Release** — Ships `itihas-<tag>.cyr` (the bundle) alongside the binary and source tarball, included in `SHA256SUMS`.

### Changed

- **cyrius.cyml** — Added `[lib]` section (the distlib module list) and `sakshi` to `[deps].stdlib` so the bundle resolves `SK_INFO` from the consumer's stdlib rather than a stripped path include.
- **src/logging.cyr** — Dropped the explicit `include "lib/sakshi.cyr"` (now auto-included via the `sakshi` stdlib dep), so the bundled module needs no path-relative include.
- **scripts/version-bump.sh** — Regenerates the dist bundle after a version bump (the bundle header embeds the version, so this keeps the CI freshness gate green).

## [2.3.1] - 2026-06-03

### Added

- **tests/itihas.tcyr** — Full 153-assertion integration suite runnable via `cyrius test` (module counts, lookups, filters, cross-module integration, case-insensitive lookups, chronological sort, JSON serialization, hoosh data-answer path). Built on the stdlib `assert_*` / `assert_summary` harness.
- **tests/itihas.bcyr** — 28-benchmark suite (`cyrius bench`, or build + run), relocated from `src/bench_main.cyr`.
- **bench-history.csv / benchmarks.md** — Benchmark history (CSV) + 3-point trend table, regenerated by `scripts/bench-history.sh`. The per-release benchmark gate (CLAUDE.md) now has a working Cyrius-native harness.

### Changed

- **src/main.cyr** — Slimmed from the 153-assertion inline harness to a lean smoke test (loads all 10 data modules, checks core invariants, exits non-zero on gross failure). The full suite now lives in `tests/itihas.tcyr`.
- **scripts/bench-history.sh** — Rewritten to build + run the Cyrius `.bcyr` suite and parse `bench_report` output (was Rust-era `cargo bench` / criterion parsing).
- **CI** — Added `cyrius test` (unit) and benchmark build+run steps; the smoke step now checks for `smoke: OK`.

### Removed

- **tests/test.sh, tests/test_itihas.sh** — Obsolete `cc3`-based shell runners, superseded by `cyrius test tests/itihas.tcyr`.

## [2.3.0] - 2026-06-03

### Changed

- **Toolchain** — Cyrius compiler pin bumped `4.0.0` → `6.0.50`. Builds clean with no source changes (itihas uses only in-memory `json_parse`, so the v4→v6 int-return→`Result` I/O migration does not apply).
- **Manifest** — Migrated `cyrius.toml` → `cyrius.cyml` (canonical CYML manifest, matching avatara). Version now derives from `VERSION` via `${file:VERSION}`; added `repository` field; the `[package].cyrius` pin is the single source of truth for the toolchain version.
- **CI** — Rebuilt `.github/workflows/ci.yml` on the avatara pattern: toolchain installed via `install.sh` reading the `cyrius.cyml` pin (no hardcoded version, no tarball download), `cyrius lint` hard-gate (non-cosmetic warnings fail CI), `CYRIUS_DCE=1` build, ELF-magic verification, and a docs job that checks `cyrius.cyml` plus VERSION/cyrius.cyml/CHANGELOG version consistency.
- **Release** — Rebuilt `.github/workflows/release.yml`: verifies VERSION and `cyrius.cyml` match the tag, DCE build, `git archive` source tarball with `SHA256SUMS`, and a changelog-extracted release body (replacing bare `generate_release_notes`).
- **CLAUDE.md** — Added a mandatory per-release benchmark gate (capture deltas vs the prior release, catch regressions); replaced the stale `cc3` cleanliness command with `cyrius build`; updated compiler/manifest references to 6.0.50 / `cyrius.cyml`.

### Fixed

- **main.cyr** — Collapsed 4 double-blank-line lint warnings so CI can run the avatara-identical lint hard-gate.

### Removed

- **.cyrius-toolchain** — Retired; the toolchain version now lives solely in `cyrius.cyml` `[package].cyrius`.
- **cyrius.toml** — Replaced by `cyrius.cyml`.

## [2.2.0] - 2026-04-13

### Added

- **hoosh** — LLM-powered historical queries via hoosh inference gateway (`src/hoosh.cyr`). 6 query types: `EventsInRange`, `CivilizationsAt`, `CausesOf`, `InteractionsBetween`, `FigureLookup`, `FreeForm`
- **hoosh** — `answer_from_data()` resolves queries from itihas static data without LLM inference. Returns structured `QueryResponse` with source, content, and JSON data
- **hoosh** — `llm_answer(port, question, model)` sends natural language questions to hoosh server with 6 tool definitions, parses tool calls, dispatches to data lookups or returns LLM text
- **hoosh** — `resolve_era_lookup()` handles era-specific queries by name or year (special case outside standard query types)
- **hoosh** — `parse_tool_call()` converts LLM tool call name + JSON arguments into structured query
- **hoosh** — 6 OpenAI-compatible tool definitions: `events_in_range`, `civilizations_at`, `causes_of`, `interactions_between`, `figure_lookup`, `era_lookup`
- **hoosh** — HTTP POST client for hoosh `/v1/chat/completions` endpoint with JSON request builder and response parser
- 30 new test assertions (153 total): answer_from_data for all 6 query types (found, not-found, empty), tool definition validation, parse_tool_call for all tools + unknown + missing args, resolve_era_lookup by name/year/unknown

### Changed

- **cyrius.toml** — Added `net` and `http` to stdlib deps for hoosh HTTP client

## [2.1.0] - 2026-04-13

### Added

- **util** — `str_lower()`, `str_upper()`, `str_eq_lower()` for ASCII case conversion and case-insensitive comparison (`src/util.cyr`)
- **util** — `vec_sort()` insertion sort with comparison function pointer for in-place sorting
- **serial** — JSON serialization for all 10 data types: `era_to_json()`, `civ_to_json()`, `event_to_json()`, `figure_to_json()`, `calendar_to_json()`, `campaign_to_json()` (with nested battles array), `site_to_json()`, `route_to_json()`, `causality_to_json()`, `interaction_to_json()` (`src/serial.cyr`)
- **bench** — 28-benchmark harness ported from Rust criterion using `bench.cyr` stdlib (`src/bench_main.cyr`). Cached lookups 5-6ns, filtered queries 300ns-3us
- **descriptions** — Full description strings restored to all 10 data modules from Rust v1.5.0 source. cc3 v4.0.0 raised str_data limit to 256KB (was 32KB), enabling inline descriptions
- 26 new test assertions (123 total): 11 case-insensitive lookups, 3 sort ordering, 12 JSON serialization roundtrips

### Changed

- **all modules** — All 8 `by_name()` functions now use case-insensitive matching via `str_eq_lower()` (ASCII). Matches Rust v1.5.0 behavior
- **era** — `eras_containing()` results sorted chronologically by start year
- **civilization** — `civs_active_at()` results sorted chronologically by founding year
- **event** — `events_between()` results sorted chronologically by year
- **campaign** — `campaigns_between()` results sorted chronologically by start year
- **cyrius.toml** — Compiler version updated to cc3 4.0.0; added `bench` and `toml` to stdlib deps

### Fixed

- **calendar** — `calendar_by_name()` was using `str_contains` (substring match) instead of exact match. "Hijri" incorrectly matched "Islamic (Hijri)". Now uses `str_eq_lower()` for case-insensitive exact match
- **ci** — Smoke test grep pattern `"0 failed"` did not match output format `"failed: 0"`

## [2.0.0] - 2026-04-12

### Added

- **Language port**: Entire codebase ported from Rust v1.5.0 to Cyrius (cc3 v3.6.3). 8,846 lines of Rust → 1,591 lines of Cyrius. 141KB static ELF binary (x86_64)
- **era** — 25 eras (8 global + 17 regional). `era_count()`, `era_get()`, `era_by_name()`, `eras_containing()`, `eras_by_scope()`, `eras_by_region()`. Accessor functions: `era_name()`, `era_start()`, `era_end()`, `era_region()`, `era_cat()`, `era_scope()`
- **civilization** — 53 civilizations. `civ_count()`, `civ_get()`, `civ_by_name()`, `civs_active_at()`, `civs_by_region()`. Accessors: `civ_name()`, `civ_region()`, `civ_found()`, `civ_end()`
- **event** — 105 events. `event_count()`, `event_by_name()`, `events_by_category()`, `events_by_significance()`, `events_at_year()`, `events_between()`. Accessors: `evt_name()`, `evt_year()`, `evt_cat()`, `evt_sig()`
- **figure** — 52 figures across 8 domains. `figure_count()`, `figure_by_name()`, `figures_by_domain()`. Accessors: `fig_name()`, `fig_domain()`
- **causality** — 13 causal links. `causality_count()`, `causes_of()`, `effects_of()`, `causal_chain()` (BFS traversal with depth tracking). Accessors: `caus_cause()`, `caus_effect()`, `chain_entry_name()`, `chain_entry_depth()`
- **interaction** — 21 civilization interactions. `interaction_count()`, `interactions_for()`, `interactions_between()`, `interactions_by_type()`, `interaction_neighbors()`, `influence_score()`, `region_proximity()`. Accessors: `int_civa()`, `int_civb()`, `int_type()`
- **calendar** — 8 calendar systems. `calendar_count()`, `calendar_by_name()`. Accessors: `cal_name()`, `cal_type()`, `cal_epoch()`, `cal_months()`
- **campaign** — 14 campaigns with 40+ battles. `campaign_count()`, `campaign_by_name()`, `campaigns_by_region()`, `campaigns_by_outcome()`, `campaigns_by_commander()`, `campaigns_by_civilization()`, `campaigns_active_at()`, `campaigns_between()`. Accessors: `camp_name()`, `camp_region()`, `camp_out()`, `camp_cmds()`, `camp_bella()`, `camp_bellb()`
- **site** — 32 archaeological sites. `site_count()`, `site_by_name()`, `sites_by_region()`, `sites_by_type()`, `sites_by_civilization()`, `sites_active_at()`. Accessors: `site_name()`, `site_region()`
- **trade** — 15 trade routes. `route_count()`, `route_by_name()`, `routes_by_commodity()`, `routes_by_region()`, `routes_by_type()`, `routes_by_civilization()`, `routes_active_at()`. Accessors: `rt_name()`, `rt_comm()`
- **error** — Integer error code enum (`ERR_UNKNOWN_ERA` through `ERR_CAMPAIGN_NOT_FOUND`)
- **logging** — `itihas_log_init()`, `itihas_log_init_level()` via sakshi (vendored stdlib)
- 128 functions across 13 source files
- 97-assertion test suite: 10 module counts, 30+ name lookups (found and not-found), 20+ filter queries, causal chain BFS verification, influence score symmetry, region proximity, date ordering validation (eras, civs, campaigns, sites, routes), cross-module boundary checks
- Heap-allocated structs via `store64`/`load64` with offset enum constants
- Lazy initialization with global pointer caching (same pattern as Rust `LazyLock`)
- Str auto-coercion for string parameters (Cyrius v3.6.0)
- `split_regions()` helper for comma-delimited region string parsing
- Rust v1.5.0 source preserved in `rust-old/` for reference
- Benchmark comparison document: `benchmarks-rust-v-cyrius.md`

### Removed

- **Rust toolchain** — Cargo.toml, rust-toolchain.toml, deny.toml, codecov.yml moved to `rust-old/`
- **serde** — JSON serialization/deserialization (no Cyrius equivalent yet; argonaut integration planned)
- **thiserror** — Error derive (replaced with integer error enum)
- **hoosh** — LLM query module (blocked: needs hoosh Cyrius port)
- **mcp** — MCP tool handlers (blocked: needs bote Cyrius port)
- **daimon** — Agent orchestrator integration (blocked: needs bote Cyrius port)
- **Description fields** — Stripped from event, figure, campaign, site, trade modules to fit cc3 32KB string data limit. Names, dates, enums, and all lookup functions preserved. Full descriptions to be restored when cc3 str_data is expanded or via external data loading

### Breaking

- All public types changed from Rust structs to Cyrius heap records accessed via accessor functions
- `by_name()` returns pointer (0 on not found) instead of `Result<T, ItihasError>`
- Filter functions return Cyrius vec instead of `Vec<T>`
- Civilization `traits` and `language_codes` stored as semicolon-delimited Str instead of `Vec<Cow<str>>`
- Trade route `regions`, `civilizations`, `commodities` stored as semicolon-delimited Str
- Campaign `belligerents_a`, `belligerents_b`, `commanders` stored as semicolon-delimited Str
- Case-sensitive name lookups (Rust was case-insensitive via `to_lowercase()`)
- No `Display`, `Ord`, `Serialize`, `Deserialize` trait impls

## [1.5.0] - 2026-04-03

### Added

- **mcp** — 5 MCP tool invoke handlers wired to bote `Dispatcher`: `itihas_era`, `itihas_civilization`, `itihas_event`, `itihas_figure`, `itihas_timeline`
- **mcp** — `register_handlers()` registers all handlers on an existing dispatcher; `register_all()` registers definitions + handlers in one call
- **mcp** — All tools annotated with `ToolAnnotations::read_only()` (MCP 2025-11-25)
- **mcp** — MCP-compliant response format: `content` array with `text` type, `isError` flag for errors
- **mcp** — Case-insensitive `EventCategory` and `FigureDomain` parsing from string params
- **mcp** — `itihas_era` tool schema now includes `region` parameter
- **mcp** — `itihas_event` tool schema now includes `category` parameter
- **mcp** — 24 new handler tests (was 4 definition tests, now 28 total)
- **site** — Archaeological sites module: `Site` struct, `SiteType` enum (Settlement, Temple, Burial, Fortress, Monument, Palace, Workshop, Cave, Port). 32 pre-built sites across 10 regions. `all_sites()`, `by_region()`, `by_civilization()`, `by_type()`, `active_at()`, `by_name()` lookups
- **trade** — Trade routes module: `TradeRoute` struct, `RouteType` enum (Land, Maritime, River, Mixed). 15 pre-built routes spanning 4500 years. `all_routes()`, `by_region()`, `by_commodity()`, `by_type()`, `by_civilization()`, `active_at()`, `by_name()` lookups
- **error** — `SiteNotFound` and `RouteNotFound` variants added to `ItihasError`
- 6 new criterion benchmarks for site and trade modules (25 total)
- **civilization** — Added Arab Caliphates (632–1517) to civilization data (53 total)
- **daimon** — Daimon agent orchestrator integration (`daimon` feature): `mcp::daimon::register_tools()` registers all itihas tools on a `McpHostRegistry`, `host_tool_descriptions()` converts tool definitions to daimon's `McpToolDescription` format, `invoke()` dispatches tool calls and returns typed `McpToolResult`
- 7 new daimon integration tests (registry registration, tool invocation, error propagation, serde roundtrip)
- **hoosh-llm** — Natural language historical queries via LLM inference (`hoosh-llm` feature): `hoosh::llm::answer()` sends NL questions to hoosh with 6 tool definitions, LLM selects the appropriate lookup, itihas resolves from data. `resolve_era_lookup()` for era-specific queries. Falls back to pure LLM generation for questions outside data coverage
- 12 new hoosh-llm tests (tool def validation, tool call parsing, era resolution, missing args handling)
- **campaign** — Military campaigns module: `Campaign` struct, `Battle` struct, `CampaignOutcome` enum (Victory, Defeat, Stalemate, Treaty, Inconclusive). 14 pre-built campaigns with 40+ battles spanning 2500 years. `all_campaigns()`, `by_region()`, `by_civilization()`, `by_commander()`, `by_outcome()`, `active_at()`, `campaigns_between()`, `by_name()` lookups. `Ord` impl for chronological sorting
- **error** — `CampaignNotFound` variant added to `ItihasError`
- 3 new criterion benchmarks for campaign module (28 total)
- **docs/sources/** — Per-module bibliography with 30+ authoritative references (Stearns, Bickerman, Dershowitz & Reingold, Keegan, Renfrew & Bahn, Frankopan, etc.)
- Inline `# Sources` doc comments on all 10 data modules linking to full bibliography

### Fixed

- **era** — Age of Enlightenment start_year 1600→1648 (1600 was Scientific Revolution, not Enlightenment)
- **era** — Renaissance end_year 1600→1610 (closes gap with Enlightenment for early 17th-century events)
- **era** — Mesoamerican Classic category Classical→Medieval (250–900 CE is Medieval by EraCategory definition)
- **civilization** — Maya peak_era "Classical Antiquity"→"Mesoamerican Classic" (has its own defined era)
- **civilization** — Mali Empire script "N'ko script"→"Arabic script" (N'Ko invented 1949, centuries after Mali)
- **event** — Decline of Indus Valley year -1300→-1900 (major cities abandoned by ~1700 BCE)
- **event** — Assassination of Julius Caesar: category War→Revolution, significance Regional→Continental
- **event** — French Revolution era "Industrial Age"→"Age of Enlightenment" (quintessential Enlightenment event)
- **event** — Fall of Constantinople: added Byzantine Empire to civilizations_involved
- **event** — Gutenberg Printing Press: added Holy Roman Empire to civilizations_involved
- **event** — Rise of Islam: added Arab Caliphates to civilizations_involved (was empty)
- **event** — First Crusade: added Holy Roman Empire to civilizations_involved
- **event** — Punic Wars: civilizations_involved "Phoenicia"→"Carthage" (Carthage was independent by 264 BCE)
- **event** — Norman Conquest: civilizations_involved "Viking/Norse"→"Normandy" (Normans ≠ Vikings by 1066)
- **event** — Polynesian Settlement of NZ: civilizations_involved "Tonga Empire"→"Polynesia" (settlers from eastern Polynesia)
- **event** — Tokugawa Shogunate era "Age of Enlightenment"→"Renaissance" (1603 predates Enlightenment)
- **figure** — Julius Caesar civilization "Roman Empire"→"Roman Republic" (died 44 BCE; Empire starts 27 BCE)
- **figure** — Hannibal Barca civilization "Phoenicia"→"Carthage" (Carthaginian, not generically Phoenician)
- **figure** — Muhammad civilization "Rashidun Caliphate"→"Quraysh" (Caliphate formed after his death)
- **figure** — Guru Nanak civilization "Mughal Empire"→"Lodhi Sultanate" (born 1469; Mughals founded 1526)
- **figure** — Aryabhata description: corrected overclaim about pioneering zero (Brahmagupta formalized zero)
- **site** — Lalibela civilization "Kingdom of Aksum"→"Zagwe Dynasty" (Aksum fell centuries before construction)
- **site** — Mycenae civilization "Ancient Greece"→"Mycenaean Civilization" (predates Classical Greece)
- **trade** — Trans-Saharan Trade start_year -500→300 (organized camel caravan trade began ~3rd century CE)
- **interaction** — Mali Empire trade partner "Ancient Egypt"→"Arab Caliphates" (Ancient Egypt ended 30 BCE)
- **causality** — Fall of Western Rome→Rise of Islam: strength Moderate→Weak (Byzantine-Sassanid exhaustion was the more direct factor)
- **calendar** — Egyptian Civil calendar type Solar→Fixed (no leap correction; drifted against solar year)
- **campaign** — Alexander belligerents_a "Ancient Greece"→"Macedon" (Macedonian army, not a Greek coalition)
- **campaign** — Second Punic War belligerents_b "Roman Empire"→"Roman Republic" (Republic, not Empire in 218 BCE)
- **campaign** — Gallic Wars belligerents_a "Roman Empire"→"Roman Republic" (Republic, not Empire in 58 BCE)
- **campaign** — Russo-Japanese War outcome Treaty→Victory (decisive Japanese win despite treaty ending)
- **campaign** — Russo-Japanese War: added Aleksei Kuropatkin to commanders (was missing Russian side)
- **no_std** — `format!` macro not in scope in `site`, `trade`, `campaign` test modules under `--no-default-features` (added `use alloc::format`)
- **deny.toml** — Added BSD-2/3-Clause, ISC, CDLA-Permissive-2.0 to allowed licenses; wildcards allowed for path deps
- **Makefile** — `check` target now includes `doc` and `deny`; `test-all` runs `cargo test --no-default-features`
- **docs** — Updated stale counts in README, CLAUDE.md, SECURITY.md, architecture overview to reflect v1.5.0 state
- **Cargo.toml** — Switched bote and hoosh from path deps to crates.io versions (0.92, 1.2)

## [1.0.1] - 2026-04-01

### Fixed

- **docs** — README now documents all 8 modules (was missing `causality` and `interaction`), all 5 features (was missing `hoosh` and `mcp`), and correct dependency version for v1
- **docs** — Roadmap reflects current state; completed items removed from backlog

## [1.0.0] - 2026-04-01

### Added

- **era** — Historical periods with date ranges and civilizational phases. `Era` struct, `EraCategory` enum (Ancient, Classical, Medieval, EarlyModern, Modern, Contemporary), `EraScope` enum (Global, Regional). 25 pre-built eras: 8 global, 9 Chinese dynasties (Xia–Qing), 5 Indian periods (Vedic–Mughal), 3 Mesoamerican periods. `eras_containing()`, `by_scope()`, `by_region()`, `by_name()` lookups. `Ord` impl for chronological sorting
- **civilization** — Major civilizations with geographic extent, peak period, key traits. 52 pre-built civilizations across all inhabited continents. `by_region()`, `active_at()`, `by_name()` lookups
- **event** — Structured historical events. `Event` struct, `EventCategory` enum (War, Treaty, Discovery, Invention, Revolution, Migration, Founding, Collapse), `EventSignificance` enum (Local, Regional, Continental, Global). 105 pre-built world events. `events_between()`, `by_category()`, `at_year()`, `by_significance()`, `by_name()` lookups. `Ord` impl for chronological sorting
- **causality** — Causal links between events with `CausalStrength` enum (Weak, Moderate, Strong, Direct). 13 pre-built causal links. `causes_of()`, `effects_of()`, `chain()` traversal
- **interaction** — Civilization interaction graph with `InteractionType` enum (Trade, War, CulturalExchange, Alliance, Conquest, Diplomacy). 22 pre-built interactions. `interactions_for()`, `by_type()`, `between()`, `neighbors()`, `influence_score()`, `region_proximity()` queries
- **calendar** — Calendar system metadata. `CalendarSystem` struct, `CalendarType` enum (Solar, Lunar, Lunisolar, Fixed). 8 pre-built calendar systems. `by_name()` lookup
- **figure** — Historical figures with era/civilization context. `Figure` struct, `FigureDomain` enum (Ruler, Philosopher, Scientist, Artist, Military, Religious, Explorer, Inventor). 52 pre-built figures. `by_domain()`, `by_name()` lookups
- **error** — `ItihasError` with variants: UnknownEra, UnknownCivilization, UnknownCalendar, InvalidYear, EventNotFound, FigureNotFound
- **hoosh** — Query types and data-driven answer resolution for AI integration (`hoosh` feature)
- **mcp** — MCP tool definitions stub for future bote integration (`mcp` feature)
- **logging** — Optional `ITIHAS_LOG` env-based tracing init (`logging` feature)
- `no_std` support via `alloc`/`core`; `std` feature adds `LazyLock` caching
- All public types: `Display` impls, `Cow<'static, str>` for zero-alloc statics, full serde roundtrip, `#[non_exhaustive]` on all enums and structs, `#[must_use]` on all pure functions

[Unreleased]: https://github.com/MacCracken/itihas/compare/v2.4.0...HEAD
[2.4.0]: https://github.com/MacCracken/itihas/compare/v2.3.5...v2.4.0
[2.3.5]: https://github.com/MacCracken/itihas/compare/v2.3.4...v2.3.5
[2.3.4]: https://github.com/MacCracken/itihas/compare/v2.3.3...v2.3.4
[2.3.3]: https://github.com/MacCracken/itihas/compare/v2.3.2...v2.3.3
[2.3.2]: https://github.com/MacCracken/itihas/compare/v2.3.1...v2.3.2
[2.3.1]: https://github.com/MacCracken/itihas/compare/v2.3.0...v2.3.1
[2.3.0]: https://github.com/MacCracken/itihas/compare/v2.2.0...v2.3.0
[2.2.0]: https://github.com/MacCracken/itihas/compare/v2.1.0...v2.2.0
[2.1.0]: https://github.com/MacCracken/itihas/compare/v2.0.0...v2.1.0
[2.0.0]: https://github.com/MacCracken/itihas/compare/v1.5.0...v2.0.0
[1.5.0]: https://github.com/MacCracken/itihas/compare/v1.0.1...v1.5.0
[1.0.1]: https://github.com/MacCracken/itihas/compare/v1.0.0...v1.0.1
[1.0.0]: https://github.com/MacCracken/itihas/releases/tag/v1.0.0
