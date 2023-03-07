use crate::{Node, Cache};

/// Prints a debug representation of the computed layout for a tree of nodes, starting with the given root node.
pub fn print_node<N: Node>(
    node: &N,
    cache: &impl Cache<CacheKey = N::CacheKey>,
    tree: &N::Tree,
    is_root: bool,
    has_sibling: bool,
    lines_string: String,
) 
where N::CacheKey: std::fmt::Display
{
    let entity = node.key();

    let fork_string = if is_root {
        "│"
    } else if has_sibling {
        "├───┤"
    } else {
        "└───┤"
    };
    println!(
        "{lines}{fork}{id}| {x:#3} {y:#3} {w:#3} {h:#3}│",
        lines = lines_string,
        fork = fork_string,
        id = entity,
        x = cache.posx(entity),
        y = cache.posx(entity),
        w = cache.width(entity),
        h = cache.height(entity),
    );
    let bar = if is_root {
        ""
    } else if has_sibling {
        "│   "
    } else {
        "    "
    };
    let new_string = lines_string + bar;

    let mut child_iter = node.children(tree).peekable();

    while let Some(child) = child_iter.next() {
        let has_sibling = child_iter.peek().is_some();
        print_node(child, cache, tree, false, has_sibling, new_string.clone());
    }
}
