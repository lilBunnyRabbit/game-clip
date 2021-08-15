pub fn lz77(buffer: Vec<u8>) {
  let sb_size = 6;
  let mut sb: (u64, u64) = (0, 0); // Search buffer = (start index, end index)
  let mut triplets: Vec<(u64, u64, u8)> = Vec::new();
  for i in 0..buffer.len() {
    if i == 0 || !in_search_buffer(&buffer, sb, i) {
      triplets.push((0, 0, buffer[i]));
    }

    sb.1 = i as u64;
    if sb.1 - sb.0 > sb_size {
      sb.0 = sb.1 - sb_size;
    }
  }

  println!("Triplets: {:?}", triplets);
}

fn in_search_buffer(buffer: &Vec<u8>, sb: (u64, u64), index: usize) -> bool {
  for i in sb.0..sb.1 {
    if buffer[i as usize] == buffer[index] {
      return true;
    }
  }

  false
}