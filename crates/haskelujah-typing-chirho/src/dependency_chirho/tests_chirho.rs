// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

use super::dependency_groups_chirho;

#[test]
fn groups_are_exactly_mutual_reachability_and_dependencies_first_chirho() {
    // Exhaust every directed graph on three vertices, including self edges.
    // The oracle is reachability, independent of the SCC implementation.
    for mask_chirho in 0..(1usize << 9) {
        let edges_chirho: Vec<Vec<usize>> = (0..3)
            .map(|from_chirho| {
                (0..3)
                    .filter(|to_chirho| mask_chirho & (1 << (from_chirho * 3 + to_chirho)) != 0)
                    .collect()
            })
            .collect();
        let mut reachable_chirho = [[false; 3]; 3];
        for (from_chirho, targets_chirho) in edges_chirho.iter().enumerate() {
            reachable_chirho[from_chirho][from_chirho] = true;
            for &to_chirho in targets_chirho {
                reachable_chirho[from_chirho][to_chirho] = true;
            }
        }
        for via_chirho in 0..3 {
            for from_chirho in 0..3 {
                for to_chirho in 0..3 {
                    reachable_chirho[from_chirho][to_chirho] |= reachable_chirho[from_chirho]
                        [via_chirho]
                        && reachable_chirho[via_chirho][to_chirho];
                }
            }
        }
        let groups_chirho = dependency_groups_chirho(&edges_chirho);
        let mut membership_chirho = [None; 3];
        for (group_id_chirho, members_chirho) in groups_chirho.iter().enumerate() {
            for &member_chirho in members_chirho {
                assert!(
                    membership_chirho[member_chirho]
                        .replace(group_id_chirho)
                        .is_none()
                );
            }
        }
        assert!(membership_chirho.iter().all(Option::is_some));
        for from_chirho in 0..3 {
            for to_chirho in 0..3 {
                assert_eq!(
                    membership_chirho[from_chirho] == membership_chirho[to_chirho],
                    reachable_chirho[from_chirho][to_chirho]
                        && reachable_chirho[to_chirho][from_chirho],
                );
                if reachable_chirho[from_chirho][to_chirho] {
                    assert!(membership_chirho[to_chirho] <= membership_chirho[from_chirho]);
                }
            }
        }
        assert_eq!(groups_chirho, dependency_groups_chirho(&edges_chirho));
    }
}

#[test]
fn deep_dependency_chains_and_cycles_do_not_use_the_call_stack_chirho() {
    let count_chirho = 100_000;
    let mut edges_chirho: Vec<Vec<usize>> = (0..count_chirho)
        .map(|index_chirho| {
            if index_chirho + 1 == count_chirho {
                Vec::new()
            } else {
                vec![index_chirho + 1]
            }
        })
        .collect();
    let groups_chirho = dependency_groups_chirho(&edges_chirho);
    assert_eq!(groups_chirho.len(), count_chirho);
    assert_eq!(groups_chirho.first(), Some(&vec![count_chirho - 1]));
    assert_eq!(groups_chirho.last(), Some(&vec![0]));
    edges_chirho[count_chirho - 1].push(0);
    let groups_chirho = dependency_groups_chirho(&edges_chirho);
    assert_eq!(groups_chirho.len(), 1);
    assert_eq!(groups_chirho[0].len(), count_chirho);
}
