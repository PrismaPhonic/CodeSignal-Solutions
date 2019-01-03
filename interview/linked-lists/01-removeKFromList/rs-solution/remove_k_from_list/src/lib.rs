struct List<'a, T> {
    value: T,
    next: Option<Box<List<'a, T>>>,
}

impl<'a, T> List<'a, T> {
    fn new(value: T) -> Self {
        List { value, next: None }
    }
}

type ListNode<'a, T> = Option<Box<List<'a, T>>>;

impl<'a> Iterator for List<'a, i32> {
    type Item = &'a mut Box<List<'a, i32>>;
    
    fn next(&mut self) -> Option<Self::Item> {
        match &self.next {
            Some(node_list) => Some(node_list),
            None => None,
        }
    }
}


fn removeKFromList(mut l: ListNode<i32>, k: i32) -> ListNode<i32> {
    l.iter_mut().for_each(move |node| {
        if node.next.unwrap().value == k {
            node.next = node.next.unwrap().next;
        }
    });

    l
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
