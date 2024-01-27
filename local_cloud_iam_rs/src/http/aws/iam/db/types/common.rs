pub(crate) trait Pageable {
    fn limit(&self) -> i32;
    fn skip(&self) -> i32;
}
