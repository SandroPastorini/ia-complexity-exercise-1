use rand::Rng;
use time::Instant;

fn main() {

    let mut num_list: Vec<i32> = Vec::new();

    for _ in 0..131072 {
        num_list.push(rand::thread_rng().gen());
    }

    let num_list: Vec<i32> = num_list.to_owned();

    let  (list_1024, _) = num_list.split_at(1024);
    let (list_4096, _) = num_list.split_at(4096);
    let (list_16384, _) = num_list.split_at(16384);
    let (list_131072, _) = num_list.split_at(131072);

    println!("Running for 1023 elements");
    bubble_sort(1023, list_1024.to_vec().as_mut());
    println!("Running for 4096 elements");
    bubble_sort(4096, list_4096.to_vec().as_mut());
    println!("Running for 16384 elements");
    bubble_sort(16384, list_16384.to_vec().as_mut());
    println!("Running for 131072 elements");
    bubble_sort(131072, list_131072.to_vec().as_mut());
}

fn bubble_sort(list_size: usize, num_list: &mut Vec<i32>) {
    for k in 0..10 {
        let start_time = time::Instant::now();
    
        for i in 0..list_size {
            for j in 0..list_size {
                if num_list.get(i) > num_list.get(j) {
                    num_list.swap(i, j);
                }
            }
        }

        let end_time: Instant = time::Instant::now();

        let delta = end_time - start_time;

        println!("Run {}: {}", k, delta.as_seconds_f64());
    }
}
