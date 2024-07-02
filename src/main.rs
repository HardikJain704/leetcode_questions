// In this what I am doing is setting a prefix to first string in vec and creating a copy of first string
// after which we can modify prefix without affecting original vec. AFter which inside the loop
// checking current string i starts with current prefix if not then remove the last character from prefix
// keep removing character from prefix unit i starts with prefix
// if prefix becomes empty it means there is nothing common. AFter which prefix will contain the longest common prefix
// This is not the most optimised solution. 
//T.C- 0(N*M) - n is no of string and m is length of shortest string in vec
// S.C- 0(M) - length of shortest string in vector 
 
pub fn longest_common_prefix(strs: Vec<String>) -> String {

       if strs.is_empty(){
        return "".to_string();

       }

       let mut prefix = strs[0].clone();
       for i in strs.iter() {
        while !i.starts_with(&prefix){
            prefix.pop();

            if prefix.is_empty() {
                return "".to_string();
            }
        }
       
       }
       prefix
}


fn main(){

    let mut strings = Vec::new();
    strings.push("club".to_string());
    strings.push("clap".to_string());
    strings.push("clove".to_string());

    let result = longest_common_prefix(strings).to_string();

    println!("{:?}", result);


}
