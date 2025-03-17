use std::{fmt, str::FromStr};

use serde::{Deserialize, Serialize};

/// 搜索关键词
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Keyword {
    /** 一般的搜索关键词。 */
    Normal(String),
    /** TAG 作品的语言。 */
    Language(String),
    /** TAG 同人作品模仿的原始作品。 */
    Parody(String),
    /** TAG 作品中出现的角色。 */
    Character(String),
    /** TAG 绘画作者/写手。 */
    Artist(String),
    /** TAG 角色扮演者。 */
    Cosplayer(String),
    /** TAG 制作社团或公司。 */
    Group(String),
    /** TAG 女性角色相关的恋物标签。 */
    Female(String),
    /** TAG 男性角色相关的恋物标签。 */
    Male(String),
    /** TAG 两性/中性的恋物标签。 */
    Mixed(String),
    /** TAG 经过确认的技术标签。 */
    Other(String),
    /** TAG 用于分类出错的图库，当某个重新分类标签权重达到 100，将移动图库至对应分类。 */
    Reclass(String),
    /** TAG 尚未正式添加至 E-Hentai 标签系统的标签。在提供翻译前，需要在 E-Hentai 论坛发帖将该标签移动到合适的命名空间。 */
    Temp(String),
    /** TAG? 上传者名称 */
    Uploader(String),
}

impl fmt::Display for Keyword {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Keyword::Normal(keyword) => write!(f, "\"{}\"", keyword),
            Keyword::Language(keyword) => write!(f, "l:\"{}$\"", keyword),
            Keyword::Parody(keyword) => write!(f, "p:\"{}$\"", keyword),
            Keyword::Character(keyword) => write!(f, "c:\"{}$\"", keyword),
            Keyword::Artist(keyword) => write!(f, "a:\"{}$\"", keyword),
            Keyword::Cosplayer(keyword) => write!(f, "cos:\"{}$\"", keyword),
            Keyword::Group(keyword) => write!(f, "g:\"{}$\"", keyword),
            Keyword::Female(keyword) => write!(f, "f:\"{}$\"", keyword),
            Keyword::Male(keyword) => write!(f, "m:\"{}$\"", keyword),
            Keyword::Mixed(keyword) => write!(f, "x:\"{}$\"", keyword),
            Keyword::Other(keyword) => write!(f, "o:\"{}$\"", keyword),
            Keyword::Reclass(keyword) => write!(f, "r:\"{}$\"", keyword),
            Keyword::Temp(keyword) => write!(f, "temp:\"{}$\"", keyword),
            Keyword::Uploader(keyword) => write!(f, "uploader:\"{}$\"", keyword),
        }
    }
}

impl From<Keyword> for String {
    fn from(keyword: Keyword) -> String {
        keyword.to_string()
    }
}

impl From<String> for Keyword {
    fn from(s: String) -> Keyword {
        let result = Keyword::from_str(&s);
        result.unwrap_or(Keyword::Normal(s))
    }
}

impl FromStr for Keyword {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let result: Vec<&str> = s.split(":").collect();
        match result.len() {
            1 => Ok(Keyword::Normal(result[0].into())),
            2 => match result[0] {
                "language" | "l" => Ok(Keyword::Language(result[1].into())),
                "parody" | "p" => Ok(Keyword::Parody(result[1].into())),
                "character" | "c" => Ok(Keyword::Character(result[1].into())),
                "artist" | "a" => Ok(Keyword::Artist(result[1].into())),
                "cosplayer" | "cos" => Ok(Keyword::Cosplayer(result[1].into())),
                "group" | "g" => Ok(Keyword::Group(result[1].into())),
                "female" | "f" => Ok(Keyword::Female(result[1].into())),
                "male" | "m" => Ok(Keyword::Male(result[1].into())),
                "mixed" | "x" => Ok(Keyword::Mixed(result[1].into())),
                "other" | "o" => Ok(Keyword::Other(result[1].into())),
                "reclass" | "r" => Ok(Keyword::Reclass(result[1].into())),
                "temp" => Ok(Keyword::Temp(result[1].into())),
                "uploader" => Ok(Keyword::Uploader(result[1].into())),
                _ => Err(format!("Invalid keyword: {}", s)),
            },
            _ => Err(format!("Invalid keyword: {}", s)),
        }
    }
}
