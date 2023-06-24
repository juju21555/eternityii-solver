use std::collections::HashMap;

fn solve(rows: HashMap<String, Vec<u32>>, cols: HashMap<u32, Vec<String>>, solution: Vec<String>) -> Vec<String> {
    if cols.is_empty() {
        return solution;
    }
    else {




    }
}

fn main() {

    let mut rows: HashMap<String, Vec<u32>> = HashMap::new();
    let mut cols: HashMap<u32, Vec<String>> = HashMap::new();

    let n = 3;
    let k = 4;

    let mut pieces = [
        [0, 1, 2, 3], [1, 0, 0, 1], [2, 1, 2, 3],
        [0, 1, 2, 3], [1, 3, 0, 2], [1, 3, 2, 3],
        [2, 3, 3, 0], [1, 2, 2, 3], [0, 3, 0, 2]
    ];

    for rotation in 0..4 {
        for row in 0u32..(n*n*n*n) {
            let which_piece = row/(n*n);
            let which_empl = row%(n*n);

            let mut v: Vec<u32> = Vec::new();
            v.push(which_piece);
            v.push(n*n+which_empl);

            let piece = pieces[which_piece as usize];
            let i_e = which_empl/n;
            let j_e = which_empl%n;

            v.push(n*n + n*n + k*(i_e*n+j_e) + piece[(0+rotation)%4 as usize]);
            for i in 0..k {
                if i != piece[(2+rotation)%4 as usize] {
                    v.push(n*n + n*n + k*((i_e*n+j_e+n)%(n*n)) + i);
                }
            }

            v.push(n*n + n*n + k*(j_e*n+i_e) + k*n*n + piece[(1+rotation)%4 as usize]);
            for i in 0..k {
                if i != piece[(3+rotation)%4 as usize] {
                    v.push(n*n + n*n + k*n*n + k*((j_e*n+i_e+n)%(n*n)) + i);
                }
            }
            rows.insert(format!("P{which_piece}_E{which_empl}_R{rotation}"), v);
        }
    }

    for i in 0u32..(2*n*n + k*2*n*n) {
        cols.insert(i, Vec::new());
    }
    for (s, v) in rows.iter_mut(){
        for j in v.iter() {
            cols.get_mut(j).expect("").push(s.clone());
        }
    }
    for (s, v) in cols.iter() {
        println!("{} : {:?}", s, v);
    }

}