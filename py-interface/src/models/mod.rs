use std::{cmp::Eq, collections::HashMap, hash::Hash};

pub mod annotation;
pub mod classes;
pub mod dataset;
pub mod entries;
pub mod licence;
pub mod metadata;

pub trait DsmToPy<P> {
    fn to_py(&self) -> P;
}

impl<K, P, D> DsmToPy<HashMap<K, P>> for HashMap<K, D>
where 
    K: Eq + Hash + Clone,
    D: DsmToPy<P>
{
    fn to_py(&self) -> HashMap<K, P> {
        let mut result = HashMap::new();
        for (k, v) in self {
            result.insert(k.clone(), v.to_py());
        }
        result
    }
}

impl<K, P, D> DsmToPy<HashMap<K, P>> for &HashMap<K, D>
where 
    K: Eq + Hash + Clone,
    D: DsmToPy<P>
{
    fn to_py(&self) -> HashMap<K, P> {
        let mut result = HashMap::new();
        for (k, v) in self.iter() {
            result.insert(k.clone(), v.to_py());
        }
        result
    }
}

impl<P, D> DsmToPy<Vec<P>> for Vec<D>
where 
    D: DsmToPy<P>
{
    fn to_py(&self) -> Vec<P> {
        let mut result = Vec::new();
        for v in self {
            result.push(v.to_py());
        }
        result
    }
}