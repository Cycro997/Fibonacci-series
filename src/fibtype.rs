use super::calc;

#[derive(Debug)]
pub struct Fibonacci{
    index: usize,
    value: u128,
}
impl Fibonacci{
    pub fn new(index: usize) -> Self{
        let value: u128 = calc::calculate(index);

        return Self{index: index, value: value};
    }
    pub fn get_value(&self) -> u128{
        return self.value;
    }
    pub fn get_index(&self) -> usize{
        return self.index;
    }
    pub fn set_index(&mut self, index: usize){
        self.index = index;
        self.value = calc::calculate(index);
    }
}
impl std::fmt::Display for Fibonacci{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}