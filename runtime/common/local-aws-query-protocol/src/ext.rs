pub(crate) trait StringExt<T: AsRef<str>> {
    fn is_numeric(&self) -> bool;
}

impl<T: AsRef<str>> StringExt<T> for T
where
    String: PartialEq<T>,
{
    fn is_numeric(&self) -> bool {
        self.as_ref().chars().all(|ch| ch.is_numeric())
    }
}
