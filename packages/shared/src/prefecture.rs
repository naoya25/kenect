use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PrefectureId {
    Hokkaido,
    Aomori,
    Iwate,
    Miyagi,
    Akita,
    Yamagata,
    Fukushima,
    Ibaraki,
    Tochigi,
    Gunma,
    Saitama,
    Chiba,
    Tokyo,
    Kanagawa,
    Niigata,
    Toyama,
    Ishikawa,
    Fukui,
    Yamanashi,
    Nagano,
    Gifu,
    Shizuoka,
    Aichi,
    Mie,
    Shiga,
    Kyoto,
    Osaka,
    Hyogo,
    Nara,
    Wakayama,
    Tottori,
    Shimane,
    Okayama,
    Hiroshima,
    Yamaguchi,
    Tokushima,
    Kagawa,
    Ehime,
    Kochi,
    Fukuoka,
    Saga,
    Nagasaki,
    Kumamoto,
    Oita,
    Miyazaki,
    Kagoshima,
    Okinawa,
}

#[derive(Debug)]
pub struct Prefecture {
    pub id: PrefectureId,
    pub name: &'static str,
    pub kana: &'static str,
    pub roman: &'static str,
    pub neighbors: &'static [PrefectureId],
}

use PrefectureId::*;

pub const PREFECTURES: &[Prefecture] = &[
    Prefecture {
        id: Hokkaido,
        name: "北海道",
        kana: "ほっかいどう",
        roman: "Hokkaido",
        neighbors: &[Aomori],
    },
    Prefecture {
        id: Aomori,
        name: "青森",
        kana: "あおもり",
        roman: "Aomori",
        neighbors: &[Hokkaido, Iwate, Akita],
    },
    Prefecture {
        id: Iwate,
        name: "岩手",
        kana: "いわて",
        roman: "Iwate",
        neighbors: &[Aomori, Akita, Miyagi],
    },
    Prefecture {
        id: Miyagi,
        name: "宮城",
        kana: "みやぎ",
        roman: "Miyagi",
        neighbors: &[Iwate, Akita, Yamagata, Fukushima],
    },
    Prefecture {
        id: Akita,
        name: "秋田",
        kana: "あきた",
        roman: "Akita",
        neighbors: &[Aomori, Iwate, Miyagi, Yamagata],
    },
    Prefecture {
        id: Yamagata,
        name: "山形",
        kana: "やまがた",
        roman: "Yamagata",
        neighbors: &[Akita, Miyagi, Fukushima, Niigata],
    },
    Prefecture {
        id: Fukushima,
        name: "福島",
        kana: "ふくしま",
        roman: "Fukushima",
        neighbors: &[Yamagata, Miyagi, Ibaraki, Tochigi, Gunma, Niigata],
    },
    Prefecture {
        id: Ibaraki,
        name: "茨城",
        kana: "いばらき",
        roman: "Ibaraki",
        neighbors: &[Fukushima, Tochigi, Saitama, Chiba],
    },
    Prefecture {
        id: Tochigi,
        name: "栃木",
        kana: "とちぎ",
        roman: "Tochigi",
        neighbors: &[Fukushima, Ibaraki, Gunma, Saitama],
    },
    Prefecture {
        id: Gunma,
        name: "群馬",
        kana: "ぐんま",
        roman: "Gunma",
        neighbors: &[Fukushima, Tochigi, Saitama, Nagano, Niigata],
    },
    Prefecture {
        id: Saitama,
        name: "埼玉",
        kana: "さいたま",
        roman: "Saitama",
        neighbors: &[Ibaraki, Tochigi, Gunma, Chiba, Tokyo, Yamanashi, Nagano],
    },
    Prefecture {
        id: Chiba,
        name: "千葉",
        kana: "ちば",
        roman: "Chiba",
        neighbors: &[Ibaraki, Saitama, Tokyo, Kanagawa],
    },
    Prefecture {
        id: Tokyo,
        name: "東京",
        kana: "とうきょう",
        roman: "Tokyo",
        neighbors: &[Saitama, Chiba, Kanagawa, Yamanashi],
    },
    Prefecture {
        id: Kanagawa,
        name: "神奈川",
        kana: "かながわ",
        roman: "Kanagawa",
        neighbors: &[Chiba, Tokyo, Yamanashi, Shizuoka],
    },
    Prefecture {
        id: Niigata,
        name: "新潟",
        kana: "にいがた",
        roman: "Niigata",
        neighbors: &[Yamagata, Fukushima, Gunma, Nagano, Toyama],
    },
    Prefecture {
        id: Toyama,
        name: "富山",
        kana: "とやま",
        roman: "Toyama",
        neighbors: &[Niigata, Nagano, Gifu, Ishikawa],
    },
    Prefecture {
        id: Ishikawa,
        name: "石川",
        kana: "いしかわ",
        roman: "Ishikawa",
        neighbors: &[Toyama, Gifu, Fukui],
    },
    Prefecture {
        id: Fukui,
        name: "福井",
        kana: "ふくい",
        roman: "Fukui",
        neighbors: &[Ishikawa, Gifu, Shiga, Kyoto],
    },
    Prefecture {
        id: Yamanashi,
        name: "山梨",
        kana: "やまなし",
        roman: "Yamanashi",
        neighbors: &[Saitama, Tokyo, Kanagawa, Nagano, Shizuoka],
    },
    Prefecture {
        id: Nagano,
        name: "長野",
        kana: "ながの",
        roman: "Nagano",
        neighbors: &[
            Gunma, Saitama, Niigata, Toyama, Yamanashi, Shizuoka, Aichi, Gifu,
        ],
    },
    Prefecture {
        id: Gifu,
        name: "岐阜",
        kana: "ぎふ",
        roman: "Gifu",
        neighbors: &[Toyama, Ishikawa, Fukui, Nagano, Aichi, Mie, Shiga],
    },
    Prefecture {
        id: Shizuoka,
        name: "静岡",
        kana: "しずおか",
        roman: "Shizuoka",
        neighbors: &[Kanagawa, Yamanashi, Nagano, Aichi],
    },
    Prefecture {
        id: Aichi,
        name: "愛知",
        kana: "あいち",
        roman: "Aichi",
        neighbors: &[Nagano, Gifu, Shizuoka, Mie],
    },
    Prefecture {
        id: Mie,
        name: "三重",
        kana: "みえ",
        roman: "Mie",
        neighbors: &[Gifu, Aichi, Shiga, Kyoto, Nara, Wakayama],
    },
    Prefecture {
        id: Shiga,
        name: "滋賀",
        kana: "しが",
        roman: "Shiga",
        neighbors: &[Fukui, Gifu, Mie, Kyoto],
    },
    Prefecture {
        id: Kyoto,
        name: "京都",
        kana: "きょうと",
        roman: "Kyoto",
        neighbors: &[Fukui, Mie, Shiga, Osaka, Hyogo, Nara],
    },
    Prefecture {
        id: Osaka,
        name: "大阪",
        kana: "おおさか",
        roman: "Osaka",
        neighbors: &[Kyoto, Hyogo, Nara, Wakayama],
    },
    Prefecture {
        id: Hyogo,
        name: "兵庫",
        kana: "ひょうご",
        roman: "Hyogo",
        neighbors: &[Kyoto, Osaka, Tottori, Okayama, Tokushima],
    },
    Prefecture {
        id: Nara,
        name: "奈良",
        kana: "なら",
        roman: "Nara",
        neighbors: &[Kyoto, Osaka, Mie, Wakayama],
    },
    Prefecture {
        id: Wakayama,
        name: "和歌山",
        kana: "わかやま",
        roman: "Wakayama",
        neighbors: &[Osaka, Nara, Mie],
    },
    Prefecture {
        id: Tottori,
        name: "鳥取",
        kana: "とっとり",
        roman: "Tottori",
        neighbors: &[Hyogo, Shimane, Okayama, Hiroshima],
    },
    Prefecture {
        id: Shimane,
        name: "島根",
        kana: "しまね",
        roman: "Shimane",
        neighbors: &[Tottori, Hiroshima, Yamaguchi],
    },
    Prefecture {
        id: Okayama,
        name: "岡山",
        kana: "おかやま",
        roman: "Okayama",
        neighbors: &[Tottori, Hyogo, Hiroshima, Kagawa],
    },
    Prefecture {
        id: Hiroshima,
        name: "広島",
        kana: "ひろしま",
        roman: "Hiroshima",
        neighbors: &[Tottori, Shimane, Okayama, Yamaguchi, Ehime],
    },
    Prefecture {
        id: Yamaguchi,
        name: "山口",
        kana: "やまぐち",
        roman: "Yamaguchi",
        neighbors: &[Shimane, Hiroshima, Fukuoka],
    },
    Prefecture {
        id: Tokushima,
        name: "徳島",
        kana: "とくしま",
        roman: "Tokushima",
        neighbors: &[Hyogo, Kagawa, Kochi, Ehime],
    },
    Prefecture {
        id: Kagawa,
        name: "香川",
        kana: "かがわ",
        roman: "Kagawa",
        neighbors: &[Tokushima, Okayama, Ehime],
    },
    Prefecture {
        id: Ehime,
        name: "愛媛",
        kana: "えひめ",
        roman: "Ehime",
        neighbors: &[Hiroshima, Tokushima, Kagawa, Kochi],
    },
    Prefecture {
        id: Kochi,
        name: "高知",
        kana: "こうち",
        roman: "Kochi",
        neighbors: &[Tokushima, Ehime],
    },
    Prefecture {
        id: Fukuoka,
        name: "福岡",
        kana: "ふくおか",
        roman: "Fukuoka",
        neighbors: &[Yamaguchi, Saga, Oita, Kumamoto],
    },
    Prefecture {
        id: Saga,
        name: "佐賀",
        kana: "さが",
        roman: "Saga",
        neighbors: &[Fukuoka, Nagasaki],
    },
    Prefecture {
        id: Nagasaki,
        name: "長崎",
        kana: "ながさき",
        roman: "Nagasaki",
        neighbors: &[Saga],
    },
    Prefecture {
        id: Kumamoto,
        name: "熊本",
        kana: "くまもと",
        roman: "Kumamoto",
        neighbors: &[Fukuoka, Oita, Miyazaki, Kagoshima],
    },
    Prefecture {
        id: Oita,
        name: "大分",
        kana: "おおいた",
        roman: "Oita",
        neighbors: &[Fukuoka, Kumamoto, Miyazaki],
    },
    Prefecture {
        id: Miyazaki,
        name: "宮崎",
        kana: "みやざき",
        roman: "Miyazaki",
        neighbors: &[Kumamoto, Oita, Kagoshima],
    },
    Prefecture {
        id: Kagoshima,
        name: "鹿児島",
        kana: "かごしま",
        roman: "Kagoshima",
        neighbors: &[Kumamoto, Miyazaki],
    },
    Prefecture {
        id: Okinawa,
        name: "沖縄",
        kana: "おきなわ",
        roman: "Okinawa",
        neighbors: &[],
    },
];

