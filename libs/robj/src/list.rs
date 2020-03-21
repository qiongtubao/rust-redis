struct ListNode<T> {
    prev: ListNode<T>,
    next: ListNode<T>,
    value: T
}
impl ListNode<T> {

}

struct List<T> {
    head: ListNode<T>,
    tail: ListNode<T>,
    len: u32,
}
impl List<T> {

}