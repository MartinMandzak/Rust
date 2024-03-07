
fn printout(board: &[bool]){
    let mut counter: i32 = 0;
    for cell in board{
        if counter % ROWS as i32 == 0{println!();}
        print!("{} ",*cell as u8);
        counter +=1;
    }
}


const ROWS: usize = 10;
const COLS: usize = 10;
fn main() {
    let mut board: [bool; ROWS*COLS] = [false; ROWS*COLS];
    printout(&board);
}



