use colored::{control, Colorize};
use termsize;
enum Color {
    Blue,
    Yellow,
}
struct Symbol {
    color: Color,
}
pub fn trident() {
    termsize::get().map(|size| {
        let schema = [
            [0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 0],
            [0, 1, 0, 0, 1, 0, 1, 0, 0, 1, 0],
            [0, 1, 1, 0, 0, 1, 0, 0, 1, 1, 0],
            [0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0],
            [0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0],
            [0, 1, 1, 0, 0, 1, 0, 0, 1, 1, 0],
            [0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0],
            [0, 1, 0, 0, 1, 1, 1, 0, 0, 1, 0],
            [0, 1, 1, 1, 1, 0, 1, 1, 1, 1, 0],
            [0, 0, 0, 0, 1, 1, 1, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0],
        ];
        let space = 1;
        let line_height = 2;
        let x: i32 = i32::from(size.cols / 11 / space);
        if x < line_height * 2 {
            println!("Your terminal is too narrow for our wide plans!");
            return;
        }
        control::set_virtual_terminal(true).unwrap();
        for row in schema {
            for _i in 1..x / line_height {
                //let mut line = String::new();
                for char in row {
                    let symbol = Symbol {
                        color: if char == 0 {
                            Color::Blue
                        } else {
                            Color::Yellow
                        },
                    };
                    let ch = String::from(["X", "O"][char]);
                    let block = std::iter::repeat(ch)
                        .take(x.try_into().unwrap())
                        .collect::<String>();
                    let colored = match symbol.color {
                        Color::Blue => block.truecolor(0, 87, 184),
                        Color::Yellow => block.truecolor(254, 221, 0),
                    };
                    //line.push_str(&block);
                    print!("{}", colored);
                }
                //println!("{}", line)
                print!("\n")
            }
        }
    });
}
