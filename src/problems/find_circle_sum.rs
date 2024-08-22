use std::collections::{HashSet, LinkedList};

pub fn solution(is_connected: Vec<Vec<i32>>) -> i32 {
    // how to traverse the 2d vector? -> how does it represent a graph?
    // is_connected[i][j] represents an edge, and is_connected[j][i] represents the same edge (non-directed graph)
    // i,j in is_connected[i][j] represent two vertices.
    // for any two vertices, we must find out if they form a connected component

    // to do this, we can traverse graph over its edges, but consider all vertices

    let mut provinces = 0;
    let mut province: LinkedList<usize> = LinkedList::new();
    let mut visited: HashSet<usize> = HashSet::new();
    for potentially_unvisited_city in 0..is_connected.len() {
        if !visited.contains(&potentially_unvisited_city) {
            // now we know the vertex has not yet been visited, so from here, we will perform a graph traversal
            province.push_back(potentially_unvisited_city); // this city is in its own province, at the very least

            while !province.is_empty() {
                let city = province.pop_front().unwrap(); // start processing whether this city contains connected cities in its province
                if !visited.contains(&city) {
                    visited.insert(city); // mark this city as visited for the sake of processing whether it contains connected cities in its province
                    for (neighboring_city, is_connected) in is_connected[city].iter().enumerate() {
                        if *is_connected == 1 && !visited.contains(&neighboring_city) {
                            province.push_back(neighboring_city); // add all city's connected cities to the province
                        }
                    }
                }
            }
            provinces += 1;
        }
    }

    provinces
}

#[cfg(test)]
mod tests {
    use crate::utils::helper_macros::macros::vec_2d;

    use super::*;

    #[test]
    fn basic() {
        println!("assert_eq!(solution(vec_2d![[1]]), 1);"); // my simple test case to start
        assert_eq!(solution(vec_2d![[1]]), 1);
        println!("assert_eq!(solution(vec_2d![[1,1,0],[1,1,0],[0,0,1]]), 2);");
        assert_eq!(solution(vec_2d![[1, 1, 0], [1, 1, 0], [0, 0, 1]]), 2);
        println!("assert_eq!(solution(vec_2d![[1,0,0],[0,1,0],[0,0,1]]), 3);");
        assert_eq!(solution(vec_2d![[1, 0, 0], [0, 1, 0], [0, 0, 1]]), 3);
        println!("assert_eq!(solution(vec_2d![[1,1,1],[1,1,1],[1,1,1]]), 1);");
        assert_eq!(solution(vec_2d![[1, 1, 1], [1, 1, 1], [1, 1, 1]]), 1);
    }
}
