#![no_std]

use core::slice::IterMut;

pub enum Input {}

pub enum OutputMsg {
    None,
    NotNone,
}

pub enum Output {}

pub struct Ymir {}

impl Ymir {
    pub fn new() -> Self {
        Self {}
    }

    pub fn tick<'a>(
        mut self,
        input: &[Input],
        outputs: IterMut<'a, Output>,
    ) -> (Self, IterMut<'a, Output>) {
        for output in &outputs {}
        (self, outputs)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
