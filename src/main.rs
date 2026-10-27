const GRID_WIDTH: usize = 32;
const GRID_HEIGHT: usize = 32;

fn advance_stage(
    current_environment: &[Vec<bool>],
    next_environment: &mut [Vec<bool>],
    neighbourhood: &[[i64; 2]],
) {
    for y in 0..GRID_HEIGHT {
        for x in 0..GRID_WIDTH {
            let mut live_neighbours = 0;

            for [dx, dy] in neighbourhood {
                let nx = x as i64 + dx;
                let ny = y as i64 + dy;

                // Is neightbout inside the grid?
                if nx >= 0 && nx < GRID_WIDTH as i64 && ny >= 0 && ny < GRID_HEIGHT as i64 {
                    if current_environment[ny as usize][nx as usize] {
                        live_neighbours += 1;
                    }
                }
            }

            let alive = current_environment[y][x];

            next_environment[y][x] = if alive {
                if live_neighbours < 2 {
                    false
                } else if live_neighbours <= 3 {
                    true
                } else {
                    false
                }
            } else {
                live_neighbours == 3
            };
        }
    }
}

fn output_stage(current_environment: &[Vec<bool>]) {
    print!("\x1B[2J\x1B[H");
    for row in current_environment {
        for cell in row {
            print!("{}", if *cell { '█' } else { ' ' });
        }
        println!();
    }
}

fn main() {
    // Initialise environment as all false
    let mut current_environment: Vec<Vec<bool>> = vec![vec![false; GRID_WIDTH]; GRID_HEIGHT];
    let mut next_environment: Vec<Vec<bool>> = vec![vec![false; GRID_WIDTH]; GRID_HEIGHT];

    // Pulsar (Top Left)
    current_environment[2][4] = true;
    current_environment[2][5] = true;
    current_environment[2][6] = true;
    current_environment[2][10] = true;
    current_environment[2][11] = true;
    current_environment[2][12] = true;
    current_environment[4][2] = true;
    current_environment[4][7] = true;
    current_environment[4][9] = true;
    current_environment[4][14] = true;
    current_environment[5][2] = true;
    current_environment[5][7] = true;
    current_environment[5][9] = true;
    current_environment[5][14] = true;
    current_environment[6][2] = true;
    current_environment[6][7] = true;
    current_environment[6][9] = true;
    current_environment[6][14] = true;
    current_environment[7][4] = true;
    current_environment[7][5] = true;
    current_environment[7][6] = true;
    current_environment[7][10] = true;
    current_environment[7][11] = true;
    current_environment[7][12] = true;
    current_environment[9][4] = true;
    current_environment[9][5] = true;
    current_environment[9][6] = true;
    current_environment[9][10] = true;
    current_environment[9][11] = true;
    current_environment[9][12] = true;
    current_environment[10][2] = true;
    current_environment[10][7] = true;
    current_environment[10][9] = true;
    current_environment[10][14] = true;
    current_environment[11][2] = true;
    current_environment[11][7] = true;
    current_environment[11][9] = true;
    current_environment[11][14] = true;
    current_environment[12][2] = true;
    current_environment[12][7] = true;
    current_environment[12][9] = true;
    current_environment[12][14] = true;
    current_environment[14][4] = true;
    current_environment[14][5] = true;
    current_environment[14][6] = true;
    current_environment[14][10] = true;
    current_environment[14][11] = true;
    current_environment[14][12] = true;

    // Choose neighbourhood indexing -> Futureproofing
    let indexing: &str = "moore";
    let neighbourhood: &[[i64; 2]] = match indexing {
        "moore" => &[
            [-1, 1],
            [0, 1],
            [1, 1],
            [-1, 0],
            [1, 0],
            [-1, -1],
            [0, -1],
            [1, -1],
        ],
        "von neumann" => &[[0, 1], [-1, 0], [1, 0], [0, -1]],
        _ => todo!(),
    };

    // Main Loop
    loop {
        output_stage(&current_environment);

        //let mut input = String::new();
        //std::io::stdin().read_line(&mut input).unwrap();

        advance_stage(&current_environment, &mut next_environment, neighbourhood);
        std::mem::swap(&mut current_environment, &mut next_environment);

        std::thread::sleep(std::time::Duration::from_millis(128));
    }
}
