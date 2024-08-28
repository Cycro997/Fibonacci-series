pub fn calculate(index: usize) -> usize{
    if index == 0{
        return 0;
    } else if index == 1{
        return 1;
    }
    return calculate(index-2) + calculate(index-1);
}