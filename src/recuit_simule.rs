use rand::Rng;

const N: usize = 4;
const K: usize = 4;


fn print_puzzle(puzzle: [[[i8; 4]; N]; N]) {
    for i in 0..N {
        for _ in 0..N {
            print!("---------");
        }
        println!();
        for j in 0..N {
            print!("|   {}   |", puzzle[i][j][0]);
        }
        println!();
        for j in 0..N {
            print!("| {} - {} |", puzzle[i][j][3], puzzle[i][j][1]);
        }
        println!();
        for j in 0..N {
            print!("|   {}   |", puzzle[i][j][2]);
        }
        println!();
        for _ in 0..N {
            print!("---------");
        }
        println!();
    }
}

fn score_swap_rotate(puzzle: [[[i8; 4]; N]; N], i1: usize, j1: usize, i2: usize, j2: usize, r1: usize, r2: usize) -> f64 {

    let mut score = 0.0;
    for (vi, vj, c) in [(0, N-1, 0), (1, 0, 1), (0, 1, 2), (N-1, 0, 3)] {

        let ni = (i1+vi)%N;
        let nj = (j1+vj)%N;

        if puzzle[j1][i1][c] == puzzle[nj][ni][(c+2)%4] {
            score -= 1.0;
        }
        if puzzle[j2][i2][(r2+c)%4] == puzzle[nj][ni][(c+2)%4] {
            score += 1.0;
        }

        let ni = (i2+vi)%N;
        let nj = (j2+vj)%N;

        if puzzle[j2][i2][c] == puzzle[nj][ni][(c+2)%4] {
            score -= 1.0;
        }
        if puzzle[j1][i1][(r1+c)%4] == puzzle[nj][ni][(c+2)%4] {
            score += 1.0;
        }
    }
    return score
}

fn score_swap(puzzle: [[[i8; 4]; N]; N], i1: usize, j1: usize, i2: usize, j2: usize) -> f64 {

    let mut score = 0.0;
    for (vi, vj, c) in [(0, N-1, 0), (1, 0, 1), (0, 1, 2), (N-1, 0, 3)] {

        let ni = (i1+vi)%N;
        let nj = (j1+vj)%N;

        if puzzle[j1][i1][c] == puzzle[nj][ni][(c+2)%4] {
            score -= 1.0;
        }
        if puzzle[j2][i2][c] == puzzle[nj][ni][(c+2)%4] {
            score += 1.0;
        }

        let ni = (i2+vi)%N;
        let nj = (j2+vj)%N;

        if puzzle[j2][i2][c] == puzzle[nj][ni][(c+2)%4] {
            score -= 1.0;
        }
        if puzzle[j1][i1][c] == puzzle[nj][ni][(c+2)%4] {
            score += 1.0;
        }
    }
    return score
}


fn score_rotate(puzzle: [[[i8; 4]; N]; N], i1: usize, j1: usize, r1: usize) -> f64 {

    let mut score = 0.0;
    for (vi, vj, c) in [(0, N-1, 0), (1, 0, 1), (0, 1, 2), (N-1, 0, 3)] {

        let ni = (i1+vi)%N;
        let nj = (j1+vj)%N;

        if puzzle[j1][i1][c] == puzzle[nj][ni][(c+2)%4] {
            score -= 1.0;
        }
        if puzzle[j1][i1][(r1+c)%4] == puzzle[nj][ni][(r1+c+2)%4] {
            score += 1.0;
        }
    }
    return score
}


fn test_puzzle(puzzle: [[[i8; 4]; N]; N]) -> bool{
    for i in 0..N {
        for j in 0..N {
            if puzzle[i][j][2] != puzzle[(i+1)%N][j][0] {
                return false;
            }
            if puzzle[i][j][0] != puzzle[(i+N-1)%N][j][2] {
                return false;
            }
            if puzzle[i][j][1] != puzzle[i][(j+1)%N][3] {
                return false;
            }
            if puzzle[i][j][3] != puzzle[i][(j+N-1)%N][1] {
                return false;
            }

        }
    }
    return true;
}

fn rotate_piece(piece:[i8; 4], r: usize) -> [i8; 4] {
    let mut n_piece = [0i8; 4];
    for i in 0..4 {
        n_piece[i] = piece[(r+i)%4];
    }
    return n_piece;
}

fn generate_solved_puzzle() -> [[[i8; 4]; N]; N] {
    let mut rng = rand::thread_rng();
    let mut puzzle: [[[i8; 4]; N]; N] = [[[0i8; 4]; N]; N];

    for i in 0..N {
        for j in 0..N {
            for (vi, vj, p) in [(0, N-1, 0), (1, 0, 1), (0, 1, 2), (N-1, 0, 3)] {
                let c: i8 = rng.gen_range(0..K as i8);
                puzzle[j][i][p] = c;
                let ni = (i+vi)%N;
                let nj = (j+vj)%N;
                puzzle[nj][ni][(p+2)%4] = c;
            }
        }
    }

    return puzzle;
}

