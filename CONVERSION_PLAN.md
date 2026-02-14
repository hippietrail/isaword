# isaword2 → Rust CLI Conversion Plan

## Summary
Both Discord bots have **identical** implementations of `/isaword2`. Converting to Rust CLI involves:
- Translating TypeScript logic 1:1 to Rust
- Replicating web scraping with equivalent libraries
- Maintaining identical dictionary detection logic
- CLI input/output instead of Discord interaction

---

## Architecture Overview

### Current TypeScript Structure
```
isaword2 (Discord slash command)
├── Main logic: query 14 dictionaries in parallel
├── Result aggregation: true/false/null per dictionary
├── Output formatting: human-friendly message with statistics
└── Dependencies:
    ├── Earl (HTTP client + HTML parser wrapper)
    ├── dom.ts (DomStroll - DOM tree navigator)
    ├── amis.ts (humanFriendlyListFormatter)
    ├── wikt.ts (Wiktionary API)
    └── etym.ts (Etymonline scraper)
```

### Target Rust Structure
```
isaword (CLI)
├── main.rs (CLI entry point)
├── lib.rs (core logic)
├── checkers/ (14 dictionary checker modules)
├── utils/
│   ├── http.rs (HTTP requests)
│   ├── dom.rs (DOM navigation)
│   ├── format.rs (output formatting)
│   └── url.rs (URL building)
└── Cargo.toml
```

---

## Detailed Conversion Map

### 1. **HTTP Layer** (Earl class)
**TypeScript**: `earl.ts` - Wrapper around `fetch()` + `html-dom-parser`

**Rust equivalent**:
- `reqwest` crate for HTTP (with feature `"cookies"` for redirect handling)
- `html5ever` + `select.rs` OR `scraper` crate for DOM parsing
- Custom `struct Earl` with methods:
  - `new(origin, pathname, search_params)` → constructor pattern
  - `set_last_path_segment()`, `set_pathname()`, etc.
  - `fetch_json()` → returns serde_json::Value
  - `fetch_dom()` → returns scraper::Html
  - `fetch_text()` → returns String
  - `check_redirect()` → HEAD request, returns Option<bool>

**Challenges**:
- Rust HTTP is async/await by default
- Need `tokio` runtime for concurrency
- Reqwest needs feature flags for JSON

### 2. **DOM Navigation** (domStroll function)
**TypeScript**: `dom.ts` - Tree walking with assertions on tag name, id, class

**Rust equivalent**:
- Use `scraper::Selector` for CSS selectors
- Custom `domstroll()` function that:
  - Takes a vector of search tuples: `[(index, tag_name, opts), ...]`
  - Each step traverses children[n] and validates tag/id/class
  - Returns Result<Element, String> with descriptive errors
  - Same debug output as TypeScript

**Challenges**:
- `scraper` Selector API is different from raw DOM tree walking
- Need careful index-based traversal (not CSS selectors)
- Error messages must match for compatibility

### 3. **Dictionary Checkers** (14 async functions)
Each checker has identical pattern:
```typescript
async function name(word: string): Promise<boolean | null>
```

**Rust equivalent**:
```rust
async fn name(word: &str) -> Option<bool>
```

Creates `struct CheckerResult`:
```rust
struct CheckerResult {
    name: &'static str,
    result: Option<bool>,
    is_community: bool,
}
```

