use common::input_to_nums;

pub fn first(input: &[String]) -> i32 {
    let input: Vec<i32> =     input
        .iter()
        .map(|s| s.parse::<i32>().ok())
        .flatten()
        .collect();

    // vector holds list nodes; the nodes prev and next are indices in this list
    // (we swap around the indices, not the lists)
    let mut circular_list_nodes: Vec<ListNode> = Vec::with_capacity(input.len());
    for (i, item) in input.iter().enumerate() {
        let prev_i = if i == 0 { 
            input.len() - 1
        } else { 
            i - 1
        };
        circular_list_nodes.push(ListNode::new(input[i], i + 1, prev_i));
    }
    // make it circular
    circular_list_nodes[input.len() - 1].next = 0;

    // keep track of the 'first' item in the list (so we can fid the 1000th etc)
    let mut circular_list_head_i = 0usize;

    // mix all the entries
    for i in 0..input.len() {
        
        let current = circular_list_nodes[i];

        // // print
        // let mut current_i = circular_list_head_i;
        // for _ in 0..input.len() {
        //     print!("{}, ", circular_list_nodes[current_i].val);
        //     current_i = circular_list_nodes[current_i].next;
        // }
        // println!();
        // println!("{:?} {}", circular_list_nodes, circular_list_head_i);

        if current.val == 0 {
            continue;
        }

        let moving_right = current.val < 0;
        for _ in 0..(current.val.abs() as usize) { // move as many times as the value specified
            // moving left is the same as moving our previous right
            let current_i = if moving_right {
                circular_list_nodes[i].prev
            } else { 
                i
            };

            // move right
            let prev_i = circular_list_nodes[current_i].prev;
            
            let next_i = circular_list_nodes[current_i].next;

            let next_next_i = circular_list_nodes[next_i].next;

            circular_list_nodes[prev_i].next = next_i;

            circular_list_nodes[next_i].prev = prev_i;
            circular_list_nodes[next_i].next = current_i;

            circular_list_nodes[current_i].prev = next_i;
            circular_list_nodes[current_i].next = next_next_i;

            circular_list_nodes[next_next_i].prev = current_i;
  
        }
    }

            // // print
            // let mut current_i = circular_list_head_i;
            // for _ in 0..input.len() {
            //     print!("{}, ", circular_list_nodes[current_i].val);
            //     current_i = circular_list_nodes[current_i].next;
            // }
            // println!();

    // get the items in question
    // find index of 0
    // let mut i = 0;
    while circular_list_nodes[circular_list_head_i].val != 0 {
        circular_list_head_i = circular_list_nodes[circular_list_head_i].next;
    }
    // println!("location of 0: {}", i);

    let mut sum = 0;
    for i in 0..=3000 {
        
        let current_item = circular_list_nodes[circular_list_head_i];
        if i == 1000 || i == 2000 || i == 3000 {
            println!("nth item? {} {}", i , current_item.val);
            sum += current_item.val;
        }
        circular_list_head_i = current_item.next;
    }
    sum
}

pub fn second(input: &[String]) -> usize {
    0
}

#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
struct ListNode {
    val: i32,
    next: usize,
    prev: usize,
}

impl ListNode {
    fn new(val: i32, next: usize, prev: usize) -> ListNode { 
        ListNode{val, next, prev}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn example() -> Vec<String> {
        let input: Vec<&str> = vec![
            "1",
            "2",
            "-3",
            "3",
            "-2",
            "0",
            "4",
        ];
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
        assert_eq!(result, 0);
    }
}
