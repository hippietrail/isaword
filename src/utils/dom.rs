use scraper::{Html};
use scraper::element_ref::ElementRef;

/// Options for DOM element validation in domstroll
/// 
/// Mirrors the options object from TypeScript domStroll:
/// /Users/hippietrail/hippiebot.js/ute/dom.ts
#[derive(Debug, Clone)]
pub struct DomOpts {
    pub id: Option<String>,
    pub cls: Option<String>,
    pub optional: bool,
    pub debug: bool,
}

impl Default for DomOpts {
    fn default() -> Self {
        DomOpts {
            id: None,
            cls: None,
            optional: false,
            debug: false,
        }
    }
}

/// DOM tree walker - navigates HTML structure with assertions
/// 
/// This is a Rust port of the TypeScript domStroll function from:
/// /Users/hippietrail/hippiebot.js/ute/dom.ts
/// 
/// Takes a sequence of (index, tag_name, options) tuples and walks the DOM tree,
/// validating structure at each step. Returns the final ElementRef or error.
/// 
/// NOTE: Differs slightly from TypeScript implementation:
/// - TS uses raw DOM tree children array with indices
/// - Rust scraper doesn't expose raw tree nodes, so we filter element children and index them
/// - This is functionally equivalent but may behave differently if HTML has unusual structure
/// - See issue isaword-026 for details on this gotcha
pub fn domstroll<'a>(
    site: &str,
    debug: bool,
    html: &'a Html,
    path: &[(usize, &str, Option<DomOpts>)],
) -> Result<ElementRef<'a>, String> {
    let mut current = scraper::element_ref::ElementRef::wrap(*html.root_element())
        .ok_or_else(|| "[domStroll] failed to get root element".to_string())?;

    for (step, (index, expected_tag, opts_maybe)) in path.iter().enumerate() {
        let opts = opts_maybe.clone().unwrap_or_default();

        // Get children and select nth child
        let children: Vec<ElementRef> = current
            .children()
            .filter_map(|child| ElementRef::wrap(child))
            .collect();

        if debug {
            print_children_for_step(site, step, &current, &children);
        }

        let node = children
            .get(*index)
            .ok_or_else(|| format!("[domStroll] {}#{} not a node (index {} out of bounds)", site, step, index))?;

        // Validate tag name
        if node.value().name() != *expected_tag {
            if opts.optional {
                return Ok(node.clone());
            }
            return Err(format!(
                "[domStroll] {}#{} not <{}> (got <{}>)",
                site,
                step,
                expected_tag,
                node.value().name()
            ));
        }

        // Validate id if specified
        if let Some(ref expected_id) = opts.id {
            let actual_id = node
                .value()
                .attr("id")
                .ok_or_else(|| format!("[domStroll] {}#{} node has no id attribute", site, step))?;
            if actual_id != expected_id {
                if opts.optional {
                    return Ok(node.clone());
                }
                return Err(format!(
                    "[domStroll] {}#{} node id is not {} (got {})",
                    site, step, expected_id, actual_id
                ));
            }
        }

        // Validate class if specified
        if let Some(ref expected_cls) = opts.cls {
            let actual_classes = node.value().attr("class").unwrap_or("");
            if !actual_classes.contains(expected_cls) {
                if opts.optional {
                    return Ok(node.clone());
                }
                return Err(format!(
                    "<{}> has no .{} class (got: {})",
                    expected_tag, expected_cls, actual_classes
                ));
            }
        }

        current = node.clone();
    }

    Ok(current)
}

fn print_children_for_step(site: &str, step: usize, parent: &ElementRef, children: &[ElementRef]) {
    let parent_name = parent.value().name();
    let parent_id = parent.value().attr("id").unwrap_or("");
    let parent_class = parent.value().attr("class").unwrap_or("");

    let parent_str = if !parent_id.is_empty() {
        format!("{}#{}", parent_name, parent_id)
    } else if !parent_class.is_empty() {
        let classes = parent_class.split_whitespace().collect::<Vec<_>>().join(".");
        format!("{}.{}", parent_name, classes)
    } else {
        parent_name.to_string()
    };

    let children_str = children
        .iter()
        .enumerate()
        .map(|(i, child)| {
            let name = child.value().name();
            let id = child.value().attr("id").unwrap_or("");
            let class = child.value().attr("class").unwrap_or("");

            let mut s = format!("[{}]<{}", i, name);
            if !id.is_empty() {
                s.push_str(&format!("#{}", id));
            }
            if !class.is_empty() {
                let classes = class.split_whitespace().collect::<Vec<_>>().join(".");
                s.push_str(&format!(".{}", classes));
            }
            s.push('>');
            s
        })
        .collect::<Vec<_>>()
        .join(" ");

    println!("[domStroll] {}#{} {} -> {}", site, step, parent_str, children_str);
}
