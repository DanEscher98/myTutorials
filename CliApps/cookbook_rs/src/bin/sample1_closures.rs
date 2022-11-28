use rand::Rng;

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn get_rand_tuple() -> (u32, u32) {
    let mut rng = rand::thread_rng();
    let x = rng.gen_range(0..99);
    let y = rng.gen_range(0..99);
    (x, y)
}

fn main() {
    let mut list = Vec::new();
    for _ in 0..10 {
        let (x, y) = get_rand_tuple();
        list.push(Rectangle {
            width: x,
            height: y,
        });
    }

    let mut num_sort_op = 0;
    list.sort_by_key(|r| {
        num_sort_op += 1;
        (r.width + r.height) / 2
    });
    println!("{:#?}, sorted in {num_sort_op} ops", list);
}
