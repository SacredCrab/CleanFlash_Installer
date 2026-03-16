pub const FLASH_VERSION: &str = "34.0.0.330";
pub const VERSION: &str = "34.0.0.330";

const API_HOST: &str = "api.github.com";
const API_PATH: &str = "/repos/cleanflash/installer/releases/latest";
const USER_AGENT: &str = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/136.0.0.0 Safari/537.36";

pub struct VersionInfo {
    pub name: String,
    pub version: String,
    pub url: String,
}

pub fn get_latest_version() -> Option<VersionInfo> {
    let json = fetch_https(API_HOST, API_PATH, USER_AGENT)?;
    let name = extract_json_string(&json, "name")?;
    let tag = extract_json_string(&json, "tag_name")?;
    let url = extract_json_string(&json, "html_url")?;

    // Validate the URL to guard against a malicious/unexpected response.
    if !url.starts_with("https://") {
        return None;
    }

    println!("Latest release: {} ({}) {}", name, tag, url);
    Some(VersionInfo { name, version: tag, url })
}

/// Perform an HTTPS GET request using WinHTTP and return the response body as UTF-8.
fn fetch_https(host: &str, path: &str, user_agent: &str) -> Option<String> {
    use windows_sys::Win32::Networking::WinHttp::{
        WinHttpCloseHandle, WinHttpConnect, WinHttpOpen, WinHttpOpenRequest,
        WinHttpQueryDataAvailable, WinHttpReadData, WinHttpReceiveResponse,
        WinHttpSendRequest, WINHTTP_FLAG_SECURE,
    };

    let wide = |s: &str| -> Vec<u16> { s.encode_utf16().chain(std::iter::once(0)).collect() };

    let agent_w = wide(user_agent);
    let host_w = wide(host);
    let path_w = wide(path);

    unsafe {
        // Open session with system default proxy settings (WINHTTP_ACCESS_TYPE_DEFAULT_PROXY = 0).
        let session = WinHttpOpen(agent_w.as_ptr(), 0, std::ptr::null(), std::ptr::null(), 0);
        if session.is_null() {
            return None;
        }

        let connection = WinHttpConnect(session, host_w.as_ptr(), 443, 0);
        if connection.is_null() {
            WinHttpCloseHandle(session);
            return None;
        }

        // Open a GET request over HTTPS (null verb = GET, null version = HTTP/1.1).
        let request = WinHttpOpenRequest(
            connection,
            std::ptr::null(),
            path_w.as_ptr(),
            std::ptr::null(),
            std::ptr::null(),
            std::ptr::null(),
            WINHTTP_FLAG_SECURE,
        );
        if request.is_null() {
            WinHttpCloseHandle(connection);
            WinHttpCloseHandle(session);
            return None;
        }

        if WinHttpSendRequest(request, std::ptr::null(), 0, std::ptr::null(), 0, 0, 0) == 0 {
            WinHttpCloseHandle(request);
            WinHttpCloseHandle(connection);
            WinHttpCloseHandle(session);
            return None;
        }

        if WinHttpReceiveResponse(request, std::ptr::null_mut()) == 0 {
            WinHttpCloseHandle(request);
            WinHttpCloseHandle(connection);
            WinHttpCloseHandle(session);
            return None;
        }

        let mut response: Vec<u8> = Vec::new();
        loop {
            let mut available: u32 = 0;
            if WinHttpQueryDataAvailable(request, &mut available) == 0 || available == 0 {
                break;
            }
            let offset = response.len();
            response.resize(offset + available as usize, 0);
            let mut read: u32 = 0;
            if WinHttpReadData(
                request,
                response[offset..].as_mut_ptr() as *mut _,
                available,
                &mut read,
            ) == 0
            {
                break;
            }
            response.truncate(offset + read as usize);
        }

        WinHttpCloseHandle(request);
        WinHttpCloseHandle(connection);
        WinHttpCloseHandle(session);

        String::from_utf8(response).ok()
    }
}

/// Extract a JSON string value for the given key from a JSON object.
/// Handles the escape sequences that appear in GitHub API responses.
fn extract_json_string(json: &str, key: &str) -> Option<String> {
    let search = format!("\"{}\"", key);
    let key_pos = json.find(&search)?;
    let rest = &json[key_pos + search.len()..];
    let colon = rest.find(':')?;
    let rest = rest[colon + 1..].trim_start();
    if !rest.starts_with('"') {
        return None;
    }
    let rest = &rest[1..];
    let mut result = String::new();
    let mut chars = rest.chars();
    loop {
        match chars.next()? {
            '"' => break,
            '\\' => match chars.next()? {
                '"' => result.push('"'),
                '\\' => result.push('\\'),
                '/' => result.push('/'),
                'n' => result.push('\n'),
                'r' => result.push('\r'),
                't' => result.push('\t'),
                c => {
                    result.push('\\');
                    result.push(c);
                }
            },
            c => result.push(c),
        }
    }
    Some(result)
}

