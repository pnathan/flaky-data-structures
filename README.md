flaky data structures
---
an exploration into data structures with Mozilla Rust

Strong elements of Okasaki's Purely Functional Data Structures are
be here.

currently: 

binary search trees

planned:

red black trees, probably the left-leaning variant ( http://www.mew.org/~kazu/proj/red-black-tree/ )

doubly-linked trees

hazy future:

unsafe circular buffer

vector-based priority queue / heap

better standard rope

cuckoo hash table: https://en.wikipedia.org/wiki/Cuckoo_hashing

todo:

turn it into a rust package

motivation:

Mozilla Rust is slightly unique and interesting in that it provides a
controllable GC as well as a controllable mutability model. it's optionally 
impure. Kind of cool. 

rust devs would like some refreshed data structures: 

http://irclog.gr/#browse/irc.mozilla.org/rust/291715


happy hacking.



License:

AGPL3 (http://www.gnu.org/licenses/agpl.txt)
