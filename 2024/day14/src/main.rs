use input_parsing::read_input;

fn main() {
    let input = read_input("./src/example.txt");
    let mut lines = input.lines();
    let (x, y) = lines.next().unwrap().split_once(" ").unwrap();
    let (x, y) = (x.parse::<i64>().unwrap(), y.parse::<i64>().unwrap());

    println!("{} {}", x, y);
    let mut bots = vec![];
    for line in lines {
        let (p, v) = line.split_once(" ").unwrap();
        let (p, v) = (p.split_once("=").unwrap().1, v.split_once("=").unwrap().1);
        let (p, v) = (p.split_once(",").unwrap(), v.split_once(",").unwrap());
        let p = (p.1.parse::<i64>().unwrap(), p.0.parse::<i64>().unwrap());
        let v = (v.1.parse::<i64>().unwrap(), v.0.parse::<i64>().unwrap());
        bots.push((p, v));
    }
    for i in 0..10000 {
        for (p, v) in bots.iter_mut() {
            *p = ((((*p).0 + (*v).0) + x) % x, ((*p).1 + (*v).1 + y) % y);
        }
        if i == 99 {
            count_part_1(&bots,x,y);
        }
        let mut grid = vec![vec![0; y as usize]; x as usize];
        for (p, _) in bots.iter() {
            grid[p.0 as usize][p.1 as usize] += 1;
        }
        let mut maybe_tree = false;
        for row in &grid {
            let mut start = 0;
            for col in 0..row.len() {
                if row[col] >= 1 {
                    if col - start >= 30 {
                        maybe_tree = true;
                        break;
                    }
                } else {
                    start = col;
                }
            }
        }
        if maybe_tree {
            println!("Iteration {}", i + 1 );
            for row in grid {
                for cell in row {
                    print!("{} ", if cell == 0 { "." } else { "#" });
                }
                println!();
            }
        }
    }
}

fn count_part_1(bots: &Vec<((i64, i64), (i64, i64))>, x: i64, y: i64) {
    let x_mid = x / 2;
    let y_mid = y / 2;
    let mut top_left = vec![];
    let mut top_right = vec![];
    let mut bottom_left = vec![];
    let mut bottom_right = vec![];

    for &bot in bots {
        let (pos, _) = bot;
        if pos.0 < x_mid {
            if pos.1 < y_mid {
                top_left.push(bot);
            } else if pos.1 > y_mid {
                top_right.push(bot);
            }
        } else if pos.0 > x_mid {
            if pos.1 < y_mid {
                bottom_left.push(bot);
            } else if pos.1 > y_mid {
                bottom_right.push(bot);
            }
        }
    }

    println!("Top left: {} bots", top_left.len());
    println!("Top right: {} bots", top_right.len());
    println!("Bottom left: {} bots", bottom_left.len());
    println!("Bottom right: {} bots", bottom_right.len());
    println!(
        "Part 1: {}",
        top_left.len() * top_right.len() * bottom_left.len() * bottom_right.len()
    );
}
