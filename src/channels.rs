use serde::Serialize;

/// Authentication posture of a channel.
#[derive(Debug, Clone, Copy, Serialize, PartialEq, Eq)]
pub enum AuthKind {
    /// No auth required (zero-config channel).
    None,
    /// Needs a cookie / token set provided by the user.
    Cookie,
    /// Requires an interactive login (often in a browser).
    Login,
    /// Requires an API key.
    ApiKey,
}

/// Geographic region a channel primarily serves; used for proxy hints.
#[derive(Debug, Clone, Copy, Serialize, PartialEq, Eq)]
pub enum Region {
    Global,
    Cn,
}

/// A platform channel: the unit agent-reach routes and health-checks.
///
/// This is the Rust port of the original `agent_reach/channels/*.py` modules,
/// consolidated into one registry. The three new CN platforms (zhihu / taobao /
/// jd) are appended at the end with documented adaptation constraints.
#[derive(Debug, Clone, Serialize)]
pub struct Channel {
    pub id: &'static str,
    pub display: &'static str,
    pub region: &'static str,
    /// First (preferred) backend candidate.
    pub preferred_backend: &'static str,
    /// Upstream tool(s) invoked directly (not wrapped).
    pub upstream_tool: &'static str,
    pub auth: AuthKind,
    /// True when usable without any login.
    pub zero_config: bool,
    /// True when CN network egress needs a proxy.
    pub requires_proxy_cn: bool,
    pub notes: &'static str,
}

pub fn all() -> Vec<Channel> {
    vec![
        Channel {
            id: "web",
            display: "Web",
            region: "global",
            preferred_backend: "jina",
            upstream_tool: "curl+jina",
            auth: AuthKind::None,
            zero_config: true,
            requires_proxy_cn: false,
            notes: "Read any URL via r.jina.ai",
        },
        Channel {
            id: "youtube",
            display: "YouTube",
            region: "global",
            preferred_backend: "yt-dlp",
            upstream_tool: "yt-dlp",
            auth: AuthKind::None,
            zero_config: true,
            requires_proxy_cn: true,
            notes: "Video metadata / subtitles",
        },
        Channel {
            id: "github",
            display: "GitHub",
            region: "global",
            preferred_backend: "gh",
            upstream_tool: "gh",
            auth: AuthKind::None,
            zero_config: true,
            requires_proxy_cn: false,
            notes: "Repo / code search",
        },
        Channel {
            id: "rss",
            display: "RSS",
            region: "global",
            preferred_backend: "feedparser",
            upstream_tool: "feedparser",
            auth: AuthKind::None,
            zero_config: true,
            requires_proxy_cn: false,
            notes: "Feed parsing",
        },
        Channel {
            id: "exa",
            display: "Exa Search",
            region: "global",
            preferred_backend: "mcporter",
            upstream_tool: "mcporter",
            auth: AuthKind::ApiKey,
            zero_config: false,
            requires_proxy_cn: false,
            notes: "Web search API",
        },
        Channel {
            id: "twitter",
            display: "X / Twitter",
            region: "global",
            preferred_backend: "twitter-cli",
            upstream_tool: "twitter-cli",
            auth: AuthKind::Cookie,
            zero_config: false,
            requires_proxy_cn: true,
            notes: "Needs AUTH_TOKEN / CT0",
        },
        Channel {
            id: "reddit",
            display: "Reddit",
            region: "global",
            preferred_backend: "opencli",
            upstream_tool: "opencli",
            auth: AuthKind::Login,
            zero_config: false,
            requires_proxy_cn: true,
            notes: "Login mandatory",
        },
        Channel {
            id: "bilibili",
            display: "Bilibili",
            region: "cn",
            preferred_backend: "bili",
            upstream_tool: "bili",
            auth: AuthKind::None,
            zero_config: true,
            requires_proxy_cn: false,
            notes: "Basic + opencli subtitles",
        },
        Channel {
            id: "xiaohongshu",
            display: "小红书",
            region: "cn",
            preferred_backend: "opencli",
            upstream_tool: "opencli",
            auth: AuthKind::Cookie,
            zero_config: false,
            requires_proxy_cn: false,
            notes: "OpenCLI or mcporter",
        },
        Channel {
            id: "xueqiu",
            display: "雪球",
            region: "cn",
            preferred_backend: "jina",
            upstream_tool: "jina+cookie",
            auth: AuthKind::Cookie,
            zero_config: false,
            requires_proxy_cn: false,
            notes: "Stock quotes + posts",
        },
        Channel {
            id: "xiaoyuzhou",
            display: "小宇宙",
            region: "cn",
            preferred_backend: "transcribe",
            upstream_tool: "transcribe.sh",
            auth: AuthKind::ApiKey,
            zero_config: false,
            requires_proxy_cn: false,
            notes: "Groq Whisper transcription",
        },
        Channel {
            id: "linkedin",
            display: "LinkedIn",
            region: "global",
            preferred_backend: "mcporter",
            upstream_tool: "mcp-server-linkedin",
            auth: AuthKind::Login,
            zero_config: false,
            requires_proxy_cn: true,
            notes: "Profile search",
        },
        Channel {
            id: "facebook",
            display: "Facebook",
            region: "global",
            preferred_backend: "opencli",
            upstream_tool: "opencli",
            auth: AuthKind::Login,
            zero_config: false,
            requires_proxy_cn: true,
            notes: "Desktop only",
        },
        Channel {
            id: "instagram",
            display: "Instagram",
            region: "global",
            preferred_backend: "opencli",
            upstream_tool: "opencli",
            auth: AuthKind::Login,
            zero_config: false,
            requires_proxy_cn: true,
            notes: "Desktop only",
        },
        Channel {
            id: "boss",
            display: "Boss直聘",
            region: "cn",
            preferred_backend: "boss-agent-cli",
            upstream_tool: "boss-agent-cli",
            auth: AuthKind::Login,
            zero_config: false,
            requires_proxy_cn: false,
            notes: "Strict CDP, desktop only",
        },
        // ===== New CN platforms integrated in this rewrite =====
        Channel {
            id: "zhihu",
            display: "知乎",
            region: "cn",
            preferred_backend: "jina",
            upstream_tool: "jina / zhihu-cli",
            auth: AuthKind::Cookie,
            zero_config: false,
            requires_proxy_cn: false,
            notes: "Q&A + columns; login boosts rate quota",
        },
        Channel {
            id: "taobao",
            display: "淘宝",
            region: "cn",
            preferred_backend: "openapi",
            upstream_tool: "Taobao Open Platform / headless+cookie",
            auth: AuthKind::ApiKey,
            zero_config: false,
            requires_proxy_cn: false,
            notes: "Merchant Open API or headless browser + cookie",
        },
        Channel {
            id: "jd",
            display: "京东",
            region: "cn",
            preferred_backend: "jingdong",
            upstream_tool: "JD Union / Jingdong API / headless+cookie",
            auth: AuthKind::ApiKey,
            zero_config: false,
            requires_proxy_cn: false,
            notes: "JD Union affiliate API or headless browser + cookie",
        },
    ]
}
