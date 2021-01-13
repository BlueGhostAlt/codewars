#[derive(Debug, PartialEq, Eq)]
pub enum Cons<T: Clone> {
    Cons(T, Box<Cons<T>>),
    Null,
}

impl<T: Clone> Cons<T> {
    pub fn new(head: T, tail: Self) -> Self {
        Cons::Cons(head, box tail)
    }

    pub fn to_vec(&self) -> Vec<T> {
        match &self {
            Cons::Null => Vec::new(),
            Cons::Cons(ref head, ref tail) => {
                let mut head = vec![head.clone()];
                head.extend(tail.to_vec());

                head
            }
        }
    }
}

impl<T: Clone> Cons<T> {
    pub fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = T>,
        I::IntoIter: Iterator + DoubleEndedIterator,
    {
        iter.into_iter()
            .rev()
            .fold(Cons::Null, |list, elem| Cons::new(elem, list))
    }

    pub fn filter<P>(&self, predicate: P) -> Self
    where
        P: Fn(&T) -> bool,
    {
        let mut new_elements = Vec::new();
        for element in self.to_vec().iter() {
            if predicate(element) {
                new_elements.push(element.clone());
            }
        }

        Cons::from_iter(new_elements)
    }

    pub fn map<F, B>(&self, f: F) -> Cons<B>
    where
        F: Fn(T) -> B,
        B: Clone,
    {
        let mut new_elements = Vec::new();
        for element in self.to_vec().into_iter() {
            new_elements.push(f(element));
        }

        Cons::from_iter(new_elements)
    }
}
