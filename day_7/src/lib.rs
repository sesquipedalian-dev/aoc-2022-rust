use std::collections::HashMap;

pub fn first(input: &[String]) -> usize {
    let dir_sizes = compute_dir_sizes(input);
    println!("dir sizes {:?}", dir_sizes);

    let mut sum_of_smaller_dirs: usize = 0;
    for (_, size) in dir_sizes.iter() {
        if *size <= 100_000 {
            sum_of_smaller_dirs += size;
        }
    }
    sum_of_smaller_dirs
}

pub fn second(input: &[String]) -> usize {
    let dir_sizes = compute_dir_sizes(input);
    println!("dir sizes {:?}", dir_sizes);

    let current_used_space = dir_sizes.get(&String::from("/")).unwrap();
    let current_unused_space = 70_000_000 - current_used_space;
    let needed_additional_space = 30_000_000 - current_unused_space;

    dir_sizes
        .iter()
        .fold(30_000_000, |min, (_, next_dir_size)| {
            if *next_dir_size > needed_additional_space && *next_dir_size < min {
                *next_dir_size
            } else {
                min
            }
        })
}

fn compute_dir_sizes(input: &[String]) -> HashMap<String, usize> {
    let mut dir_sizes = HashMap::new();
    let mut dir_names: Vec<&str> = vec![];
    for line in input {
        if line.starts_with("$") {
            match line.strip_prefix("$ cd ") {
                // move to parent dir
                Some("..") => {
                    dir_names.pop();
                }
                // move to dir within current dir
                Some(dir_label) => {
                    dir_names.push(dir_label);
                }
                // actually an ls command; no-op
                _ => (),
            }
        } else if !line.starts_with("dir") {
            // size of a file
            let file_size: usize = line.split_whitespace().next().unwrap().parse().unwrap();
            let mut dir_name_accum = String::from("");
            for dir_name in dir_names.iter() {
                dir_name_accum.push_str(dir_name); // make the hash key the path to here to avoid duplicate name problems
                let existing_size = dir_sizes
                    .get(&dir_name_accum)
                    .map(|r| *r)
                    .unwrap_or_default();
                dir_sizes.insert(dir_name_accum.clone(), existing_size + file_size);
            }
        }
    }

    dir_sizes
}

#[cfg(test)]
mod tests {
    use super::*;

    fn example() -> Vec<String> {
        let input: Vec<&str> = vec![
            "$ cd /",
            "$ ls",
            "dir a",
            "14848514 b.txt",
            "8504156 c.dat",
            "dir d",
            "$ cd a",
            "$ ls",
            "dir e",
            "29116 f",
            "2557 g",
            "62596 h.lst",
            "$ cd e",
            "$ ls",
            "584 i",
            "$ cd ..",
            "$ cd ..",
            "$ cd d",
            "$ ls",
            "4060174 j",
            "8033020 d.log",
            "5626152 d.ext",
            "7214296 k",
        ];
        input.iter().map(|s: &&str| String::from(*s)).collect()
    }

    fn example2() -> Vec<String> {
        let input: Vec<&str> = vec![
            "$ cd /", "$ ls", "dir a", "dir b", "$ cd a", "$ ls", "dir c", "$ cd c", "$ ls",
            "99999 d", // /, a, and c all get 99_999
            "$ cd ..", "$ cd ..", "$ cd b", "$ ls", "dir c", "$ cd c", "$ ls",
            "99999 d", // /, b, and c all get 99_999
        ];
        input.iter().map(|s: &&str| String::from(*s)).collect()
    }

    #[test]
    fn test_same_name_dirs() {
        let input = example2();
        let result = first(&input);
        assert_eq!(result, 99_999 * 4);
    }

    #[test]
    fn first_test() {
        let input = example();
        let result = first(&input);
        assert_eq!(result, 95437);
    }

    #[test]
    fn second_test() {
        let input = example();
        let result = second(&input);
        assert_eq!(result, 24933642);
    }
}
