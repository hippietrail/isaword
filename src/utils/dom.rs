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

/// Find the body element's index in the root's children
/// 
/// Since different websites have different amounts of whitespace/comments,
/// the body element can be at different indices. This helper finds it dynamically.
pub fn find_body_index(html: &Html) -> Option<usize> {
    let root = html.root_element();
    for (i, child) in root.children().enumerate() {
        if let Some(el) = ElementRef::wrap(child) {
            if el.value().name() == "body" {
                return Some(i);
            }
        }
    }
    None
}

/// DOM tree walker - navigates HTML structure with assertions
/// 
/// This is a Rust port of the TypeScript domStroll function from:
/// /Users/hippietrail/hippiebot.js/ute/dom.ts
/// 
/// Takes a sequence of (index, tag_name, options) tuples and walks the DOM tree,
/// validating structure at each step. Returns the final ElementRef or error.
/// 
/// Key difference from TypeScript:
/// - TS domStroll receives an array of DomNode[] (the root-level children)
/// - Rust domstroll receives Html, but internally works with children arrays
/// - Both iterate through ALL nodes (text, comments, elements) at same indices
pub fn domstroll<'a>(
    site: &str,
    debug: bool,
    html: &'a Html,
    path: &[(usize, &str, Option<DomOpts>)],
) -> Result<ElementRef<'a>, String> {
    // Start with the root element and its children
    let root = html.root_element();
    let mut current_element = root;

    // If path is empty, return root
    if path.is_empty() {
        return Ok(current_element);
    }

    for (step, (index, expected_tag, opts_maybe)) in path.iter().enumerate() {
        let opts = opts_maybe.clone().unwrap_or_default();

        // Get ALL children (including text nodes, comments, etc) - matches TypeScript behavior
        let all_children: Vec<_> = current_element.children().collect();

        if debug {
            print_children_debug_info(site, step, &current_element, &all_children);
        }

        // Select the child at the specified index
        let child_node = all_children
            .get(*index)
            .ok_or_else(|| format!("[domStroll] {}#{} not a node (index {} out of bounds)", site, step, index))?;

        // Try to wrap as ElementRef - only elements are relevant
        let node = ElementRef::wrap(*child_node)
            .ok_or_else(|| format!("[domStroll] {}#{} child at index {} is not an element node", site, step, index))?;

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

        // Move to this node for next iteration
        current_element = node.clone();
    }

    Ok(current_element)
}

fn print_children_debug_info(
    site: &str,
    step: usize,
    parent: &ElementRef,
    children: &[impl std::fmt::Debug],
) {
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
        .map(|(i, _)| {
            // Can't introspect the generic impl std::fmt::Debug, just show index
            format!("[{}]", i)
        })
        .collect::<Vec<_>>()
        .join(" ");

    println!("[domStroll] {}#{} {} has {} children: {}", site, step, parent_str, children.len(), children_str);
}
