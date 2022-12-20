pub fn first(input: &[String]) -> i64 {
    mix_it_up(&input, 1, 1)
}

pub fn second(input: &[String]) -> i64 {
    mix_it_up(&input, 811589153i64, 10)
}

fn mix_it_up(input: &[String], decryption_key: i64, max_mixes: usize) -> i64 {
    let input: Vec<i64> = input
        .iter()
        .map(|s| s.parse::<i64>().ok())
        .flatten()
        .map(|v| v * decryption_key)
        .collect();

    // vector holds list nodes; the nodes prev and next are indices in this list
    // (we swap around the indices, not the lists)
    let mut circular_list_nodes: Vec<ListNode> = Vec::with_capacity(input.len());
    for (i, item) in input.iter().enumerate() {
        let prev_i = if i == 0 { input.len() - 1 } else { i - 1 };
        circular_list_nodes.push(ListNode::new(*item, i + 1, prev_i));
    }
    // make it circular
    circular_list_nodes[input.len() - 1].next = 0;

    // keep track of the 'first' item in the list (so we can fid the 1000th etc)
    let mut circular_list_head_i = 0usize;

    for max_i in 0..max_mixes {
        // mix all the entries
        for i in 0..input.len() {
            let current = circular_list_nodes[i];
            let current_val = current.val;

            if current_val == 0 {
                continue;
            }

            let moves = if current_val < 0 {
                // moving left is the same as moving right an opposite amount
                circular_list_nodes.len()
                    - 1
                    - ((current.val.abs() as usize) % (circular_list_nodes.len() - 1))
            } else {
                (current_val as usize) % (circular_list_nodes.len() - 1)
            };

            // example: move 2 3 to the right
            // p c n   np nn
            // 1 2 3 4 5  6
            // p n   np c nn
            // 1 3 4 5  2 6

            let prev_i = circular_list_nodes[i].prev;
            let next_i = circular_list_nodes[i].next;

            let mut new_prev_i = i;
            for _ in 0..moves {
                new_prev_i = circular_list_nodes[new_prev_i].next;
            }

            let new_next_i = circular_list_nodes[new_prev_i].next;

            circular_list_nodes[prev_i].next = next_i;

            circular_list_nodes[next_i].prev = prev_i;

            circular_list_nodes[new_prev_i].next = i;

            circular_list_nodes[i].prev = new_prev_i;
            circular_list_nodes[i].next = new_next_i;

            circular_list_nodes[new_next_i].prev = i;
        }
    }

    // get the items in question
    // find index of 0
    while circular_list_nodes[circular_list_head_i].val != 0 {
        circular_list_head_i = circular_list_nodes[circular_list_head_i].next;
    }

    let mut sum = 0;
    for i in 0..=3000 {
        let current_item = circular_list_nodes[circular_list_head_i];
        if i == 1000 || i == 2000 || i == 3000 {
            println!("nth item? {} {}", i, current_item.val);
            sum += current_item.val;
        }
        circular_list_head_i = current_item.next;
    }
    sum
}
#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
struct ListNode {
    val: i64,
    next: usize,
    prev: usize,
}

impl ListNode {
    fn new(val: i64, next: usize, prev: usize) -> ListNode {
        ListNode { val, next, prev }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn example() -> Vec<String> {
        let input: Vec<&str> = vec!["1", "2", "-3", "3", "-2", "0", "4"];
        input.iter().map(|s: &&str| String::from(*s)).collect()
    }

    #[test]
    fn first_test() {
        let input = example();
        let result = first(&input);
        assert_eq!(result, 3);
    }

    #[test]
    fn second_test() {
        let input = example();
        let result = second(&input);
        assert_eq!(result, 1623178306);
    }
}
