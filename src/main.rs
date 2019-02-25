#[macro_use]
extern crate t_bang;

use std::io::Read;
use std::io::Write;

use t_bang::*;

struct Stru {
    x: u16,
    y: u16,
}
impl Stru {
    fn f1(a: u32) -> bool {
        a == 0
    }
    fn f2(&self, b: u16) -> Self {
        if b == self.x || b == self.y {
            Stru {
                x: self.x + 1,
                y: self.y + 1,
            }
        } else {
            Stru {
                x: self.x - 1,
                y: self.y - 1,
            }
        }
    }
}

fn main() {
    let s = Stru { x: 23, y: 456 };
    print!("{} {}", Stru::f1(500_000), s.f2(456).x);
}