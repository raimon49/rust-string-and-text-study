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

        let mut choco = "chocolate".to_string();
        assert_eq!(choco.drain(3..6).collect::<String>(), "col");
        assert_eq!(choco, "choate");
        let mut winston = "Churchill".to_string();
        winston.drain(2..6);
        assert_eq!(winston, "Chill");
    }
    {
        let haystack = "One fine day, in the middle of the night";
        assert_eq!(haystack.find(','), Some(12));
        assert_eq!(haystack.find("night"), Some(35));
        assert_eq!(haystack.find(char::is_whitespace), Some(3)); // パターンとしてのFnMut(char) -> boolは、クロージャがtrueを返す1文字にマッチ
        assert_eq!("April 2".find(char::is_numeric), Some(6));   // 同上（数字にマッチ）

        assert_eq!("## Elephants"
                   .trim_start_matches(|ch: char| ch == '#' || ch.is_whitespace()),
                   "Elephants");

        let code = "\t    function noodle() { ";
        assert_eq!(code.trim_start_matches(&[' ', '\t'] as &[char]), // char値のスライスは配列リテラルで書いた場合、型を合わせるためas式が必要
                   "function noodle() { ");

        assert!(haystack.contains("middle"));
        assert!(haystack.starts_with("One"));
        assert!(haystack.ends_with("night"));

        // 見つかったバイトオフセットiをSome(i)として返す
        let quip = "We also know there are known unknowns";
        assert_eq!(quip.find("know"), Some(8));
        assert_eq!(quip.rfind("know"), Some(31));
        assert_eq!(quip.find("ya know"), None);
        assert_eq!(quip.rfind(char::is_uppercase), Some(0));

        assert_eq!("The only thing we have to fear is fear itself"
                   .replace("fear", "spin"),
                   "The only thing we have to spin is spin itself");
        assert_eq!("`Borrow` and `BorrowMut`"
                   .replace(|ch:char| !ch.is_alphanumeric(), ""),
                   "BorrowandBorrowMut");
        assert_eq!("This is a soup" // replacen()は3つ目の引数で指定された回数しか置換しない
                   .replacen('s', "S", 2),
                   "ThiS iS a soup");
    }
    {
        // slice.char_indices()はslice上の各文字とバイトオフセット位置を生成するイテレータを返す
        assert_eq!("Élan".char_indices().collect::<Vec<_>>(),
                   vec![(0, 'É'), // 1文字だが2バイトの長さがあり、次のオフセットは+2される
                        (2, 'l'),
                        (3, 'a'),
                        (4, 'n')]);
        // slice.bytes()はslice上の各バイトを生成するイテレータを返す（UTF-8エンコードされた値はそのまま返す）
        assert_eq!("Élan".bytes().collect::<Vec<_>>(),
                   vec![195, 137, b'l', b'a', b'n']);

        assert_eq!("jimb:1000:Jim Blandy:".split(':').collect::<Vec<_>>(),
                   vec!["jimb", "1000", "Jim Blandy", ""]); // 最後のpattern「：」で分割された後ろに空文字列が生成

        assert_eq!("127.0.0.1 localhost\n\
                    127.0.0.1 www.reddit.com\n"
                   .split_terminator('\n').collect::<Vec<_>>(),
                   vec!["127.0.0.1 localhost",
                        "127.0.0.1 www.reddit.com"]); // 最後の""は生成されていない点がsplit()と異なる点に注意

        // split_whitespace()のホワイトスペース定義はchar::is_whitespaceに準じる
        let poem = "This  is  just  to say\n\
                    I have eaten\n\
                    the plumus\n\
                    again\n";
        assert_eq!(poem.split_whitespace().collect::<Vec<_>>(),
                   vec!["This", "is", "just", "to", "say",
                        "I", "have", "eaten",
                        "the", "plumus",
                        "again"]);

        assert_eq!("\t*.rs  ".trim(), "*.rs");
        assert_eq!("\t*.rs  ".trim_start(), "*.rs  ");
        assert_eq!("\t*.rs  ".trim_end(), "\t*.rs");
    }
}
