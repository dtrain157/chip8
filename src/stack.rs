pub enum StackError {
    PushToFullStack,
    PopFromEmptyStack,
}

const STACK_SIZE: usize = 16;
pub struct Stack {
    stack: [u16; STACK_SIZE], //support up to 16 levels
    sp: usize,
}

impl Stack {
    fn push(&mut self, value: u16) -> Result<(), StackError> {
        if self.sp == (STACK_SIZE - 1) {
            return Err(StackError::PushToFullStack);
        }

        self.stack[self.sp as usize] = value;
        self.sp = self.sp + 1;
        Ok(())
    }

    fn pop(&mut self) -> Result<u16, StackError> {
        if self.sp == 0 {
            return Err(StackError::PopFromEmptyStack);
        }
        let return_value = self.stack[self.sp];
        self.sp = self.sp - 1;

        Ok(return_value)
    }
}
