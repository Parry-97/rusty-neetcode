struct MinStack {
    mins: Vec<i32>,
    elems: Vec<i32>,
}

//NOTE: I was able to come up with the intuition of using the two stacks, but not to
//come up with the clever insertion. Basically we are keeping track of the min relative
//to every elem in the stack. Using this approach we can maintain the order of the
//elements in the stack in a typical FIFO strategy while also being able to quickly
//retrieve the min.
//We could extend this approach, as Neetcode did and use only a single stack
//structure.
//
impl MinStack {
    fn new() -> Self {
        MinStack {
            mins: vec![],
            elems: vec![],
        }
    }

    fn push(&mut self, val: i32) {
        self.elems.push(val);
        if let Some(last) = self.mins.last() {
            if val < *last {
                self.mins.push(val);
            } else {
                self.mins.push(*last);
            }
        } else {
            self.mins.push(val);
        }

        // A faster but maybe less clear way of writing the above insertion
        //
        // let val = val.min(*self.mins.last().unwrap_or(&val));
        // self.mins.push(val);
    }

    fn pop(&mut self) {
        //NOTE:  As mentioned in the pop documentation, this takes O(1) time complexity
        self.mins.pop();
        self.elems.pop();
    }

    fn top(&self) -> i32 {
        *self.elems.last().unwrap()
    }

    fn get_min(&self) -> i32 {
        *self.mins.last().unwrap()
    }
}
