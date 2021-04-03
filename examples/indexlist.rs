use indexlist::IndexList;

fn main() {
    let mut hello = vec!["Hello,", "world!"];
    let name = "IndexList";
    let mut list = IndexList::from(&mut hello);
    let ndx = list.insert_last(name);
    list.insert_before(ndx, "I");
    list.insert_before(ndx, "am");
    let parts: Vec<&str> = list.iter().map(|e| e.as_ref()).collect();
    println!("{}", parts.join(" "));
}
