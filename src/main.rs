use std::{
    io::{self, Read},
    thread,
    time::Duration,
};

struct Winsize {
    ws_row: u16,
    ws_col: u16,
}

fn size() -> Option<(usize, usize)> {
    unsafe {
        let mut ws = Winsize {
            ws_row: 0,
            ws_col: 0,
        };

        for fd in [libc::STDOUT_FILENO, libc::STDERR_FILENO, libc::STDIN_FILENO]
        {
            if libc::ioctl(fd, libc::TIOCGWINSZ, &mut ws) == 0 && ws.ws_col != 0
            {
                return Some((ws.ws_row as usize, ws.ws_col as usize));
            }
        }
    }
    None
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let lines: Vec<&str> = input.lines().collect();

    if lines.is_empty() {
        return;
    }

    let art_height = lines.len();

    let art_width = lines
        .iter()
        .map(|line| line.chars().count())
        .max()
        .unwrap_or(0);

    let (term_height, term_width) = size().unwrap_or((24, 80));

    let top_padding = term_height.saturating_sub(art_height) / 2;
    let left_padding = term_width.saturating_sub(art_width) / 2;

    print!("\x1B[2J\x1B[H");
    print!("\x1b[?25l");

    for _ in 0..top_padding {
        println!();
    }

    let indent = " ".repeat(left_padding);

    for line in lines {
        println!("{indent}{line}");
    }

    for _ in 0..top_padding {
        println!();
    }

    thread::sleep(Duration::from_secs(5));

    print!("\x1b[2J");
}
