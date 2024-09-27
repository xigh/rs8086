use tracing::info;

pub fn dump(buf: &[u8], origin: usize, len: usize) {
    let len = len.min(buf.len());
    let mut i = 0;
    let mut line = vec![];

    while i < len {
        line.push(format!("{:08X} ", origin + i));
        for j in 0..16 {
            if i + j < len {
                line.push(format!("{:02X} ", buf[i + j]));
            } else {
                line.push(format!("   "));
            }
        }
        line.push(format!(" "));
        for j in 0..16 {
            if i + j < len {
                let c = buf[i + j];
                if c >= 32 && c < 127 {
                    line.push(format!("{}", c as char));
                } else {
                    line.push(format!("."));
                }
            } else {
                line.push(format!(" "));
            }
        }
        info!("{}", line.join(""));
        line.clear();
        i += 16;
    }
}
