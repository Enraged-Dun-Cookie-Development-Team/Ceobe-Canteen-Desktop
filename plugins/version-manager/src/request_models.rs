use semver::{Op, Version};
use serde::ser::SerializeStruct;
use serde::{Deserialize, Serialize, Serializer};
use url::Url;

#[cfg(debug_assertions)]
const BASE_URL: &'static str = "http://cdn.example.com";

#[cfg(not(debug_assertions))]
// TODO: 使用正式版接口
const BASE_URL: &'static str = "http://cdn.example.com";

pub fn fetch_url(version: Option<Version>) -> Url {
    let mut base_url: Url = BASE_URL.parse().expect("Failure Conv Url");
    base_url.set_path("/cdn/operate/version/fetch");
    let query =
        serde_qs::to_string(&FetchRequestPayload { version }).expect("Failure to Genreate Qs");
    base_url.set_query(query.as_str().into());
    base_url
}

struct FetchRequestPayload {
    pub version: Option<Version>,
}

impl Serialize for FetchRequestPayload {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut st = serializer.serialize_struct("Payload", 2)?;
        st.serialize_field("version", &self.version)?;
        st.serialize_field("platform", "desktop")?;
        st.end()
    }
}

#[derive(Serialize, Deserialize)]
pub struct Response<T> {
    pub code: String,

    pub data: T,

    pub message: String,
}

/// ReleaseVersion
#[derive(Serialize, Deserialize)]
pub struct ReleaseVersion {
    /// 是否弃用，默认情况下，该参数不需提供
    deleted: bool,

    /// 发布版本描述，如果未提供，该字段获得时会跳过序列化
    description: Option<String>,

    /// 可用下载源
    download_source: Vec<DownloadSource>,

    /// 发布目标平台，- Desktop: 桌面端
    /// - Pocket: 移动端
    /// - Plugin： 插件端
    platform: Empty,

    /// 前一个强制版本
    previous_mandatory_version: String,

    /// 发布的新版本
    version: String,
}

#[derive(Serialize, Deserialize)]
pub struct DownloadSource {
    /// 下载源描述
    description: Option<String>,

    /// 下载源名称
    name: String,

    /// 主要下载链接
    primary_url: PrimaryUrl,

    /// 备用下载链接，如果没有任何备用下载链接，这个字段将不会被序列化
    spare_urls: Option<Vec<SpareUrl>>,
}

/// 主要下载链接
#[derive(Serialize, Deserialize)]
pub struct PrimaryUrl {
    /// 是否为手动下载链接
    manual: bool,

    /// 在Primary中的URL的name为可选填写，如果需要填写需要为“Primary"”
    name: Option<Name>,

    /// 支持的平台
    support_platforms: Option<Vec<SupportPlatform>>,

    /// 下载链接
    url: String,
}

#[derive(Serialize, Deserialize)]
pub enum Name {
    Primary,
}

/// Support Platform
#[derive(Serialize, Deserialize)]
pub enum SupportPlatform {
    Android,

    #[serde(rename = "BrowserZIP")]
    BrowserZip,

    Chrome,

    Edge,

    Firefox,

    Harmony,

    #[serde(rename = "IE")]
    Ie,

    Ios,

    Linux,

    #[serde(rename = "MacOS")]
    MacOs,

    Safari,

    Webkit,

    #[serde(rename = "WindowsPhone")]
    WindowsPhone,
}

#[derive(Serialize, Deserialize)]
pub struct SpareUrl {
    /// 是否为手动下载链接
    manual: bool,

    /// 备用下载名，备用下载链接名为必填
    name: String,

    /// 支持的平台
    support_platforms: Option<Vec<SupportPlatform>>,

    /// 下载链接
    url: String,
}

/// 发布目标平台，- Desktop: 桌面端
/// - Pocket: 移动端
/// - Plugin： 插件端
///
/// 发布版本平台
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Empty {
    Desktop,

    Plugin,

    Pocket,
}
