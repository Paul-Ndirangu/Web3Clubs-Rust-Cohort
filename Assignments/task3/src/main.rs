fn main() {
    let values = [1.2, 3.5, 8.9, 5.6];
    let output1 = array_input(&values);
    println!("{}", output1);
}

fn array_input(par1:&[f32]) -> f32{
    let mut index = 0;
    let length = par1.len();
    let mut totals = 0.0;
    while index < length{
        // index += 1
        index = index + 1;
        totals = totals + par1[index-1];
    
    }
    return totals
}