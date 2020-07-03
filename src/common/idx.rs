use std::default::Default;
use std::fmt;
use std::hash::Hash;
use std::iter::FromIterator;
use std::marker::PhantomData;
use std::ops::{Index, IndexMut};

pub trait Idx: 'static + Copy + Eq + Hash + fmt::Debug {
    fn index(&self) -> usize;

    fn new(index: usize) -> Self;
}

#[macro_export]
macro_rules! idx_ty {
    ($v:vis struct $name:ident { .. }) => {
        #[derive(Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
        $v struct $name {
            private: u32,
        }

        impl $crate::common::Idx for $name {
            #[inline(always)]
            fn index(&self) -> usize {
                self.private as usize
            }

            #[inline(always)]
            fn new(index: usize) -> Self {
                Self {
                    private: index as u32,
                }
            }
        }
    };
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct IdxVec<I, T> {
    raw: Vec<T>,
    _phantom: PhantomData<fn(&I)>,
}

impl<I, T> IdxVec<I, T>
where
    I: Idx,
{
    pub fn new() -> IdxVec<I, T> {
        IdxVec {
            raw: vec![],
            _phantom: PhantomData::default(),
        }
    }

    pub fn with_capacity(capacity: usize) -> IdxVec<I, T> {
        IdxVec {
            raw: Vec::with_capacity(capacity),
            _phantom: PhantomData::default(),
        }
    }

    pub fn push(&mut self, elem: T) -> I {
        let index = self.raw.len();
        self.raw.push(elem);
        I::new(index)
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.raw.is_empty()
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.raw.len()
    }

    #[inline]
    pub fn contains_key(&self, index: I) -> bool {
        self.raw.len() < index.index()
    }

    #[inline]
    pub fn get(&self, index: I) -> Option<&T> {
        self.raw.get(index.index())
    }

    #[inline]
    pub fn get_mut(&mut self, index: I) -> Option<&mut T> {
        self.raw.get_mut(index.index())
    }

    #[inline]
    pub fn iter(&self) -> impl Iterator<Item = (I, &T)> {
        self.keys().zip(self.values())
    }

    #[inline]
    pub fn keys(&self) -> impl Iterator<Item = I> {
        (0..self.raw.len()).map(I::new)
    }

    #[inline]
    pub fn values(&self) -> impl Iterator<Item = &T> {
        self.raw.iter()
    }

    #[inline]
    pub fn values_mut(&mut self) -> impl Iterator<Item = &mut T> {
        self.raw.iter_mut()
    }

    #[inline]
    pub fn clear(&mut self) {
        self.raw.clear()
    }
}

impl<I, T> IdxVec<I, T>
where
    T: Eq,
{
    #[inline]
    pub fn contains(&self, t: &T) -> bool {
        self.raw.contains(t)
    }
}

impl<I, T> Default for IdxVec<I, T>
where
    I: Idx,
{
    #[inline]
    fn default() -> IdxVec<I, T> {
        IdxVec::new()
    }
}

impl<I, T> FromIterator<T> for IdxVec<I, T>
where
    I: Idx,
{
    fn from_iter<U: IntoIterator<Item = T>>(iter: U) -> Self {
        let mut idx_vec = IdxVec::new();
        for t in iter {
            idx_vec.push(t);
        }
        idx_vec
    }
}

impl<I, T> Index<I> for IdxVec<I, T>
where
    I: Idx,
{
    type Output = T;

    fn index(&self, index: I) -> &Self::Output {
        &self.raw[index.index()]
    }
}

impl<I, T> IndexMut<I> for IdxVec<I, T>
where
    I: Idx,
{
    fn index_mut(&mut self, index: I) -> &mut Self::Output {
        &mut self.raw[index.index()]
    }
}
