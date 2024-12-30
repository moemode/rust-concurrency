use std::sync::mpsc;

fn sum(s: &[i32]) -> i32 {
    return s.iter().sum();
}

fn sum_worker(s: &[i32], tx: mpsc::Sender<i32>) {
    let sum = sum(s);
    tx.send(sum).unwrap();
}

fn main() {
    let (tx, rx) = mpsc::channel();
    let s = vec![7, 2, 8, -9, 4, 0];
    let mid = s.len() / 2;
    let s1 = &s[..mid];
    let s2 = &s[mid..];
    sum_worker(s1, tx.clone());
    sum_worker(s2, tx.clone());
    let x = rx.recv().unwrap();
    let y = rx.recv().unwrap();
    println!("x: {}, y: {}, sum: {}", x, y, x + y);
}
