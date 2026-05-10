use crate::location::{LocationId, Region, RegionKind};

/// JIS X 0401 都道府県コード（1–47）
#[repr(u32)]
#[derive(Clone, Copy)]
pub(crate) enum P {
    Hokkaido = 1,
    Aomori = 2,
    Iwate = 3,
    Miyagi = 4,
    Akita = 5,
    Yamagata = 6,
    Fukushima = 7,
    Ibaraki = 8,
    Tochigi = 9,
    Gunma = 10,
    Saitama = 11,
    Chiba = 12,
    Tokyo = 13,
    Kanagawa = 14,
    Niigata = 15,
    Toyama = 16,
    Ishikawa = 17,
    Fukui = 18,
    Yamanashi = 19,
    Nagano = 20,
    Gifu = 21,
    Shizuoka = 22,
    Aichi = 23,
    Mie = 24,
    Shiga = 25,
    Kyoto = 26,
    Osaka = 27,
    Hyogo = 28,
    Nara = 29,
    Wakayama = 30,
    Tottori = 31,
    Shimane = 32,
    Okayama = 33,
    Hiroshima = 34,
    Yamaguchi = 35,
    Tokushima = 36,
    Kagawa = 37,
    Ehime = 38,
    Kochi = 39,
    Fukuoka = 40,
    Saga = 41,
    Nagasaki = 42,
    Kumamoto = 43,
    Oita = 44,
    Miyazaki = 45,
    Kagoshima = 46,
    Okinawa = 47,
}

impl P {
    pub(crate) const fn id(self) -> LocationId {
        LocationId(self as u32)
    }
}

use P::*;

