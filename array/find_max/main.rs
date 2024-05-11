
fn find_max(arr: &mut Vec<i32>) -> i32{
 
    let mut  max: i32 = i32::min_value();

    
    for &num in arr.iter() {
        if num > max {
            max = num;
        }
    }

   return  max;

}
fn main() {
    let mut arr = vec![2,343,3,1,4,5,34,34,34,5,4,34,35,-4,54,5,34,3,4,34,5,45,45,3,4,3,5,45,45,4,53,45,34];
    println!("max element is:{}",find_max(&mut arr));
 }