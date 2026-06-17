use std::collections::{BTreeMap, BTreeSet};

pub(crate) trait MergePartsOfLevel {
    fn merge_parts(mut self, other: Self) -> Self
    where
        Self: Sized,
    {
        self.merge_parts_ref(other);
        self
    }

    fn merge_parts_ref(&mut self, other: Self) -> &mut Self;
}

macro_rules! impl_merge_parts {
    ($($index: tt),+) => {
        pastey::paste!{
            impl<$(  [<_ $index >] ),+> MergePartsOfLevel for ($([<_ $index>]),+)

                where $([<_ $index>]: MergePartsOfLevel),+
            {
                fn merge_parts_ref(&mut self, other: Self) -> &mut Self {
                    $(
                        self.$index.merge_parts_ref(other.$index);
                    )+
                    self
                }
            }
        }
    };
}

impl_merge_parts!(0, 1);
impl_merge_parts!(0, 1, 2);
impl_merge_parts!(0, 1, 2, 3);
impl_merge_parts!(0, 1, 2, 3, 4);
impl_merge_parts!(0, 1, 2, 3, 4, 5);
impl_merge_parts!(0, 1, 2, 3, 4, 5, 6);
impl_merge_parts!(0, 1, 2, 3, 4, 5, 6, 7);
impl_merge_parts!(0, 1, 2, 3, 4, 5, 6, 7, 8);
impl_merge_parts!(0, 1, 2, 3, 4, 5, 6, 7, 8, 9);
impl_merge_parts!(0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10);
impl_merge_parts!(0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11);
impl_merge_parts!(0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12);

impl<T: Ord> MergePartsOfLevel for BTreeSet<T> {
    fn merge_parts_ref(&mut self, mut other: Self) -> &mut Self {
        self.append(&mut other);
        self
    }
}

impl<T: Ord, V> MergePartsOfLevel for BTreeMap<T, V> {
    fn merge_parts_ref(&mut self, mut other: Self) -> &mut Self {
        self.append(&mut other);
        self
    }
}
