/*

pcwalton is looking for a doubly-linked tree

parent; first child; last child; next sibling; prev sibling

operations are: add as first child; add as last child; insert before some child; remove from tree
*/
struct Node<T> {
    data: T,
    parent: Option<@Node<T>>,
    first_child: Option<@Node<T>>,
    last_child: Option<@Node<T>>,
    next_sibling: Option<@Node<T>>,
    prev_sibling: Option<@Node<T>>
}

trait DoublyLinkedNode<T> {
    fn push_first_child<T>(@Node<T>) -> @Node<T>;
}
