#[derive(Debug)]
pub struct Paginator<'a, T> {
    container: &'a [T],
    curr_page: usize,
    per_page: usize,
}

impl<'a, T> Paginator<'a, T> {
    pub fn new(container: &'a [T], per_page: usize) -> Self {
        Paginator {
            container: container,
            curr_page: 0,
            per_page: per_page,
        }
    }

    pub fn at(&self, page: usize) -> Option<&'a [T]> {
        if self.container.len() > self.per_page * page {
            let start = self.per_page * page;
            let end = if self.container.len() < start + self.per_page {
                self.container.len() - 1
            } else {
                self.per_page * (page + 1) - 1
            };
            Some(&self.container[start..=end])
        } else {
            None
        }
    }

    pub fn len(&self) -> usize {
        self.container.len() / self.per_page + 1
    }
}

impl<'a, T> Iterator for Paginator<'a, T> {
    type Item = &'a [T];

    fn next(&mut self) -> Option<&'a [T]> {
        let slice = self.at(self.curr_page);
        self.curr_page += 1;
        slice
    }
}

#[test]
fn test_paginator() {
    let vec: Vec<_> = (0..=28).collect();
    let paginator = Paginator::new(&vec, 10);

    assert_eq!(3, paginator.len());

    for (i, p) in paginator.enumerate() {
        match i {
            0 => {
                assert_eq!(10, p.len());
                assert_eq!((0..=9).collect::<Vec<_>>(), p);
            }
            1 => {
                assert_eq!(10, p.len());
                assert_eq!((10..=19).collect::<Vec<_>>(), p);
            }
            _ => {
                assert_eq!(9, p.len());
                assert_eq!((20..=28).collect::<Vec<_>>(), p);
            }
        }
    }
}
