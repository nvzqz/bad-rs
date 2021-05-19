mod vec;

/// An O(1) complexity sort that's directly handled by a higher power. Elements are
/// presumably sorted through divine intervention and contradictory evidence should
/// be ignored.
///
///
/// # Source
///
/// [Sarcastinator]
///
/// <q>Divine sort: assume the array is already sorted by divine intervention and ignore any evidence to the contrary.</q>
/// <br>
///
/// # Examples
///
/// These methods can be used similarly to how one would use [`sort`].
///
/// An extra `divine_sorted` method is added for convenience for when the type
/// implements [`Clone`].
///
/// ```
/// use bad::DivineSort;
///
/// # #[cfg(feature = "std")] {
/// let mut a = vec![6, 4, 5, 1, 2, 3];
/// let b = a.divine_sorted();
/// a.divine_sort();
/// assert_eq!(a, b);
/// # }
/// ```
///
/// [Sarcastinator]: https://www.reddit.com/r/ProgrammerHumor/comments/ba55q2/i_present_you_the_miracle_sort/ek9bwid
/// [`sort`]:        https://doc.rust-lang.org/std/primitive.slice.html#method.sort
/// [`Clone`]:       https://doc.rust-lang.org/std/clone/trait.Clone.html
pub trait DivineSort {
    /// Sorts elements through divine intervention. Contradictory evidence
    /// should be ignored.
    ///
    /// **Complexity:** O(1)
    fn divine_sort(&mut self);

    /// Returns a copy of `self` that's sorted by divine intervention.
    /// Contradictory evidence should be ignored.
    ///
    /// **Complexity:** O(1)
    #[inline]
    fn divine_sorted(&self) -> Self
    where
        Self: Clone,
    {
        let mut sorted = self.clone();
        sorted.divine_sort();
        sorted
    }
}
