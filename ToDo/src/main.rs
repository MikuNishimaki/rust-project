use std::io;
// use chrono::NaiveDate;

fn main() {
    println!("やりたいことを選んでください");
    println!("1:追加 2:確認 3:削除 4:編集");

    let mut yarukoto = String::new();

    io::stdin()
        .read_line(&mut yarukoto)
        .expect("errer");

    let yarukoto = yarukoto.trim(); //入力後のエンターで改行してしまうことの対策

    if yarukoto == "1" {
        add();
    };
    if yarukoto == "2" {
        view();
    }
}

// struct Task  {
//     taskname: String,
//     kigen: NaiveDate,
//     yusendo: u32,
//     memo: String,
// }

fn add() {
    println!("タスク名、期限、優先度(高い順に3か2か1)、memoを半角開けて入力してください");
    println!("例：買い物 2024-10-20 3 人参と玉ねぎ");

    
}

fn view() {
    println!("以下が現在のタスクです");
}