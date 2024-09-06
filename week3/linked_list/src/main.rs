use linked_list::LinkedList;
pub mod linked_list;

fn main() {
    let mut list: LinkedList<u32> = LinkedList::new();
    assert!(list.is_empty());
    assert_eq!(list.get_size(), 0);
    for i in 1..12 {
        list.push_front(i);
    }
    println!("{}", list);
    println!("list size: {}", list.get_size());
    println!("top element: {}", list.pop_front().unwrap());
    println!("{}", list);
    println!("size: {}", list.get_size());
    println!("{}", list.to_string()); // ToString impl for anything impl Display

    // If you implement iterator trait:
    for val in list.clone().into_iter() {
       println!("iter:{} {}", val,list.get_size());
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_linked_list() {
        let mut list: LinkedList<u32> = LinkedList::new();
        assert!(list.is_empty());
        assert_eq!(list.get_size(), 0);
        for i in 1..5 {
            list.push_front(i);
        }
        assert_eq!(list.get_size(), 5);
        assert!(!list.is_empty());
        assert_eq!(list.pop_front().unwrap(), 5);
        assert_eq!(list.get_size(), 4);
        assert_eq!(list.pop_front().unwrap(), 4);
        assert_eq!(list.get_size(), 3);
        assert_eq!(list.pop_front().unwrap(), 3);
        assert_eq!(list.get_size(), 2);
        assert_eq!(list.pop_front().unwrap(), 2);
        assert_eq!(list.get_size(), 1);
        assert_eq!(list.pop_front().unwrap(), 1);
        assert_eq!(list.get_size(), 0);
        assert!(list.is_empty());
    }
    #[test]
    fn test_linked_list_generics() {
        let mut list: LinkedList<char> = LinkedList::new();
        list.push_front('a');
        list.push_front('b');
        list.push_front('c');
        assert!(list.pop_front().unwrap() == 'c');


    }

    #[test]
    fn test_linked_list_clone(){
        let mut list: LinkedList<u32> = LinkedList::new();
        let mut copy_list = list.clone();
        assert!(list==copy_list);
        copy_list.push_front(1);
        assert!(list!=copy_list);
        list.push_front(1);
        assert!(list==copy_list);
    }
    #[test]
    fn test_linked_list_iteration() {
        let mut list: LinkedList<char> = LinkedList::new();
        list.push_front('a');
        list.push_front('b');
        list.push_front('c');
        for val in &list{
            println!("{}",val);
        }
        assert!(list.get_size()==3);

    }

}