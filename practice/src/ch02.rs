// u32は0以上の整数
pub fn tip_percentage(people: u32) -> u32 {
    if people == 0 {
        0
    } else if people <= 5 {
        10
    } else {
        20
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn zero_people() {
        // 入力とアウトプットを揃えて書いておく
        assert_eq!(tip_percentage(0),0);
    }

    #[test]
    fn three_people() {
        assert_eq!(tip_percentage(3),10);
    }
}


