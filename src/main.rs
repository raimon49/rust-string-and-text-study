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

        assert_eq!("うどんudon".as_bytes(),
                   &[0xe3, 0x81, 0x86, // う
                     0xe3, 0x81, 0xa9, // ど
                     0xe3, 0x82, 0x93, // ん
                     0x75, 0x64, 0x6f, 0x6e //:udon
                   ]);
    }
}
