use std::error;
use std::fmt;

const STACK_SIZE: usize = 16; //support up to 16 levels

pub struct Stack {
    stack: [u16; STACK_SIZE],
    sp: usize,
}

impl Stack {
    pub fn new() -> Self {
        Stack {
            stack: [0; STACK_SIZE],
            sp: 0,
        }
    }

    pub fn push(&mut self, value: u16) -> Result<(), StackError> {
        if self.sp == (STACK_SIZE - 1) {
            return Err(StackError::PushToFullStack);
        }
        self.stack[self.sp] = value;
        self.sp = self.sp + 1;
        Ok(())
    }

    pub fn pop(&mut self) -> Result<u16, StackError> {
        if self.sp == 0 {
            return Err(StackError::PopFromEmptyStack);
        }
        self.sp = self.sp - 1;
        let return_value = self.stack[self.sp];
        Ok(return_value)
    }
}

#[derive(Debug)]
pub enum StackError {
    PushToFullStack,
    PopFromEmptyStack,
}

impl fmt::Display for StackError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            StackError::PushToFullStack => write!(f, "Tried to push to a full stack!"),
            StackError::PopFromEmptyStack => write!(f, "Tried to pop from an empty stack!"),
        }
    }
}

impl error::Error for StackError {}

#[cfg(test)]
mod stack_tests {
    use super::*;

    #[test]
    fn stack_push_pop() {
        let mut stack = Stack::new();
        stack.push(0x1111).unwrap();
        let popped_value = stack.pop().unwrap();
        assert_eq!(popped_value, 0x1111);
    }
}
