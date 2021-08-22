pub struct FpsDisplay {
  data: f64,
  counter: usize,
  fps: usize,
}

impl FpsDisplay {
  pub fn new(fps: usize) -> Self {
    FpsDisplay {
      data: 0.0,
      fps,
      counter: 0
    }
  }

  pub fn add(&mut self, element: f64) {
    self.data += element;
    self.counter += 1;
    if self.counter == self.fps {
      println!("FPS: {}", (self.data / self.counter as f64).round());
      self.data = 0.0;
      self.counter = 0;
    }
  }
}