pub fn get_prefecture(id: PrefectureId) -> Option<&'static Prefecture> {
    PREFECTURES.get(id as usize)
}

pub fn get_neighbors(id: PrefectureId) -> &'static [PrefectureId] {
    get_prefecture(id).map(|p| p.neighbors).unwrap_or(&[])
}

pub fn is_adjacent(a: PrefectureId, b: PrefectureId) -> bool {
    get_neighbors(a).contains(&b) && get_neighbors(b).contains(&a)
}

pub fn all_prefectures() -> &'static [Prefecture] {
    PREFECTURES
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hokkaido_and_aomori_are_adjacent() {
        assert!(is_adjacent(PrefectureId::Hokkaido, PrefectureId::Aomori));
        assert!(is_adjacent(PrefectureId::Aomori, PrefectureId::Hokkaido));
    }

    #[test]
    fn okinawa_has_no_neighbors() {
        assert!(get_neighbors(PrefectureId::Okinawa).is_empty());
    }

    #[test]
    fn tokyo_and_osaka_are_not_adjacent() {
        assert!(!is_adjacent(PrefectureId::Tokyo, PrefectureId::Osaka));
    }

    #[test]
    fn all_47_prefectures_defined() {
        assert_eq!(PREFECTURES.len(), 47);
    }

    #[test]
    fn adjacency_is_bidirectional() {
        for pref in PREFECTURES {
            for &neighbor_id in pref.neighbors {
                assert!(
                    is_adjacent(neighbor_id, pref.id),
                    "{:?} → {:?} は片方向になっています",
                    pref.id,
                    neighbor_id
                );
            }
        }
    }
}
