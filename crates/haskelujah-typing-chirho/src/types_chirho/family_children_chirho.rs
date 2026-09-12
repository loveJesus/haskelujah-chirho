// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! A constant-work borrowed view: inspecting a tuple's fanout must not copy it
//! before the equation-work budget can decline that traversal.

pub(crate) struct FamilyChildrenChirho<'term_chirho, TermChirho> {
    first_chirho: Option<&'term_chirho TermChirho>,
    second_chirho: Option<&'term_chirho TermChirho>,
    tail_chirho: &'term_chirho [TermChirho],
}

impl<'term_chirho, TermChirho> FamilyChildrenChirho<'term_chirho, TermChirho> {
    pub(crate) fn single_chirho(first_chirho: &'term_chirho TermChirho) -> Self {
        Self {
            first_chirho: Some(first_chirho),
            second_chirho: None,
            tail_chirho: &[],
        }
    }

    pub(crate) fn pair_chirho(
        first_chirho: &'term_chirho TermChirho,
        second_chirho: &'term_chirho TermChirho,
    ) -> Self {
        Self {
            first_chirho: Some(first_chirho),
            second_chirho: Some(second_chirho),
            tail_chirho: &[],
        }
    }

    pub(crate) fn slice_chirho(tail_chirho: &'term_chirho [TermChirho]) -> Self {
        Self {
            first_chirho: None,
            second_chirho: None,
            tail_chirho,
        }
    }

    pub(crate) fn len_chirho(&self) -> usize {
        usize::from(self.first_chirho.is_some())
            + usize::from(self.second_chirho.is_some())
            + self.tail_chirho.len()
    }
}

impl<'term_chirho, TermChirho> IntoIterator for FamilyChildrenChirho<'term_chirho, TermChirho> {
    type Item = &'term_chirho TermChirho;
    type IntoIter = std::iter::Chain<
        std::iter::Chain<std::option::IntoIter<Self::Item>, std::option::IntoIter<Self::Item>>,
        std::slice::Iter<'term_chirho, TermChirho>,
    >;

    fn into_iter(self) -> Self::IntoIter {
        self.first_chirho
            .into_iter()
            .chain(self.second_chirho)
            .chain(self.tail_chirho.iter())
    }
}
