pub fn single_number(nums: Vec<i32>) -> i32 {
       let mut zor = 0;

       for i in nums {
        zor = zor ^ i;

       }
       zor 
}


fn main() {

    let nums = [4,1,2,1,2];
    let res = single_number(nums.to_vec());
    println!("{:?}" , res);
    

}