**The 14 checkers** (in order from isaword.js):
1. `ahd()` - American Heritage Dict (DOM scrape: div#results structure)
2. `cambridge()` - Cambridge Dict (check redirect)
3. `chambers()` - Chambers (DOM: p.message child count)
4. `dictcom()` - Dictionary.com (DOM: main children length)
5. `etymonline()` - Etymonline (uses `etym()` helper)
6. `longman()` - Longman (DOM: head.metadata class)
7. `mw()` - Merriam-Webster (DOM: body class check)
8. `oed()` - Oxford English Dict (JSON API: array.some())
9. `oxfordlearners()` - Oxford Learners (DOM: deep traversal)
10. `wordnet()` - WordNet (DOM: tag sequence check)
11. `wordnik()` - Wordnik (DOM: guts.active structure)
12. `urban()` - Urban Dictionary (JSON API: list.length)
13. `wikt()` - Wiktionary (API call with lang param)
14. **Collins** - COMMENTED OUT (JavaScript-heavy, skip for v1)

### 4. **Helper Functions**

#### `humanFriendlyListFormatter()`
**TypeScript**: Simple string formatting
```typescript
// ["a", "b", "c"] with 'and' → "a, b, and c"
```

**Rust**: 
```rust
fn format_list(items: &[&str], conjunction: &str) -> String
```

#### `wikt()` - Wiktionary API
**TypeScript**: Queries Wiktionary API for language, returns 0/1/null

**Rust**: Similar async function with reqwest + serde

#### `etym()` - Etymonline scraper
**TypeScript**: Complex DOM traversal + substring matching

**Rust**: Replicate exact DOM navigation with `scraper`

### 5. **Main Logic** (isaword function)

**TypeScript**:
```typescript
const results = await Promise.all(dictionaries.map(d => d[1](word)));
```

**Rust**:
```rust
let futures = checkers.iter().map(|c| c.check(word));
let results = futures::future::join_all(futures).await;
```

**Output logic** (identical):
- Count ins/notins/nulls
- Count pro vs community dicts in results
- Format response message with branching logic:
  - "found in all" / "found in none"
  - "found in X only" (pro vs community distinction)
  - "found in these but not those"
  - "found in these at least..." (if nulls present)

### 6. **Logging**
**TypeScript**: `console.log([ISAWORD/checker] ...)`

**Rust**: `println!()` or `log::info!()`

Maintain exact same format for debugging/transparency.

---

## Implementation Steps

### Phase 1: Project Setup
- [ ] `cargo init isaword --name isaword`
- [ ] Add Cargo.toml dependencies:
  ```toml
  [dependencies]
  reqwest = { version = "0.11", features = ["json"] }
  tokio = { version = "1", features = ["full"] }
  scraper = "0.17"
  serde = { version = "1", features = ["derive"] }
  serde_json = "1"
  url = "2"
  ```
- [ ] Set up clap for CLI args (word: required positional)
- [ ] README with usage examples

### Phase 2: Core Utilities
- [ ] `lib.rs` - main entry point, expose checker functions
- [ ] `utils/http.rs` - Earl struct + methods
- [ ] `utils/dom.rs` - domstroll function
- [ ] `utils/format.rs` - humanFriendlyListFormatter
- [ ] Test HTTP + parsing with one simple checker

### Phase 3: Dictionary Checkers (batch)
Implement in groups by complexity:
1. **Simple (API-based)**: urban(), oed(), wikt()
2. **Simple (Redirect)**: cambridge()
3. **Medium (DOM, 1-3 steps)**: chambers(), dictcom(), longman()
4. **Complex (Deep DOM)**: ahd(), mw(), oxfordlearners(), wordnet(), wordnik()
5. **Complex (Helper)**: etymonline(), etym()

### Phase 4: Main Logic & Integration
- [ ] `main.rs` - CLI parsing, call checker_all()
- [ ] `checker_all()` - parallel execution of all 13 checkers
- [ ] Result aggregation & formatting
- [ ] Error handling & user-friendly messages

### Phase 5: Testing & Validation
- [ ] Unit tests for each checker (mock HTTP responses)
- [ ] Integration test: compare output with TypeScript for same words
- [ ] Test edge cases: unfound words, API errors, network timeouts

---

## Potential Challenges & Solutions

| Challenge | Solution |
|-----------|----------|
| **Async/await complexity** | Use tokio + futures crate; hide behind simple API |
| **DOM parsing differences** | Test each scraper against live websites; document selectors |
| **Error handling** | Match TS behavior: return `null` on errors, log them |
| **Timeouts** | Add configurable timeout per request (default 5s) |
| **User-Agent blocking** | Add User-Agent header to requests |
| **Website structure changes** | Same risk as TS version; validate against live sites |

---

## Success Criteria

- ✅ CLI accepts single word argument: `isaword test`
- ✅ Returns human-readable message matching TS version
- ✅ All 13 checkers (excluding Collins) implemented & working
- ✅ Parallel execution (same concurrency as TS)
- ✅ Exact same output format for identical test words
- ✅ Error messages logged with same format
- ✅ No external config needed (self-contained)

---

## Commands to Generate

```bash
# Run with word
cargo run -- myword

# Example:
cargo run -- hello
# Output: 'hello' is in 13 professional dictionaries at least...

cargo run -- xyzabc
# Output: No sign of 'xyzabc' in any dictionary I checked!
```

---

## Differences from TypeScript Version

1. **No Discord integration** - pure CLI input/output
2. **No dotenv** - no .env file needed (URLs are hardcoded)
3. **Single-shot execution** - exits after word check (no REPL)
4. **Logging format** - same structure, using `println!()` instead of `console.log()`
5. **Error handling** - compile-time safety vs runtime exceptions
6. **Startup time** - Rust binary faster than Node.js
