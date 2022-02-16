use crate::source::Source;
use std::ops::Add;

pub struct PolySines<S1, S2>
where
    S1: Source,
    S1::Item: Add<S2::Item> + Add<S1::Item>,
    S2: Source,
    S2::Item: Add<S2::Item> + Add<S1::Item>,
{
    input1: S1,
    input2: S2,
}

impl<S1, S2> PolySines<S1, S2>
where
    S1: Source,
    S1::Item: Add<S2::Item> + Add<S1::Item>,
    S2: Source,
    S2::Item: Add<S2::Item> + Add<S1::Item>,
{
    pub fn new(input1: S1, input2: S2) -> Self {
        PolySines { input1, input2 }
    }
}

impl<S1, S2> Source for PolySines<S1, S2>
where
    S1: Source,
    S1::Item: Add<S2::Item> + Add<S1::Item>,
    S2: Source,
    S2::Item: Add<S2::Item> + Add<S1::Item>,
    // <S1::Item as Add<S2::Item>>::Output: Add,
    <PolySines<S1, S2> as Iterator>::Item: Add<<PolySines<S1, S2> as Iterator>::Item>,
{
    fn num_samples(&self) -> usize {
        0
    }
}

impl<S1, S2> Iterator for PolySines<S1, S2>
where
    S1: Source,
    S1::Item: Add<S2::Item> + Add<S1::Item>,
    S2: Source,
    S2::Item: Add<S2::Item> + Add<S1::Item>,
{
    type Item = <S1::Item as Add<S2::Item>>::Output;

    fn next(&mut self) -> Option<Self::Item> {
        match (self.input1.next(), self.input2.next()) {
            (Some(s1), Some(s2)) => Some(s1 + s2),
            _ => None,
        }
    }
}
