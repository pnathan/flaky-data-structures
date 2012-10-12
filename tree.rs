// tree.rs.
// Happy hacking.
use core::cmp::{Eq, Ord};
use core::rand::{random};
enum MaybeNode<T: Ord>
{
    Empty,
    Node(@T, @MaybeNode<T>,@MaybeNode<T>)
}


fn insert<T: Ord> (newdata: @T, root: @MaybeNode<T>) -> @MaybeNode<T>
{
    @match root {
      @Empty => {
        Node(newdata, @Empty, @Empty)
      }
      @Node(data, left, right) => {
        if ( data < newdata ) {
            Node(data, insert(newdata, left), right)
        }
        else {
            Node(data, left, insert(newdata, right))
        }
      }

    }
}


fn main()
{
    let mut root : @MaybeNode<uint> = @Empty;

    let mut counter = 3;
    while counter > 0 {

        let rint = random() % 20;
        root = insert(@rint, root);

        io::println(fmt!("Inserted %? ", rint));
        io::println(fmt!("%?", root));

        counter -= 1;
    }

}