// In this what I am trying to do is create a stack, traverse the string and
// if there is any open bracket I am pushing to stack and else if any close bracket
// comes then matching it with open and both matches then simply it return empty.
// At the end just checking whether my stack is empty or not if yes then return true or else false.
// T.c = O(n) =  where n is size of string 
// S.c = 0(n) = because using extra space stack for pushing

pub fn is_valid(s: String) -> bool {
        let mut stack = vec![];
        for i in s.chars(){
            if i == '(' || i == '{' || i == '[' {
                stack.push(i);
            }else{
                match (stack.pop() , i) {
                    (Some('{'), '}') => {},
                    (Some('('), ')') => {},
                    (Some('['), ']') => {},
                     _ => return false,

                }
            }

        }
        stack.is_empty()
}


fn main(){
    let s = "(()){";
    let check = is_valid(s.to_owned());
    if check {
    println!("true");
   }else{
    println!("false");
    
   }

}