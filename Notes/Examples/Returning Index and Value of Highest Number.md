### Done
This is a short program showing how you would return the `index` and `value` of the highest number in something like an array using `iter().enumerate()`.

<br><br>
# Program
```Rust
// main.rs

fn main(){
	let mut max: usize = 0;
	let array: [i32; 5] = [6, 1, 82, 65, 24];

	for (index, value) in array.iter().enumerate(){
		if value > &array[max] {
			max = index;
		}
    }

	println!("Array is : {:?}", array);
	println!("The highest number in the list is: {} at index: {}", array[max], max);
}
```

### Output
```
Array is : [6, 1, 82, 65, 24]
The highest number in the list is: 82 at index: 2
```

<br><br>
# Breakdown
For this program, our objective is to return the `index and value of the largest number in an array`. To begin, we start with our `max` variable.

#### let mut max: usize: 0
```Rust
let mut max: usize = 0;
```

We name it `max`, because it represents the index of the number that the max is located. We want its type to be `usize`, because `usize` is how we index arrays. We set it to `mut` because we want to be able to change its value, and we set it to `0` to initialize it.

From there we initialize our array

#### let array: [i32; 5] = [6, 1, 82, 65, 24];
```Rust
let array: [i32; 5] = [6, 1, 82, 65, 24];
```

We declare the array as `[i32; 5]`, saying that this array will be a collection of `5` values whos types will be `i32`. Following the `=` sign, we populate the array.

From there we go to our `for loop`.

#### for loop
```Rust
for (index, value) in array.iter().enumerate(){
	if index > max {
		max = index;
	}
}
```

When we loop through our array, we want to track the `highest value` in the array, but we also want to track `what the index of that value is`. For this reason, we can use the `enumerate()` method attached to `iter()`.

#### (index, value)
```Rust
(index, value)
```
We declare a pair `(index, value)` that our relative index and value of each element will be stored in when we iterate

#### array.iter().enumerate()
```Rust
array.iter().enumerate()
```
Then we say we're iterating on `array` with `iter().enumerate()`. `iter()`is a method attached to any `list` type value, that allows you to iterate over each element contained within it. `enumerate()` is a method attached to `iter()` that represents our iterated value as an `index` *and* that index's `relative value` stored in the array.

So effectively, `array.iter().enumerate()` iterates through our array, and stores each relative `index` and `value` pair in our `(index, value)` pair that we created.

#### if value > &array[max]
```Rust
if value > &array[max]
```
With this, we can move into our `if` statement. We only want to change the value of `max` if we find `the index of a number that is bigger than it` in our array, so we set our condition to `value > &array[max]`. This says that if we find a `value` that is bigger than the `value of the number stored in our array at index: max`, then we will make the `index` of this new value the `index of our 'max' value`.

#### max = index
```Rust
max = index;
```
When this condition is found, we just say `max = index` to set the index of the new highest number.

#### println!()
```Rust
println!("Array is : {:?}", array);
println!("The highest number in the list is: {} at index: {}", max, array[max]);
```

From there, we can simply just print our array with `{:?}, array` and print our values, and the program is finished.