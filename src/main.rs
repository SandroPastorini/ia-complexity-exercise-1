use rand::Rng;
use time::Instant;

fn main() {

    println!("Running for 1023 elements");
    bubble_sort(1023);
    println!("Running for 4096 elements");
    bubble_sort(4096);
    println!("Running for 16384 elements");
    bubble_sort(16384);
    println!("Running for 131072 elements");
    bubble_sort(131072);
}

// fn selection_sort() {

// }

fn bubble_sort(list_size: usize) {

    let mut num_list: Vec<i32> = Vec::new();

    for _ in 0..list_size {
        num_list.push(rand::thread_rng().gen());
    }

    for k in 0..10 {
        let start_time = time::Instant::now();
    
        for i in 0..list_size {
            for j in 0..list_size {
                if num_list.get(i) > num_list.get(j) {
                    // let aux = num_list.get(i);
                    num_list.swap(i, j);
                }
            }
        }

        let end_time: Instant = time::Instant::now();

        let delta = end_time - start_time;

        println!("Run {}: {}", k, delta.as_seconds_f64());
    }
}
