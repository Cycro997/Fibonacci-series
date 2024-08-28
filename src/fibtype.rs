use super::calc;

#[derive(Debug)]
pub struct Fibonacci{
    index: usize,
    value: usize,
}
impl Fibonacci{
    pub fn new(index: usize) -> Self{
        let value: usize = calc::calculate(index);

        return Self{index: index, value: value};
    }
    pub fn get_value(&self) -> usize{
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