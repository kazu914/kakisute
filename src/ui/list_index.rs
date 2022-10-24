use anyhow::{anyhow, Ok, Result};
use std::usize;

pub struct ListIndex {
    index: Option<usize>,
    size: usize,
}

impl ListIndex {
    pub fn new(size: usize) -> Self {
        let index = if size == 0 { None } else { Some(0) };
        Self { index, size }
    }
    pub fn increment(&mut self) {
        if self.is_none() {
            return;
        }
        self.index = Some((self.index.unwrap() + 1) % self.size);
    }

    pub fn increment_n(&mut self, n: u16) {
        if self.is_none() {
            return;
        }
        if self.index.unwrap() + usize::from(n) > self.size - 1 {
            self.index = Some(self.size - 1)
        } else {
            self.index = Some(self.index.unwrap() + usize::from(n));
        }
    }

    pub fn decrement(&mut self) {
        if self.is_none() {
            return;
        }
        self.index = Some((self.index.unwrap() + self.size - 1) % self.size);
    }

    pub fn decrement_n(&mut self, n: u16) {
        if self.is_none() {
            return;
        }
        if self.index.unwrap() < usize::from(n) {
            self.index = Some(0)
        } else {
            self.index = Some(self.index.unwrap() - usize::from(n));
        }
    }

    pub fn is_some(&self) -> bool {
        self.index.is_some()
    }

    pub fn is_none(&self) -> bool {
        self.index.is_none()
    }

    pub fn get_index(&self) -> Result<usize> {
        if self.is_none() {
            return Err(anyhow!("Received get_index while ListIndex is None"));
        }
        Ok(self.index.unwrap())
    }
}

#[cfg(test)]
use speculate::speculate;
#[cfg(test)]
speculate! {
    describe "new" {
        it "with 0 returns index = None"{
            let list_index = ListIndex::new(0);
            assert_eq!(list_index.index, None)
        }

        it "with 3 returns index = Some(0)"{
            let list_index = ListIndex::new(3);
            assert_eq!(list_index.index, Some(0))
        }
    }


    describe "empty ListIndex" {
        it "get_index returns err "{
            let list_index = ListIndex::new(0);
            let res = list_index.get_index().is_err();
            assert!(res)
        }

        it "increment with nothing"{
            let mut list_index = ListIndex::new(0);
            list_index.increment();
            assert_eq!(list_index.index, None)
        }

        it "decrement with nothing"{
            let mut list_index = ListIndex::new(0);
            list_index.decrement();
            assert_eq!(list_index.index, None)
        }
    }

    describe "ListIndex with size = 3" {
        it "get_index returns Ok(n) "{
            let list_index = ListIndex::new(3);
            let res = list_index.get_index().unwrap();
            assert_eq!(res,0);
        }

        it "increment from 0 -> 1"{
            let  mut list_index = ListIndex::new(3);
            list_index.increment();
            let res = list_index.get_index().unwrap();
            assert_eq!(res, 1)
        }

        it "increment from 2 -> 3"{
            let  mut list_index = ListIndex::new(3);
            list_index.increment();
            list_index.increment();
            let res = list_index.get_index().unwrap();
            assert_eq!(res, 2);
            list_index.increment();
            let res = list_index.get_index().unwrap();
            assert_eq!(res, 0)
        }

        it "increment_n from 0 -> 2, given 2"{
            let  mut list_index = ListIndex::new(3);
            list_index.increment_n(2);
            let res = list_index.get_index().unwrap();
            assert_eq!(res, 2)
        }

        it "increment_n from 0 -> 2, given 3"{
            let  mut list_index = ListIndex::new(3);
            list_index.increment_n(3);
            let res = list_index.get_index().unwrap();
            assert_eq!(res, 2)
        }

        it "decrement from 0 -> 2"{
            let mut list_index = ListIndex::new(3);
            list_index.decrement();
            let res = list_index.get_index().unwrap();
            assert_eq!(res, 2);
        }

        it "decrement from 2 -> 1"{
            let mut list_index = ListIndex::new(3);
            list_index.decrement();
            list_index.decrement();
            let res = list_index.get_index().unwrap();
            assert_eq!(res, 1);
        }


        it "decrement_n from 2 -> 0, given 2"{
            let  mut list_index = ListIndex::new(3);
            list_index.increment_n(2);
            let res = list_index.get_index().unwrap();
            assert_eq!(res, 2);
            list_index.decrement_n(2);
            let res = list_index.get_index().unwrap();
            assert_eq!(res, 0)
        }

        it "decrement_n from 0 -> 2, given 3"{
            let  mut list_index = ListIndex::new(3);
            list_index.increment_n(2);
            let res = list_index.get_index().unwrap();
            assert_eq!(res, 2);
            list_index.decrement_n(3);
            let res = list_index.get_index().unwrap();
            assert_eq!(res, 0)
        }
    }

}
