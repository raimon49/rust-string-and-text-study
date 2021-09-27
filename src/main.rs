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

        // trim_start()（旧:trim_left()）は先頭のホワイトスペースのみ削除
        // trim_end() （旧:trim_right()）は末尾のホワイトスペースのみ削除
        assert_eq!("\t*.rs  ".trim(), "*.rs");
        assert_eq!("\t*.rs  ".trim_start(), "*.rs  ");
        assert_eq!("\t*.rs  ".trim_end(), "\t*.rs");

        // patternにマッチするものを全て削除したサブスライスを返す
        // trim_start_matches()は先頭のpattern全て削除
        // trim_end_matches()は末尾のpattern全て削除
        assert_eq!("001990".trim_matches('0'), "199");
        assert_eq!("001990".trim_start_matches('0'), "1990");
        assert_eq!("001990".trim_end_matches('0'), "00199");
    }
    {
        use std::str::FromStr;

        // Rustの基本型はすべてFromStr traitを実装しておりResultを返す
        assert_eq!(usize::from_str("3628800"), Ok(3628800));
        assert_eq!(f64::from_str("128.5625"), Ok(128.5625));
        assert_eq!(bool::from_str("true"), Ok(true));

        // parseできない文字列はResultがErrとなる
        assert!(f64::from_str("not a float at all").is_err());
        assert!(bool::from_str("TRUE").is_err());

        use std::net::IpAddr;

        let address = IpAddr::from_str("fe80::0000:3ea9:f4ff:fe34:7a50");
        let to_addr = IpAddr::from([0xfe80, 0, 0, 0, 0x3ea9, 0xf4ff, 0xfe34, 0x7a50]);
        assert_eq!(address,
                   Ok(to_addr));

        // sliceからFromStrを実装している任意の型にパースするparse()メソッドでもIpAddr型が得られる
        let other_address = "fe80::0000:3ea9:f4ff:fe34:7a50".parse::<IpAddr>();
        assert_eq!(other_address,
                   Ok(to_addr));

        assert_eq!(format!("{}, wow", "doge"), "doge, wow");
        assert_eq!(format!("{}", true), "true");
        assert_eq!(format!("({:.3}, {:.3})",
                           0.5, f64::sqrt(3.0)/2.0),
                   "(0.500, 0.866)");
        let formatted_addr: String = format!("{}", address.unwrap());
        assert_eq!(formatted_addr, "fe80::3ea9:f4ff:fe34:7a50");
        assert_eq!(other_address.unwrap().to_string(), "fe80::3ea9:f4ff:fe34:7a50"); // std::fmt::Displayを実装している型は自動的にstd::str::ToStringも実装されているため、format!を使わずto_string()を呼んでもよい

        let addresses = vec![
                             IpAddr::from_str("fe80::0000:3ea9:f4ff:fe34:7a50"),
                             IpAddr::from_str("192.168.0.1")
                            ];
        // format!マクロでフォーマット指示子{:?}を使うとstd::fmt::Debugの実装が呼ばれる
        assert_eq!(format!("{:?}", addresses),
                  "[Ok(V6(fe80::3ea9:f4ff:fe34:7a50)), Ok(V4(192.168.0.1))]");
    }
    {
        let good_utf8: Vec<u8> = vec![0xe9, 0x8c, 0x86];
        assert_eq!(String::from_utf8(good_utf8).ok(), Some("錆".to_string()));

        let bad_utf8: Vec<u8> = vec![0x9f, 0xf0, 0xa6, 0x80];
        let result = String::from_utf8(bad_utf8);
        assert!(result.is_err());
        // エラー値からそのまま取り出すことができる
        assert_eq!(result.unwrap_err().into_bytes(),
                  vec![0x9f, 0xf0, 0xa6, 0x80]);
    }
    {
        fn get_name() -> String {
            std::env::var("USER").unwrap_or("whoever you are".to_string())
        }

        println!("Greetings, {}!", get_name());
    }
    {
        use std::borrow::Cow;

        // Unix系OSでないとstd::env::var()で環境変数USERが取れないため、
        // 取れたらmap()の結果のStringをCow::Ownedとして返す
        // 失敗した場合はunwrap_or()の部分でstaticな&strがCow::Borrowedとして返される
        fn get_name() -> Cow<'static, str> {
            // std::env::var("USER")
            //     .map(|v| Cow::Owned(v))
            //     .unwrap_or(Cow::Borrowed("whoever you are"))

            // Cow(Clone-on-writeによる書き込む時までクローンを遅延させる)は文字列に使われることが多いため
            // Cow<'static, str> には標準ライブラリで特別なサポートが入っている
            // Stringと&strからCow<'static, str>へはFromとIntoによる変換があるため、下のコードは上と同じになる
            std::env::var("USER")
                .map(|v| v.into())
                .unwrap_or("whoever you are".into())
        }

        println!("Greetings, {}!", get_name());
    }
    {
        // Rustのformatパラメータは{ which : how }の形式で指定する
        //  whichの部分はindexや名前が指定でき、指定しなければ引数の左から順に採用される
        //    howの部分は引数をフォーマットする方法を示す
        println!("{:.3}us: relocated {} at {:#x} to {:#x}, {} bytes",
                 0.84391, "object",
                 140737488346304_usize, 6299664_usize, 64);

        assert_eq!(format!("number of {}: {}", "elephants", 19),
                  "number of elephants: 19");

        // { which : how }のwhich部を指定しているため順番が入れ替わってフォーマットされる
        assert_eq!(format!("from {1} to {0}", "the grave", "the cradle"),
                  "from the cradle to the grave");

        // { which : how }のhow部を指定してデバッグ出力フォーマット
        assert_eq!(format!("v = {:?}", vec![0,1,2,5,12,29]),
                   "v = [0, 1, 2, 5, 12, 29]");

        assert_eq!(format!("name = {:?}", "Nemo"),
                   "name = \"Nemo\"");

        // 8桁小数点2桁
        assert_eq!(format!("{:8.2} km/s", 11.186),
                   "   11.19 km/s");

        // 20桁出力、2進数、2進数
        assert_eq!(format!("{:20} {:02x} {:02x}", "abc #42", 105, 42),
                   "abc #42              69 2a");

        assert_eq!(format!("{1:02x} {2:02x} {0}", "abc #42", 105, 42),
                   "69 2a abc #42");

        assert_eq!(format!("{lsb:02x} {msb:02x} {insn}", insn="abc #42", lsb=105,msb=42),
                   "69 2a abc #42");

        // 出力に'{'や'}'を使いたい場合は、テンプレート文字列中では{{や}}のように二重で書けばよい
        assert_eq!(format!("{{a, c}} ⊂ {{a, b, c}}"),
                   "{a, c} ⊂ {a, b, c}");
    }
    {
        // formatパラメータ{ : how }部分の様々な指定
        let input = "bookends";

        // デフォルト
        assert_eq!(format!("{}", input),
                   "bookends");
        // 最短フィールド幅
        assert_eq!(format!("{:10}", input),
                   "bookends  ");

        // 最長テキスト長
        assert_eq!(format!("{:.4}", input),
                   "book");

        // フィールド幅、テキスト長
        assert_eq!(format!("{:12.20}", input), // フィールド最短12 テキスト最長20
                   "bookends    ");
        assert_eq!(format!("{:4.20}", input),  // フィールド最短 4 テキスト最長20
                   "bookends");
        assert_eq!(format!("{:4.6}", input),   // フィールド最短 4 テキスト最長6
                   "booken");
        assert_eq!(format!("{:6.4}", input),   // フィールド最短 6 テキスト最長4
                   "book  ");

        // 左寄せ、フィールド幅
        assert_eq!(format!("{:<12}", input),
                   "bookends    ");
        // 中央揃え、フィールド幅
        assert_eq!(format!("{:^12}", input),
                   "  bookends  ");
        // 右寄せ、フィールド幅
        assert_eq!(format!("{:>12}", input),
                   "    bookends");
        // '='でパディング、中央揃え、フィールド幅
        assert_eq!(format!("{:=^12}", input),
                   "==bookends==");
        // '*'でパディング、右寄せ、フィールド幅、テキスト最長4
        assert_eq!(format!("{:*>12.4}", input),
                   "********book");

        // RustのテキストフォーマットはUnicodeの合字は無視して素朴に扱う
        assert_eq!(format!("{:4}", "th\u{e9}"),
                   "th\u{e9} ");
    }
}
