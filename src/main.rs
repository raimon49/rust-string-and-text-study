fn main() {
    {
        fn latin1_to_char(latin1: u8) -> char {
            latin1 as char
        }

        fn char_to_latin1(c: char) -> Option<u8> {
            if c as u32 <= 0xff {
                Some(c as u8)
            } else {
                None
            }
        }

        // 文字'*'はコードポイント42であり、Latin-1コードブロックの範囲内
        // UnicodeはLatin-1のスーパーセットのため、相互の変換には変換テーブル不要
        assert_eq!(latin1_to_char(42), '*');
        assert_eq!(char_to_latin1('*'), Some(42));

        assert_eq!("うどん: udon".as_bytes(),
                   &[0xe3, 0x81, 0x86, // う
                     0xe3, 0x81, 0xa9, // ど
                     0xe3, 0x82, 0x93, // ん
                     0x3a, 0x20, 0x75, 0x64, 0x6f, 0x6e //: udon
                   ]);

        // char型の分類メソッド
        assert!('4'.is_numeric());
        assert!('⑧'.is_numeric());
        assert!('q'.is_alphabetic());
        assert!('七'.is_alphabetic());
        assert!('9'.is_alphanumeric()); // 数字かアルファベットならtrue
        assert!(!'*'.is_alphanumeric());
        assert!(' '.is_whitespace());
        assert!('\n'.is_whitespace());
        assert!('\u{A0}'.is_whitespace());
        assert!('\n'.is_control());     // 制御文字ならtrue
        assert!('\u{85}'.is_control());

        assert_eq!('8'.to_digit(10), Some(8));  // 10進数で数値に変換できればSome(N)が、できなければNoneが返る
        assert_eq!('F'.to_digit(16), Some(15)); // 16進数で数値に変換できればSome(N)が、できなければNoneが返る
        assert_eq!(std::char::from_digit(15, 16), Some('f')); // 上記の逆変換
        assert!(char::is_digit('f', 16));

        let mut upper = 's'.to_uppercase();  // 大文字化された文字列を生成するイテレータを返す
        assert!(!'s'.is_uppercase());
        assert_eq!(upper.next(), Some('S')); // イテレータから1文字取り出す
        assert_eq!(upper.next(), None);      // それ以上なければNoneが返る

        let ch = 'İ'; // トルコ語のドット付きiはiの後ろに\u{307} COMBING DOT ABOVEを付けたものと定義されている
        let mut lower = ch.to_lowercase();
        assert_eq!(lower.next(), Some('i'));
        assert_eq!(lower.next(), Some('\u{307}'));
        assert_eq!(lower.next(), None);

        assert_eq!('B' as u32, 66);
        assert_eq!('二' as i8, -116); // 上位ビットは丸められる
        assert_eq!(char::from(66), 'B');
        assert_eq!(std::char::from_u32(0xd800), None); // UTF-16用に予約された領域
    }
    {
        let s1 = "literal text".to_string();
        let s2 = String::from("literal text");
        assert_eq!(s1, s2);

        let spacey = "man hat tan";
        let spaceless: String = spacey.chars().filter(|c| !c.is_whitespace()).collect();
        assert_eq!(spaceless, "manhattan");

        let onwed_str: &str = "str";
        let copied_str: String = onwed_str.to_owned(); // &str型はCloneを実装できないがslice.to_owned()でコピーした新たなStringが取得できる
        assert_eq!(onwed_str, &copied_str);
    }
    {
        let full = "bookkeeping";
        assert_eq!(&full[..4], "book");
        assert_eq!(&full[5..], "eeping");
        assert_eq!(&full[2..4], "ok");
        assert_eq!(full[..].len(), 11);
        assert_eq!(full[5..].contains("boo"), false);
        assert_eq!(full[0..0].is_empty(), true);

        assert_eq!("abcd".split_at(2), ("ab", "cd"));
        assert_eq!("abcd".is_char_boundary(1), true); // 指定されたバイトオフセットが文字境界ならtrue
        assert_eq!("ああ".is_char_boundary(1), false);
    }
    {
        let mut s = "abcd".to_string();
        s.push('e');
        assert_eq!("abcde", s);
        s.push_str("fghij");
        assert_eq!("abcdefghij", s);

        let mut also_spaceless = "con".to_string();
        also_spaceless.extend("tri but ion".split_whitespace());
        assert_eq!(also_spaceless, "contribution");

        let mut x = "xz".to_string();
        x.insert(1, 'y');
        assert_eq!(x, "xyz");

        let mut z = "z".to_string();
        z.insert_str(0, "xy");
        assert_eq!(z, "xyz");
    }
    {
        use std::fmt::Write;

        let mut letter = String::new();
        // write!/writeln!マクロは出力ストリームに書き出すためResultを返す
        // see) https://doc.rust-lang.org/std/macro.writeln.html
        writeln!(letter, "Whose {} these are I think I know", "rutabagas").unwrap(); // .unwrap(); でなく ?;だとコンパイラが警告を出す
        writeln!(letter, "His house is in the village throgh;").unwrap();
        assert_eq!(letter, "Whose rutabagas these are I think I know\n\
                            His house is in the village throgh;\n");

        let left = "partners".to_string();
        let mut right = "crime".to_string();
        assert_eq!(left + " in " + &right, "partners in crime");

        right += " doesn't pay";
        assert_eq!(right, "crime doesn't pay");

        right.truncate(4);
        assert_eq!(right, "crim");

        assert_eq!(right.remove(1), 'r');
        assert_eq!(right, "cim");

        right.clear();
        assert_eq!(right.len(), 0);
    }
}
