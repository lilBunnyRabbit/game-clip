pub fn _lz77(buffer: Vec<u8>) {
  let sb_size = 6;
  let mut sb: (usize, usize) = (0, 0); // Search buffer = (start index, end index)
  let mut triplets: Vec<(usize, usize, u8)> = Vec::new();
  let buffer_len = buffer.len();
  let mut i: usize = 0;
  while i < buffer_len {
    match _find_in_search_buffer(&buffer, sb, i) {
      Some(idx) => {
        let distance = idx.1 - idx.0;
        let new_i = i + distance;
        triplets.push((i - idx.0, distance, buffer[new_i]));
        i = new_i;
        sb.1 = i - 1;
        sb.0 = sb.1 - sb_size;
        continue;
      },
      None => {
        triplets.push((0, 0, buffer[i]));
        sb.1 = i;
        if sb.1 - sb.0 > sb_size {
          sb.0 = sb.1 - sb_size;
        }
        i += 1;
        continue;
      }
    }
  }

  println!("Triplets: {:?}", triplets);
}

fn _find_in_search_buffer(buffer: &Vec<u8>, sb: (usize, usize), index: usize) -> Option<(usize, usize)> {
  if index == 0 {
    return None;
  }

  let mut idx: (usize, usize) = (0, 0);

  for i in sb.1..sb.0 {
    if buffer[i] == buffer[index] {
      let mut add: usize = 1;
      while buffer[i + add] == buffer[index + add] {
        add += 1;
      };

      if add - i > idx.1 - idx.0 {
        idx = (i, i + add);
      }
    }
  }

  if idx.1 - idx.0 != 0 {
    return Some(idx);
  }

  return None;
}