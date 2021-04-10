use index_list::IndexList;

fn main() {
    let mut hello = vec!["Hello,", "world!"];
    let name = "IndexList";
    let mut list = IndexList::from(&mut hello);
    let index = list.insert_last(name);
    list.insert_before(index, "I");
    list.insert_before(index, "am");
    let parts: Vec<&str> = list.iter().map(|e| e.as_ref()).collect();
    println!("{}.", parts.join(" "));
    println!("The {} -- {} {}!",
             list.get(list.move_index(index, -3)).unwrap_or(&"?"),
             list.get_first().unwrap_or(&"?"),
             list.get_last().unwrap_or(&"?"));
}
