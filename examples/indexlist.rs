use index_list::{Index, IndexList};

fn main() {
    let name = "IndexList";
    let mut list = IndexList::<String>::from(
        &mut vec!["Hello,".into(), "world!".into()]);
    let index = list.insert_last(name.to_string());
    list.insert_before(index, String::from("I"));
    list.insert_before(index, String::from("am"));
    let parts: Vec<&str> = list.iter().map(|e| e.as_ref()).collect();
    println!("{}.", parts.join(" "));
    println!("The {} -- {} {}!",
             list.get(list.move_index(index, -3)).unwrap(),
             list.get_first().unwrap(),
             list.get_last().unwrap());
    let mut index = Index::new();
    while let index = list.next_index(index) {
        list.get_mut(index).map(|s| {
            *s = if s.chars().last().unwrap().is_ascii_punctuation() {
                s.get(0..s.len()-1).unwrap()
            } else {
                &s
            }.to_string().to_lowercase();
        });
    }
    println!("{}", list);
}
