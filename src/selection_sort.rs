use rand::Rng;

fn main() {
    let mut num_list: Vec<i32> = Vec::new();

    for _ in 0..131072 {
        num_list.push(rand::thread_rng().gen());
    }

    println!("Generated unordered list");

    let num_list: Vec<i32> = num_list.to_owned();

    let  (list_1024, _) = num_list.split_at(1024);
    let (list_4096, _) = num_list.split_at(4096);
    let (list_16384, _) = num_list.split_at(16384);
    let (list_131072, _) = num_list.split_at(131072);

    println!("Running selection_sort for 1023 elements");
    selection_sort_profiling(1023, list_1024.to_vec().as_mut());
    println!("Running selection_sort for 4096 elements");
    selection_sort_profiling(4096, list_4096.to_vec().as_mut());
    println!("Running selection_sort for 16384 elements");
    selection_sort_profiling(16384, list_16384.to_vec().as_mut());
    println!("Running selection_sort for 131072 elements");
    selection_sort_profiling(131072, list_131072.to_vec().as_mut());

}

fn selection_sort_profiling(list_size: usize, num_list: &mut Vec<i32>) {
    for k in 0..9 {
        let start_time = time::Instant::now();
        selection_sort(list_size, num_list);
        let end_time = time::Instant::now();
        let delta = end_time - start_time;

        println!("Run {}: {}", k, delta.as_seconds_f64());
    }
}

fn selection_sort(list_size: usize, num_list: &mut Vec<i32>) {
    for i in 0..list_size {
        let mut min = i;
        
        for j in i..list_size {
            if num_list.get(min) > num_list.get(j) {
                min = j;
            }
        }

        num_list.swap(i, min);
    }
}