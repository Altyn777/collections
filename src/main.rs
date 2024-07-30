use std::collections::HashMap;

struct ListResult {
    median: f32,
    mode: i8,
}

/*
 * Given a list of integers, use a vector and return the median (when sorted, the
 * value in the middle position) and mode (the value that occurs most often; a hash map
 * will be helpful here) of the list.
 */
fn get_median_and_mode(list: &mut Vec<i8>) -> ListResult {
    list.sort();
    let i = list.len() / 2;
    let median = if list.len() % 2 == 0 {
        (list[i - 1] as f32 + list[i] as f32) / 2.0
    } else {
        list[i] as f32
    };

    let mut map = HashMap::new();

    let mut mode = 0;
    let mut max = 0;

    for &integer in list.iter() {
        let count = map.entry(integer).or_insert(0);
        *count += 1;
        if max < *count {
            max = *count;
            mode = integer;
        }
    }

    ListResult { median, mode }
}

fn main() {
    let mut list = vec![3, 1, 5, 7, 2, 8, 4, 1, 0];
    let ListResult { median, mode } = get_median_and_mode(&mut list);
    println!("Median is {median}, Mode is {mode}");
}
