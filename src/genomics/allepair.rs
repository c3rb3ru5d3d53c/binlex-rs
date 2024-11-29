use crate::genomics::Gene;

#[allow(dead_code)]
#[derive(Debug, Clone, Copy)]
pub struct AllelePair {
    pub high: Gene,
    pub low: Gene,
}
