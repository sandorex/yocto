//! Simple implementation of gap buffer structure

/// Simple gap buffer using strings
#[derive(PartialEq)]
pub struct GapBuffer {
    /// Starting piece of buffer, cursor is at the end of this buffer
    pub before: String,

    /// Rest of buffer
    pub after: String,
}

// add nice view of gap buffer
impl std::fmt::Debug for GapBuffer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        const TRUNCATE: usize = 32;

        // truncate long buffers so as not to spam stdout
        let gap_size = self.before.capacity() - self.before.len();
        let before = if self.before.len() > TRUNCATE { &self.before[self.before.len() - TRUNCATE..] } else { &self.before[..] };
        let after = if self.after.len() > TRUNCATE { &self.after[..TRUNCATE] } else { &self.after[..] };
        let data = format!("{}|{}|{}", before, "_".repeat(gap_size), after);
        let pos = self.position();
        let len = self.len();

        f.debug_struct("GapBuffer")
            .field("data", &data)
            .field("pos", &pos)
            .field("gap", &gap_size)
            .field("len", &len)
            .finish()
    }
}

#[allow(dead_code)]
impl GapBuffer {
    pub fn new(input: String) -> Self {
        Self {
            before: String::new(),
            after: input,
        }
    }

    pub fn position(&self) -> usize {
        self.before.len()
    }

    /// Length of the buffer (used part only) in bytes
    pub fn len(&self) -> usize {
        self.before.len() + self.after.len()
    }

    /// Moves the gap to desired position (expensive)
    ///
    /// TODO: cutting unicode in the middle may cause panic afterwards
    pub fn move_to(&mut self, pos: usize) {
        assert!(pos <= self.len());

        let cur_pos = self.position();
        if pos > cur_pos {
            // moving from after to before
            // ||Hello World!| -> |Hel|lo World!|
            let rel_pos = pos - cur_pos;

            self.before.push_str(&self.after[..rel_pos]);

            // delete the part that was copied
            self.after = self.after[rel_pos..].to_string();
        } else {
            // moving from before to after
            // |Hello| World!| -> ||Hello World!|
            self.after.insert_str(0, &self.before[pos..]);

            // delete the part that was copied
            self.before = self.before[..pos].to_string();
        }
    }

    /// Move the gap by desired amount
    pub fn move_rel(&mut self, amount: usize) {
        self.move_to(self.position() - amount);
    }

    pub fn push_char(&mut self, ch: char) {
        // let String deal with allocation
        self.before.push(ch);
    }

    pub fn push_str(&mut self, input: &str) {
        self.before.push_str(input);
    }

    pub fn remove_char(&mut self) -> bool {
        self.before.pop().is_some()
    }

    pub fn remove_many(&mut self, amount: usize) -> usize {
        for i in 0..amount {
            // if no more characters then return current count
            if self.before.pop().is_none() {
                return i;
            }
        }

        return amount;
    }
}

#[cfg(test)]
mod tests {
    use super::GapBuffer;

    #[test]
    fn test_new() {
        let buf = GapBuffer::new("Hello".to_string());
        assert_eq!(buf.before, "");
        assert_eq!(buf.position(), 0);
        assert_eq!(buf.len(), 5);
    }

    #[test]
    fn test_push() {
        let mut buf = GapBuffer::new("Hello".to_string());
        assert_eq!(buf.len(), 5);

        buf.push_char('?');

        assert_eq!(buf.before, "?");
        assert_eq!(buf.len(), 6);
    }

    #[test]
    fn test_move() {
        let mut buf = GapBuffer::new("Hello World!".to_string());

        buf.push_char('?');
        buf.move_to(6);

        assert_eq!(buf.position(), 6);
        assert_eq!(buf.before, "?Hello");
        assert_eq!(buf.after, " World!");

        buf.move_to(4);
        assert_eq!(buf.position(), 4);
        assert_eq!(buf.before, "?Hel");
        assert_eq!(buf.after, "lo World!");
    }

    #[test]
    fn test_remove() {
        let mut buf = GapBuffer::new("Hello World!".to_string());
        buf.move_to(6);
        buf.remove_char();

        assert_eq!(buf.before, "Hello");
        assert_eq!(buf.after, "World!");

        buf.remove_many(5);

        assert_eq!(buf.before, "");
        assert_eq!(buf.after, "World!");
    }

    #[test]
    fn test_unicode() {
        let mut buf = GapBuffer::new("ÃÃÃ".to_string());
        buf.move_to(2);
        buf.remove_char();

        assert_eq!(buf.before, "");
        assert_eq!(buf.after, "ÃÃ");
    }
}
