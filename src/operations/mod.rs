pub mod addition;
pub mod division;
pub mod multiplication;
pub mod subtraction;



#[derive(Debug)]
pub enum Operation {
    Addition,
    division,
    multiplication,
    subtraction,
}
