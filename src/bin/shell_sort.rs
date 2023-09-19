use rand::Rng;

fn main() {
    let mut num_list: Vec<i32> = Vec::new();

    // for _ in 0..131072 {
    //     num_list.push(rand::thread_rng().gen());
    // }

    // println!("Generated unordered list");

    // let num_list: Vec<i32> = num_list.to_owned();

    // let (list_1024, _) = num_list.split_at(1024);
    // let (list_4096, _) = num_list.split_at(4096);
    // let (list_16384, _) = num_list.split_at(16384);
    // let (list_131072, _) = num_list.split_at(131072);

    // shell_sort(list_1024.to_vec().as_mut());
    num_list.push(32);
    num_list.push(16);
    num_list.push(22);
    num_list.push(1);
    num_list.push(9);
    shell_sort(&mut num_list)
}

fn shell_sort(num_list: &mut Vec<i32>) {
    let mut dist = num_list.len();
    let mut cuotient = 1;

    // Iterate using Shell's sequence (N/2, N/4, ..., 1)
    while dist >= cuotient {
        cuotient *= 2;
        dist /= cuotient;
        println!("{}", dist);
        
    }
}