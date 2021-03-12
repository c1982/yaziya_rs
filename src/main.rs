fn main() {
    let _birler = [
        "", "bir", "iki", "üç", "dört", "beş", "altı", "yedi", "sekiz", "dokuz",
    ];

    let _onlar = [
        "", "on", "yirmi", "otuz", "kırk", "elli", "altmış", "yetmiş", "seksen", "doksan",
    ];

    let _yuzler = [
        "",
        "yüz",
        "ikiyüz",
        "üçyüz",
        "dörtyüz",
        "beşyüz",
        "altıyüz",
        "yediyüz",
        "sekizyüz",
        "dokuzyüz",
    ];

    let _binler = [
        "",
        "bin",
        "milyon",
        "milyar",
        "trilyon",
        "katrilyon",
        "kentilyon",
        "seksilyon",
        "septilyon",
        "oktilyon",
        "nobilyon",
        "desilyon",
    ];
}

fn cevir(number: f64) -> &'static str {
    let b: i32 = 0;

    if number < 0.0 {
        //TODO: error
    }

    let number_str: String = number.to_string();

    if number_str.len() > 34 {
        //TODO: error
    }

    ""
}

pub fn padding3(text: String) -> String {
    let mut s = String::from(text);
    loop {
        let m: usize = s.len() % 3;
        if m > 0 {
            s.push_str("0")
        } else if m == 0 {
            break;
        }
    }
    s
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_padding3() {
        assert_eq!(padding3("10".to_string()), "100");
    }
}