pub const PREFECTURES: &[Region] = &[
    Region {
        id: Hokkaido.id(),
        kind: RegionKind::Prefecture,
        parent: None,
        name: "北海道",
        kana: "ほっかいどう",
        roman: "Hokkaido",
        neighbors: &[Aomori.id()],
    },
    Region {
        id: Aomori.id(),
        kind: RegionKind::Prefecture,
        parent: None,
        name: "青森",
        kana: "あおもり",
        roman: "Aomori",
        neighbors: &[Hokkaido.id(), Iwate.id(), Akita.id()],
    },
    Region {
        id: Iwate.id(),
        kind: RegionKind::Prefecture,
        parent: None,
        name: "岩手",
        kana: "いわて",
        roman: "Iwate",
        neighbors: &[Aomori.id(), Akita.id(), Miyagi.id()],
    },
    Region {
        id: Miyagi.id(),
        kind: RegionKind::Prefecture,
        parent: None,
        name: "宮城",
        kana: "みやぎ",
        roman: "Miyagi",
        neighbors: &[Iwate.id(), Akita.id(), Yamagata.id(), Fukushima.id()],
    },
    Region {
        id: Akita.id(),
        kind: RegionKind::Prefecture,
        parent: None,
        name: "秋田",
        kana: "あきた",
        roman: "Akita",
        neighbors: &[Aomori.id(), Iwate.id(), Miyagi.id(), Yamagata.id()],
    },
    Region {
        id: Yamagata.id(),
        kind: RegionKind::Prefecture,
        parent: None,
        name: "山形",
        kana: "やまがた",
        roman: "Yamagata",
        neighbors: &[Akita.id(), Miyagi.id(), Fukushima.id(), Niigata.id()],
    },
    Region {
        id: Fukushima.id(),
        kind: RegionKind::Prefecture,
        parent: None,
        name: "福島",
        kana: "ふくしま",
        roman: "Fukushima",
        neighbors: &[
            Yamagata.id(),
            Miyagi.id(),
            Ibaraki.id(),
            Tochigi.id(),
            Gunma.id(),
            Niigata.id(),
        ],
    },
    Region {
        id: Ibaraki.id(),
        kind: RegionKind::Prefecture,
        parent: None,
        name: "茨城",
        kana: "いばらき",
        roman: "Ibaraki",
        neighbors: &[Fukushima.id(), Tochigi.id(), Saitama.id(), Chiba.id()],
    },
    Region {
        id: Tochigi.id(),
        kind: RegionKind::Prefecture,
        parent: None,
        name: "栃木",
        kana: "とちぎ",
        roman: "Tochigi",
        neighbors: &[Fukushima.id(), Ibaraki.id(), Gunma.id(), Saitama.id()],
    },
    Region {
        id: Gunma.id(),
        kind: RegionKind::Prefecture,
        parent: None,
        name: "群馬",
        kana: "ぐんま",
        roman: "Gunma",
        neighbors: &[
            Fukushima.id(),
            Tochigi.id(),
            Saitama.id(),
            Nagano.id(),
            Niigata.id(),
        ],
    },
    Region {
        id: Saitama.id(),
        kind: RegionKind::Prefecture,
        parent: None,
        name: "埼玉",
        kana: "さいたま",
        roman: "Saitama",
        neighbors: &[
            Ibaraki.id(),
            Tochigi.id(),
            Gunma.id(),
            Chiba.id(),
            Tokyo.id(),
            Yamanashi.id(),
            Nagano.id(),
        ],
    },
    Region {
        id: Chiba.id(),
        kind: RegionKind::Prefecture,
        parent: None,
        name: "千葉",
        kana: "ちば",
        roman: "Chiba",
        neighbors: &[Ibaraki.id(), Saitama.id(), Tokyo.id(), Kanagawa.id()],
    },
    Region {
        id: Tokyo.id(),
        kind: RegionKind::Prefecture,
        parent: None,
        name: "東京",
        kana: "とうきょう",
        roman: "Tokyo",
        neighbors: &[Saitama.id(), Chiba.id(), Kanagawa.id(), Yamanashi.id()],
    },
    Region {
        id: Kanagawa.id(),
        kind: RegionKind::Prefecture,
        parent: None,
        name: "神奈川",
        kana: "かながわ",
        roman: "Kanagawa",
        neighbors: &[Chiba.id(), Tokyo.id(), Yamanashi.id(), Shizuoka.id()],
    },
    Region {
        id: Niigata.id(),
        kind: RegionKind::Prefecture,
        parent: None,
        name: "新潟",
        kana: "にいがた",
        roman: "Niigata",
        neighbors: &[
            Yamagata.id(),
            Fukushima.id(),
            Gunma.id(),
            Nagano.id(),
            Toyama.id(),
        ],
    },
    Region {
        id: Toyama.id(),
        kind: RegionKind::Prefecture,
        parent: None,
        name: "富山",
        kana: "とやま",
        roman: "Toyama",
        neighbors: &[Niigata.id(), Nagano.id(), Gifu.id(), Ishikawa.id()],
    },
    Region {
        id: Ishikawa.id(),
        kind: RegionKind::Prefecture,
        parent: None,
        name: "石川",
        kana: "いしかわ",
        roman: "Ishikawa",
        neighbors: &[Toyama.id(), Gifu.id(), Fukui.id()],
    },
    Region {
        id: Fukui.id(),
        kind: RegionKind::Prefecture,
        parent: None,
        name: "福井",
        kana: "ふくい",
        roman: "Fukui",
        neighbors: &[Ishikawa.id(), Gifu.id(), Shiga.id(), Kyoto.id()],
    },
    Region {
        id: Yamanashi.id(),
        kind: RegionKind::Prefecture,
        parent: None,
        name: "山梨",
        kana: "やまなし",
        roman: "Yamanashi",
        neighbors: &[
            Saitama.id(),
            Tokyo.id(),
            Kanagawa.id(),
            Nagano.id(),
            Shizuoka.id(),
        ],
    },
    Region {
        id: Nagano.id(),
        kind: RegionKind::Prefecture,
        parent: None,
        name: "長野",
        kana: "ながの",
        roman: "Nagano",
        neighbors: &[
            Gunma.id(),
            Saitama.id(),
            Niigata.id(),
            Toyama.id(),
            Yamanashi.id(),
            Shizuoka.id(),
            Aichi.id(),
            Gifu.id(),
        ],
    },
    Region {
        id: Gifu.id(),
        kind: RegionKind::Prefecture,
        parent: None,
        name: "岐阜",
        kana: "ぎふ",
        roman: "Gifu",
        neighbors: &[
            Toyama.id(),
            Ishikawa.id(),
            Fukui.id(),
            Nagano.id(),
            Aichi.id(),
            Mie.id(),
            Shiga.id(),
        ],
    },
    Region {
        id: Shizuoka.id(),
        kind: RegionKind::Prefecture,
        parent: None,
        name: "静岡",
        kana: "しずおか",
        roman: "Shizuoka",
        neighbors: &[Kanagawa.id(), Yamanashi.id(), Nagano.id(), Aichi.id()],
    },
    Region {
        id: Aichi.id(),
        kind: RegionKind::Prefecture,
        parent: None,
        name: "愛知",
        kana: "あいち",
        roman: "Aichi",
        neighbors: &[Nagano.id(), Gifu.id(), Shizuoka.id(), Mie.id()],
    },
    Region {
        id: Mie.id(),
        kind: RegionKind::Prefecture,
        parent: None,
        name: "三重",
        kana: "みえ",
        roman: "Mie",
        neighbors: &[
            Gifu.id(),
            Aichi.id(),
            Shiga.id(),
            Kyoto.id(),
            Nara.id(),
            Wakayama.id(),
        ],
    },
    Region {
        id: Shiga.id(),
        kind: RegionKind::Prefecture,
        parent: None,
        name: "滋賀",
        kana: "しが",
        roman: "Shiga",
        neighbors: &[Fukui.id(), Gifu.id(), Mie.id(), Kyoto.id()],
    },
    Region {
        id: Kyoto.id(),
        kind: RegionKind::Prefecture,
        parent: None,
        name: "京都",
        kana: "きょうと",
        roman: "Kyoto",
        neighbors: &[
            Fukui.id(),
            Mie.id(),
            Shiga.id(),
            Osaka.id(),
            Hyogo.id(),
            Nara.id(),
        ],
    },
    Region {
        id: Osaka.id(),
        kind: RegionKind::Prefecture,
        parent: None,
        name: "大阪",
        kana: "おおさか",
        roman: "Osaka",
        neighbors: &[Kyoto.id(), Hyogo.id(), Nara.id(), Wakayama.id()],
    },
    Region {
        id: Hyogo.id(),
        kind: RegionKind::Prefecture,
        parent: None,
        name: "兵庫",
        kana: "ひょうご",
        roman: "Hyogo",
        neighbors: &[
            Kyoto.id(),
            Osaka.id(),
            Tottori.id(),
            Okayama.id(),
            Tokushima.id(),
        ],
    },
    Region {
        id: Nara.id(),
        kind: RegionKind::Prefecture,
        parent: None,
        name: "奈良",
        kana: "なら",
        roman: "Nara",
        neighbors: &[Kyoto.id(), Osaka.id(), Mie.id(), Wakayama.id()],
    },
    Region {
        id: Wakayama.id(),
        kind: RegionKind::Prefecture,
        parent: None,
        name: "和歌山",
        kana: "わかやま",
        roman: "Wakayama",
        neighbors: &[Osaka.id(), Nara.id(), Mie.id()],
    },
    Region {
        id: Tottori.id(),
        kind: RegionKind::Prefecture,
        parent: None,
        name: "鳥取",
        kana: "とっとり",
        roman: "Tottori",
        neighbors: &[Hyogo.id(), Shimane.id(), Okayama.id(), Hiroshima.id()],
    },
    Region {
        id: Shimane.id(),
        kind: RegionKind::Prefecture,
        parent: None,
        name: "島根",
        kana: "しまね",
        roman: "Shimane",
        neighbors: &[Tottori.id(), Hiroshima.id(), Yamaguchi.id()],
    },
    Region {
        id: Okayama.id(),
        kind: RegionKind::Prefecture,
        parent: None,
        name: "岡山",
        kana: "おかやま",
        roman: "Okayama",
        neighbors: &[Tottori.id(), Hyogo.id(), Hiroshima.id(), Kagawa.id()],
    },
    Region {
        id: Hiroshima.id(),
        kind: RegionKind::Prefecture,
        parent: None,
        name: "広島",
        kana: "ひろしま",
        roman: "Hiroshima",
        neighbors: &[
            Tottori.id(),
            Shimane.id(),
            Okayama.id(),
            Yamaguchi.id(),
            Ehime.id(),
        ],
    },
    Region {
        id: Yamaguchi.id(),
        kind: RegionKind::Prefecture,
        parent: None,
        name: "山口",
        kana: "やまぐち",
        roman: "Yamaguchi",
        neighbors: &[Shimane.id(), Hiroshima.id(), Fukuoka.id()],
    },
    Region {
        id: Tokushima.id(),
        kind: RegionKind::Prefecture,
        parent: None,
        name: "徳島",
        kana: "とくしま",
        roman: "Tokushima",
        neighbors: &[Hyogo.id(), Kagawa.id(), Kochi.id(), Ehime.id()],
    },
    Region {
        id: Kagawa.id(),
        kind: RegionKind::Prefecture,
        parent: None,
        name: "香川",
        kana: "かがわ",
        roman: "Kagawa",
        neighbors: &[Tokushima.id(), Okayama.id(), Ehime.id()],
    },
    Region {
        id: Ehime.id(),
        kind: RegionKind::Prefecture,
        parent: None,
        name: "愛媛",
        kana: "えひめ",
        roman: "Ehime",
        neighbors: &[Hiroshima.id(), Tokushima.id(), Kagawa.id(), Kochi.id()],
    },
    Region {
        id: Kochi.id(),
        kind: RegionKind::Prefecture,
        parent: None,
        name: "高知",
        kana: "こうち",
        roman: "Kochi",
        neighbors: &[Tokushima.id(), Ehime.id()],
    },
    Region {
        id: Fukuoka.id(),
        kind: RegionKind::Prefecture,
        parent: None,
        name: "福岡",
        kana: "ふくおか",
        roman: "Fukuoka",
        neighbors: &[Yamaguchi.id(), Saga.id(), Oita.id(), Kumamoto.id()],
    },
    Region {
        id: Saga.id(),
        kind: RegionKind::Prefecture,
        parent: None,
        name: "佐賀",
        kana: "さが",
        roman: "Saga",
        neighbors: &[Fukuoka.id(), Nagasaki.id()],
    },
    Region {
        id: Nagasaki.id(),
        kind: RegionKind::Prefecture,
        parent: None,
        name: "長崎",
        kana: "ながさき",
        roman: "Nagasaki",
        neighbors: &[Saga.id()],
    },
    Region {
        id: Kumamoto.id(),
        kind: RegionKind::Prefecture,
        parent: None,
        name: "熊本",
        kana: "くまもと",
        roman: "Kumamoto",
        neighbors: &[Fukuoka.id(), Oita.id(), Miyazaki.id(), Kagoshima.id()],
    },
    Region {
        id: Oita.id(),
        kind: RegionKind::Prefecture,
        parent: None,
        name: "大分",
        kana: "おおいた",
        roman: "Oita",
        neighbors: &[Fukuoka.id(), Kumamoto.id(), Miyazaki.id()],
    },
    Region {
        id: Miyazaki.id(),
        kind: RegionKind::Prefecture,
        parent: None,
        name: "宮崎",
        kana: "みやざき",
        roman: "Miyazaki",
        neighbors: &[Kumamoto.id(), Oita.id(), Kagoshima.id()],
    },
    Region {
        id: Kagoshima.id(),
        kind: RegionKind::Prefecture,
        parent: None,
        name: "鹿児島",
        kana: "かごしま",
        roman: "Kagoshima",
        neighbors: &[Kumamoto.id(), Miyazaki.id()],
    },
    Region {
        id: Okinawa.id(),
        kind: RegionKind::Prefecture,
        parent: None,
        name: "沖縄",
        kana: "おきなわ",
        roman: "Okinawa",
        neighbors: &[],
    },
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn all_47_prefectures_defined() {
        assert_eq!(PREFECTURES.len(), 47);
    }

    #[test]
    fn jis_codes_are_correct() {
        assert_eq!(Hokkaido.id().0, 1);
        assert_eq!(Tokyo.id().0, 13);
        assert_eq!(Osaka.id().0, 27);
        assert_eq!(Okinawa.id().0, 47);
    }

    #[test]
    fn prefectures_have_no_parent() {
        assert!(PREFECTURES.iter().all(|r| r.parent.is_none()));
    }

    #[test]
    fn hokkaido_neighbors_include_aomori() {
        let h = PREFECTURES.iter().find(|p| p.id == Hokkaido.id()).unwrap();
        assert!(h.neighbors.contains(&Aomori.id()));
    }

    #[test]
    fn okinawa_has_no_neighbors() {
        let o = PREFECTURES.iter().find(|p| p.id == Okinawa.id()).unwrap();
        assert!(o.neighbors.is_empty());
    }

    #[test]
    fn tokyo_neighbors_exclude_osaka() {
        let t = PREFECTURES.iter().find(|p| p.id == Tokyo.id()).unwrap();
        assert!(!t.neighbors.contains(&Osaka.id()));
    }

    #[test]
    fn adjacency_is_bidirectional() {
        for region in PREFECTURES {
            for &neighbor_id in region.neighbors {
                let neighbor = PREFECTURES.iter().find(|p| p.id == neighbor_id).unwrap();
                assert!(
                    neighbor.neighbors.contains(&region.id),
                    "{} → {} は片方向",
                    region.name,
                    neighbor.name
                );
            }
        }
    }
}
