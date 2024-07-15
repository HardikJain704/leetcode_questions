use std::cmp::max;

pub fn largest_rectangle_area(heights: Vec<i32>) -> i32 {
        let (mut stack , mut max_area ) = (Vec::<(usize , i32)> :: new() , 0);
        for (i , &j) in heights.iter().enumerate() {
            let mut start = i;
            while !stack.is_empty() && stack.last().unwrap().1 > j {
                let (index , height) = stack.pop().unwrap();
                max_area = max(max_area , height * (i.checked_sub(index).unwrap()) as i32);
                start = index;
            }
            stack.push((start , j));




        }


        for (index, height ) in stack.into_iter() {
            max_area = max(max_area , height * (heights.len().checked_sub(index).unwrap()) as i32);
        }
        max_area
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_largest_rectangle_area() {
        let heights1 = vec![2, 1, 5, 6, 2, 3];
        assert_eq!(largest_rectangle_area(heights1), 10);

        let heights2 = vec![2, 4];
        assert_eq!(largest_rectangle_area(heights2), 4);

        let heights3 = vec![0];
        assert_eq!(largest_rectangle_area(heights3), 0);

        let heights4 = vec![1];
        assert_eq!(largest_rectangle_area(heights4), 1);

        let heights5 = vec![1, 2, 3, 4, 5];
        assert_eq!(largest_rectangle_area(heights5), 9);

        let heights6 = vec![5, 4, 3, 2, 1];
        assert_eq!(largest_rectangle_area(heights6), 9);

        let heights7 = vec![2, 1, 2];
        assert_eq!(largest_rectangle_area(heights7), 3);

        let heights8 = vec![6, 2, 5, 4, 5, 1, 6];
        assert_eq!(largest_rectangle_area(heights8), 12);
    }
}


fn main() {

    let  heights = vec![2,1,5,6,2,3];
    let res =largest_rectangle_area(heights);

    println!("{:?}" , res);
     

}
