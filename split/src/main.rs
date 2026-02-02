use split::split_at_mut;

fn main() {
    let mut v = vec![1, 2, 3, 4, 5, 6];

    {
        let r = &mut v[..];
        let (a, b) = split_at_mut(r, 3);

        a[0] = 10;
        b[0] = 40;

        println!("a = {:?}, b = {:?}", a, b);
    } 

    println!("v = {:?}", v);
}


