// fn largest_jolt(s:&str)->u32{
//     let mut first = 0;
//     let mut second = 0;

//     for c in s.chars(){
//         let d = c.to_digit(10).unwrap();
//         if first<second{
//             first = second;
//             second = 0;
//         }
//         if d>second{
//             second=d;
//         }
//     }
//     first*10+second
// }

fn largest_jolt<const N:usize>(s:&str)->u64{
    let mut digits = [0;N];

    for c in s.chars(){
        let d = c.to_digit(10).unwrap();
        for i in (0..N-1).into_iter().rev(){
            if digits[i+1]<digits[i]{
                digits[i+1]=digits[i];
                digits[i]=0;
            }
        }
        if d>digits[0]{
            digits[0]=d;
        }
    }
    let mut tens = 1;
    let mut ans : u64 = 0;
    for d in digits.iter(){
        ans+=tens*(*d as u64);
        tens*=10;
    }

    ans
}

fn main() {
    let input = include_str!("input.txt");
    let mut total = 0;
    for l in input.lines(){
        total+=largest_jolt::<12>(l);
        // println!("largest {}",largest_jolt(l));
    }

    println!("got {total}");
}
