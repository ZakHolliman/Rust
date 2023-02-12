// Given an array of integers nums and an integer target, return indices of the 
// two numbers such that they add up to target.

fn main(){
    let test: Vec<i32> = vec![2, 5, 9, 14, 20];

    let tuple = two_sum(&test, 11);

    println!("Returned tuple is {} {}", tuple.0, tuple.1);
}

fn two_sum(nums: &Vec<i32>, target: i32) -> (i32, i32) {
    let size = nums.len();

    let mut tup: (i32, i32) = (-1, -1);

    'outer: for num in 0..size {
        for num2 in num..size {
            if nums[num] + nums[num2] == target {
                // println!("{} + {} = {}", nums[num], nums[num2], target);

                tup = (num as i32, num2 as i32);

                break 'outer;
            }
        }
    }

    tup
}