// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Shared dependency ordering for value and kind inference.
//! Edges point from a declaration to what it needs. Tarjan emits dependencies
//! first, preserving the supplied traversal order without hash-order choices.
//! Workflow: language-features-chirho/declaration-kinds-chirho.

#[cfg(test)]
mod tests_chirho;

/// O(vertices + edges) work and heap storage. Explicit DFS frames keep a long
/// dependency chain from exhausting the Rust call stack.
pub(crate) fn dependency_groups_chirho(edges_chirho: &[Vec<usize>]) -> Vec<Vec<usize>> {
    let count_chirho = edges_chirho.len();
    let mut index_chirho = 0;
    let mut indices_chirho = vec![None; count_chirho];
    let mut lowlinks_chirho = vec![0; count_chirho];
    let mut on_stack_chirho = vec![false; count_chirho];
    let mut stack_chirho = Vec::new();
    let mut frames_chirho = Vec::new();
    let mut groups_chirho = Vec::new();

    for root_chirho in 0..count_chirho {
        if indices_chirho[root_chirho].is_some() {
            continue;
        }
        frames_chirho.push((root_chirho, 0));
        while let Some((vertex_chirho, next_edge_chirho)) = frames_chirho.last_mut() {
            let vertex_chirho = *vertex_chirho;
            if indices_chirho[vertex_chirho].is_none() {
                indices_chirho[vertex_chirho] = Some(index_chirho);
                lowlinks_chirho[vertex_chirho] = index_chirho;
                index_chirho += 1;
                stack_chirho.push(vertex_chirho);
                on_stack_chirho[vertex_chirho] = true;
            }
            if let Some(&child_chirho) = edges_chirho[vertex_chirho].get(*next_edge_chirho) {
                *next_edge_chirho += 1;
                if let Some(child_index_chirho) = indices_chirho[child_chirho] {
                    if on_stack_chirho[child_chirho] {
                        lowlinks_chirho[vertex_chirho] =
                            lowlinks_chirho[vertex_chirho].min(child_index_chirho);
                    }
                } else {
                    frames_chirho.push((child_chirho, 0));
                }
                continue;
            }

            frames_chirho.pop();
            if Some(lowlinks_chirho[vertex_chirho]) == indices_chirho[vertex_chirho] {
                let mut group_chirho = Vec::new();
                loop {
                    let member_chirho = stack_chirho.pop().expect("active dependency vertex");
                    on_stack_chirho[member_chirho] = false;
                    group_chirho.push(member_chirho);
                    if member_chirho == vertex_chirho {
                        break;
                    }
                }
                groups_chirho.push(group_chirho);
            }
            if let Some(&(parent_chirho, _)) = frames_chirho.last() {
                lowlinks_chirho[parent_chirho] =
                    lowlinks_chirho[parent_chirho].min(lowlinks_chirho[vertex_chirho]);
            }
        }
    }
    groups_chirho
}
