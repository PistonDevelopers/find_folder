
extern crate find_folder;

use find_folder::Search;

fn main() {
    println!("{:?}", Search::Parents(3).for_folder("src"));
    println!("{:?}", Search::Kids(3).for_folder("examples"));
    println!("{:?}", Search::Both(3, 3).for_folder("target"));
    println!("{:?}", Search::ParentsThenKids(3, 3).for_folder("src"));
    println!("{:?}", Search::KidsThenParents(3, 3).for_folder("examples"));
}


