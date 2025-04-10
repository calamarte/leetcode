use std::collections::HashSet;

#[allow(unused_variables, clippy::useless_vec)]
fn main() {
    let rooms = vec![vec![1], vec![2], vec![3], vec![]];
    let rooms = vec![vec![1, 3], vec![3, 0, 1], vec![2], vec![0]];

    println!("Result -> {}", can_visit_all_rooms(rooms));
}

/// - Patterns: DFS
/// - Time complexity: $$O(n + k)$$
/// - Space complexity: $$O(n + k)$$
fn can_visit_all_rooms(rooms: Vec<Vec<i32>>) -> bool {
    fn explore_room(room_idx: usize, rooms: &[Vec<i32>], visited: &mut HashSet<usize>) {
        if !visited.insert(room_idx) {
            return;
        }

        for &i in &rooms[room_idx] {
            explore_room(i as usize, rooms, visited);
        }
    }

    let mut visited = HashSet::new();
    explore_room(0, &rooms, &mut visited);

    rooms.len() == visited.len()
}
