fn main() {
    let corrects = [1,1,2,6,24,120,720,5040,40320,362880,3628800];
    let mut i = 0;
    for correct in corrects{
        println!("{}", test(factorial(i), correct));
        i += 1;
    }
}
fn factorial(num: u32) -> u32{
    if num == 0 || num == 1{
        return 1;
    }
    let mut output = 1u32;
    for n in 2..=num{
        output *= n;
    }
    return output;
}
fn test(answer: u32, correct:u32) -> bool{
    return answer == correct;
}