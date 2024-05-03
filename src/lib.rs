pub fn get_diamond(c: char) -> Vec<String> {
    let mut diamond: Vec<String> = Vec::new();
    let size = (c as u8 - b'A' + 1) as usize;
    
    for i in 0..size {
        let mut row = vec![' '; size * 2 - 1];
        let ch = (b'A' + i as u8) as char;
        let index = size - 1 - i;
        row[index] = ch;
        row[size + i - 1] = ch;
        diamond.push(row.iter().collect());
    }

    for i in (0..size - 1).rev() {
        let row = diamond[i].clone();
        diamond.push(row);
    }

    diamond
}
