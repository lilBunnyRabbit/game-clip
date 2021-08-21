pub struct CircularBuffer<T> {
  capacity: usize,
  buffer: Vec<T>,
  position: usize,
  is_full: bool,
}

impl<T> CircularBuffer<T>
where
  T: std::fmt::Display + Clone,
{
  pub fn new(capacity: usize) -> Self {
    CircularBuffer {
      capacity,
      buffer: Vec::with_capacity(capacity),
      position: 0,
      is_full: false,
    }
  }

  pub fn add(&mut self, element: T) {
    if self.is_full {
      self.buffer[self.position] = element;
      self.position = if self.position == self.capacity - 1 {
        0
      } else {
        self.position + 1
      }
    } else {
      self.buffer.push(element);
      if self.buffer.len() == self.capacity {
        self.is_full = true;
      }
    }
  }

  pub fn clone_buffer(&self) -> Vec<T> {
    let buffer_len = self.buffer.len();
    let mut buffer = Vec::with_capacity(buffer_len);

    for i in self.position..buffer_len {
      buffer.push(self.buffer[i].clone());
    }

    for i in 0..self.position {
      buffer.push(self.buffer[i].clone());
    }

    return buffer;
  }

  pub fn _print(&self) {
    print!("CircularBuffer: [");
    let buffer_len = self.buffer.len();
    for i in 0..buffer_len {
      if i == self.position {
        print!("\x1b[0;32m{}\x1b[0m", self.buffer[i]);
      } else {
        print!("{}", self.buffer[i]);
      }

      if i < buffer_len - 1 {
        print!(", ");
      }
    }
    println!("]");
  }
}