fn shuffle_puzzle(mut puzzle: [[[i8; 4]; N]; N]) -> [[[i8; 4]; N]; N] {

    let mut rng = rand::thread_rng();

    for _ in 0..250 {
        let i1: usize = rng.gen_range(0..N);
        let j1: usize = rng.gen_range(0..N);
        let i2: usize = rng.gen_range(0..N);
        let j2: usize = rng.gen_range(0..N);
        let r1: usize = rng.gen_range(1..4);
        let r2: usize = rng.gen_range(1..4);

        let piece1: [i8; 4] = puzzle[j1][i1];
        let piece2: [i8; 4] = puzzle[j2][i2];
        puzzle[j1][i1] = rotate_piece(piece2, r2);
        puzzle[j2][i2] = rotate_piece(piece1, r1);

    }

    return puzzle
}


fn main() {
    let mut rng = rand::thread_rng();

    let mut pieces = shuffle_puzzle(generate_solved_puzzle());
    //print_puzzle(pieces);

    /*
    let mut pieces: [[[i8; 4]; N]; N] = [[[0i8; 4]; N]; N];
    if true { //0 Vert 1 Bleu 2 Jaune 3 Rouge
    pieces = [
    [[0, 1, 2, 3], [1, 0, 0, 1], [2, 1, 2, 3]],
    [[0, 1, 2, 3], [1, 3, 0, 2], [1, 3, 2, 3]],
    [[2, 3, 3, 0], [1, 2, 2, 3], [0, 3, 0, 2]]
    ];
    }*/


    let mut temperature: f64 = 100000.0;
    //let mut temperature: f64 = 0.3;

    print_puzzle(pieces);

    while !test_puzzle(pieces) {
        temperature *= 0.999;
        if temperature < 0.01 {
            temperature = 5000.0;
        }

        let rand_float: f64 = rng.gen();
        if false { // Recuit simulé normal swap+rotate en même temps
            let i1: usize = rng.gen_range(0..N);
            let j1: usize = rng.gen_range(0..N);
            let i2: usize = rng.gen_range(0..N);
            let j2: usize = rng.gen_range(0..N);
            let r1: usize = rng.gen_range(1..4);
            let r2: usize = rng.gen_range(1..4);

            let score: f64 = score_swap_rotate(pieces, i1, j1, i2, j2, r1, r2);
            let val: f64 = -score.abs()/temperature;
            let proba: f64 = val.exp();

            let r_float: f64 = rng.gen();
            if (score > 0.0) || (r_float < proba) {
                let piece1: [i8; 4] = pieces[j1][i1];
                let piece2: [i8; 4] = pieces[j2][i2];
                pieces[j1][i1] = rotate_piece(piece2, r2);
                pieces[j2][i2] = rotate_piece(piece1, r1);
            }
        }
        else if true { // Recuit simulé temperature constante
            let i1: usize = rng.gen_range(0..N);
            let j1: usize = rng.gen_range(0..N);
            let i2: usize = rng.gen_range(0..N);
            let j2: usize = rng.gen_range(0..N);
            let r1: usize = rng.gen_range(1..4);
            let r2: usize = rng.gen_range(1..4);

            let score: f64 = score_swap_rotate(pieces, i1, j1, i2, j2, r1, r2);
            let val: f64 = -score.abs()/temperature;
            let proba: f64 = val.exp();

            let r_float: f64 = rng.gen();
            if (score > 0.0) || (r_float < proba) {
                let piece1: [i8; 4] = pieces[j1][i1];
                let piece2: [i8; 4] = pieces[j2][i2];
                pieces[j1][i1] = rotate_piece(piece2, r2);
                pieces[j2][i2] = rotate_piece(piece1, r1);
            }
        }
        else { // Recuit simulé normal swap ou rotate
            if rand_float < 0.3 {
                let i1: usize = rng.gen_range(0..N);
                let j1: usize = rng.gen_range(0..N);
                let r1: usize = rng.gen_range(1..4);

                let score: f64 = score_rotate(pieces, i1, j1, r1);
                let val: f64 = -score.abs()/0.5;
                let proba: f64 = val.exp();

                let r_float: f64 = rng.gen();
                if (score > 0.0) || (r_float < proba) {
                    let piece1: [i8; 4] = pieces[j1][i1];
                    pieces[j1][i1] = rotate_piece(piece1, r1);
                }
                //println!("{} | {} | {}", score, proba, temperature);
            }
            else {
                let i1: usize = rng.gen_range(0..N);
                let j1: usize = rng.gen_range(0..N);
                let i2: usize = rng.gen_range(0..N);
                let j2: usize = rng.gen_range(0..N);

                let score: f64 = score_swap(pieces, i1, j1, i2, j2);
                let val: f64 = -score.abs()/temperature;
                let proba: f64 = val.exp();

                let r_float: f64 = rng.gen();
                if (score > 0.0) || (r_float < proba) {
                    let piece1: [i8; 4] = pieces[j1][i1];
                    let piece2: [i8; 4] = pieces[j2][i2];
                    pieces[j2][i2] = piece1;
                    pieces[j1][i1] = piece2;
                }
            }
        }

        //println!("{}", temperature);
        //print_puzzle(pieces);

    }

    println!("Result : ");

    print_puzzle(pieces);
}
