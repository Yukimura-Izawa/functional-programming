// 練習問題１ (first two)
// let firstTwo (list: vec<>) -> vec<> {
//    [list[0], list[1]]
//}

//次の回答
//fn first_two(string: &[String])-> Vec<String> {
//    vec![string[0], string[1]];
//}

//次の回答
fn first_two(string: &[String])-> Vec<String> {
    vec![string[0].clone(), string[1].clone()]
}
// リストを借りているだけなので、中の String を持ち去れない。だから clone() で複製するなので、clone()でコピーする必要がある。

//
// 練習問題2 (last two)
//fn last_two(list: &[String])-> Vec<String> {
//    vec![list[list.length()-2], list[list.length()-1]]
//}

//次の回答
fn last_two(list: &[String])-> Vec<String> {
    vec![list[list.len()-2].clone(), list[list.len()-1].clone()]
}

//別解
fn last_two_athoer(list: &[String])-> Vec<String>{
    list[list.len()-2..].to_vec()
}
// [n..]はn番目から最後まで。(list.len()-2)番目から最後までってこと。
// to_vec()は借りているスライスから、新しい Vec を作るメソッド

// 練習問題3 movedfirsttwototheend
//fn moved_first_two_to_the_end(list: &[String])->Vec<String>{
//    list.to_vec().
//}

// 宣言型でかきたい
// 切り取って、後ろにくっつける
// 答えを参考に
fn moved_first_two_to_the_end(list: &[String])->Vec<String>{
    let first_two = &list[..2]; //最初から2こ
    let rest = &list[2..]; //2番目から後
    [rest,first_two].concat() //concat() : 中身を複製して、新しい Vec を作る
    // concat()はいくつかのスライスを順番に繋げて新しいvec作っている
}

// 練習問題4 (insertedforeLast)
// fn inserted_before_last(list: &[String])-> Vec<String>{
//    let last = &list[..list.len()-1];
//    [last, element, list[list.len()]].concat 
// }

fn inserted_before_last(list: &[String], item:&str)-> Vec<String>{
    let without_last = &list[..list.len()-1]; //最後以外
    let last = &list[list.len()-1..]; //最後
    [without_last, &[item.to_string()], last].concat()
}
// itemは&strなのでStringの型にする。さらに、スライスにして、借用の&をつける