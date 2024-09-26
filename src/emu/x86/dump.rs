pub fn dump(buf: &[u8], origin: usize, len: usize) {
    let len = len.min(buf.len());
    let mut i = 0;
    while i < len {
        print!("{:08X} ", origin + i);
        for j in 0..16 {
            if i + j < len {
                print!("{:02X} ", buf[i + j]);
            } else {
                print!("   ");
            }
        }
        print!(" ");
        for j in 0..16 {
            if i + j < len {
                let c = buf[i + j];
                if c >= 32 && c < 127 {
                    print!("{}", c as char);
                } else {
                    print!(".");
                }
            } else {
                print!(" ");
            }
        }
        println!();
        i += 16;
    }
}
