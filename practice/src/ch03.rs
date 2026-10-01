// 練習問題１
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

