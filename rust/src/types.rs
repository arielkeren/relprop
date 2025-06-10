pub type Relation = [u8; crate::constants::MAX_SET_SIZE];
pub type PropertyFn = fn(Relation, usize) -> bool;
pub type CountArray = [u64; crate::constants::NUMBER_OF_PROPERTIES];
pub type PropertyVec = Vec<usize>;
