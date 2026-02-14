# isaword - Rust CLI for Dictionary Word Checking

A fast Rust CLI tool that checks if a word exists in 13 different dictionaries in parallel.

This is a direct port of the `/isaword2` Discord bot slash command from:
- [hippiebot.js](../hippiebot.js/commands/isaword.js)
- [lyre](../lyre/commands/isaword.js)

Both Discord bot implementations are identical - this Rust version maintains the exact same logic.

## Features

- **13 Dictionary Checkers** in parallel:
  - **Professional/Authoritative**: American Heritage, Cambridge, Chambers, Dictionary.com, Etymonline, Longman, Merriam-Webster, OED, Oxford Learners, Wordnet, Wordnik
  - **Community-Contributed**: Wiktionary, Urban Dictionary

- **Mixed Query Methods**:
  - JSON APIs: Urban Dictionary, OED, Wiktionary
  - HTML DOM Scraping: American Heritage, Cambridge, Chambers, Dictionary.com, Etymonline, Longman, Merriam-Webster, Oxford Learners, Wordnet, Wordnik
  - Redirect Detection: Cambridge

- **Smart Output Formatting**:
  - Distinguishes professional vs. community dictionaries
  - Handles edge cases: found nowhere, found everywhere, found in one only, etc.
  - Human-friendly list formatting

## Usage

```bash
cargo run --release -- <word>

# Examples:
cargo run --release -- hello
cargo run --release -- xyznotaword
```

## Output Example

```
[ISAWORD] in: Cambridge, Wiktionary, and Urban Dictionary
[ISAWORD] in 1 professional dictionaries vs 2 community dictionaries
[ISAWORD] not in: Longman
[ISAWORD] null: American Heritage, Chambers, Dictionary.com, ...

'hello' is in Cambridge, Wiktionary, and Urban Dictionary but not in Longman
```

## Architecture

### Core Components

- **Earl** (`src/utils/earl.rs`) - HTTP client wrapper
  - URL building with query parameters
  - JSON/HTML fetching
  - Redirect checking
  - Custom header support (for Wikimedia APIs)

- **domStroll** (`src/utils/dom.rs`) - DOM tree navigator
  - Index-based element traversal with assertions
  - Tag name, id, class validation
  - Optional element matching for conditional paths
  - Exact replication of TypeScript version

- **Checkers** (`src/checkers/`) - 13 Dictionary implementations
  - Each returns `Option<bool>` (found/not found/error)
  - Parallel execution via `tokio::join!`
  - Consistent error handling

- **Integration** (`src/lib.rs`)
  - `checker_all()` - Concurrent execution of all checkers
  - `format_and_print_results()` - Output formatting with branching logic

### Implementation Decisions

**Why Tokio?**
- Required by reqwest's dependencies (hyper, hickory-dns)
- Investigated smol but found tokio unavoidable via transitive dependencies
- CLI startup overhead negligible (~100ms)
- See `isaword-8uv` beads issue for investigation details

**Why Wrapper Pattern?**
- Makes TypeScript/Rust comparison during debugging trivial
- Function signatures nearly identical between languages
- Easy to test individual checkers
- Graceful error handling (returns None on failures, logs them)

## Known Issues & Limitations

### Website Structure Changes
Many dictionary websites have changed their HTML structure since the TypeScript version was written:
- ❌ American Heritage, Chambers, Dictionary.com, Merriam-Webster, Oxford Learners, Wordnet, Wordnik (DOM paths outdated)
- ✅ Cambridge, Longman (still working)
- ✅ Urban Dictionary, Wiktionary, OED (APIs still working)

These return `None` (unknown) when navigation fails. **This is expected behavior** - websites evolve. To update a checker, modify its `domstroll()` path specification.

### OED API
The OED autocomplete endpoint returns 403 Forbidden. Likely disabled automated access or requires credentials. The implementation is correct but unused.

## Building

```bash
# Debug build
cargo build

# Release build (optimized)
cargo build --release

# Run tests
cargo test
```

## Dependencies

- **reqwest** - HTTP client
- **tokio** - Async runtime
- **scraper** - HTML parsing
- **serde/serde_json** - JSON serialization
- **url** - URL parsing
- **futures** - Async utilities

## Code Structure

```
src/
├── main.rs           # CLI entry point
├── lib.rs            # Main logic, checker_all(), formatting
├── utils/
│   ├── earl.rs      # HTTP client wrapper
│   ├── dom.rs       # DOM navigator
│   ├── format.rs    # List formatting
│   └── mod.rs
└── checkers/
    ├── mod.rs           # CheckerResult type
    ├── ahd.rs          # American Heritage
    ├── cambridge.rs    # Cambridge
    ├── chambers.rs     # Chambers
    ├── dictcom.rs      # Dictionary.com
    ├── etymonline.rs   # Etymonline wrapper
    ├── etym.rs         # Etymonline scraper
    ├── longman.rs      # Longman
    ├── mw.rs           # Merriam-Webster
    ├── oed.rs          # Oxford English Dictionary
    ├── oxfordlearners.rs # Oxford Learners
    ├── wordnet.rs      # WordNet
    ├── wordnik.rs      # Wordnik
    ├── urban.rs        # Urban Dictionary
    └── wikt.rs         # Wiktionary
```

## Testing

Each checker can be tested independently by updating `src/main.rs` to call it directly, or by examining the logs when running `checker_all()`.

Expected behavior:
- All requests include proper User-Agent headers for Wikimedia APIs
- Timeouts/network errors log but don't crash
- DOM navigation errors log error path and return `None`
- Output always matches TypeScript version format exactly

## References

- Original TypeScript implementations:
  - [hippiebot.js isaword.js](../hippiebot.js/commands/isaword.js)
  - [lyre isaword.js](../lyre/commands/isaword.js)
  - [hippiebot.js ute/earl.ts](../hippiebot.js/ute/earl.ts)
  - [hippiebot.js ute/dom.ts](../hippiebot.js/ute/dom.ts)

- Beads issues:
  - isaword-8uv: Tokio requirement investigation
  - isaword-026: DOM tree traversal differences (gotcha)
  - isaword-5pm: Website structure changes
  - isaword-e27: Current DOM structure issues

## License

Same as original hippiebot projects (if applicable).
