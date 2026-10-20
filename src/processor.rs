use regex::Regex;
use std::sync::LazyLock;

static EMAIL_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"[\w\.-]+@[\w\.-]+\.\w+").expect("valid email regex"));

static SECRET_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"(?i)(bearer\s+|token[=:\s]+|password[=:\s]+|api_key[=:\s]+)[a-zA-Z0-9_\-\.]+")
        .expect("valid secret regex")
});

static TIMESTAMP_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"\d{4}-\d{2}-\d{2}[T\s]\d{2}:\d{2}:\d{2}(\.\d+)?(Z|[+-]\d{2}:\d{2})?")
        .expect("valid timestamp regex")
});

static UUID_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"\b[0-9a-fA-F]{8}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{12}\b")
        .expect("valid uuid regex")
});

static IP_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"\b\d{1,3}\.\d{1,3}\.\d{1,3}\.\d{1,3}(:\d+)?\b").expect("valid IP regex")
});

static HEX_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"\b0x[0-9a-fA-F]+\b").expect("valid hex regex"));

static ID_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"\b([a-zA-Z]+[_-]\d+|\d+)\b").expect("valid id regex"));

pub fn masking(log_line: &str) -> String {
    // 1. Mask email addresses
    let email_masked = EMAIL_RE.replace_all(log_line, "[EMAIL_ADDRESS]");

    // 2. Mask secrets (bearer tokens, passwords, api_keys)
    let secret_masked = SECRET_RE.replace_all(&email_masked, "$1[SECRET]");

    // 3. Mask timestamps
    let timestamp_masked = TIMESTAMP_RE.replace_all(&secret_masked, "[TIMESTAMP]");

    // 4. Mask UUIDs
    let uuid_masked = UUID_RE.replace_all(&timestamp_masked, "[UUID]");

    // 5. Mask IPs
    let ip_masked = IP_RE.replace_all(&uuid_masked, "[IP_ADDRESS]");

    // 6. Mask Hex values
    let hex_masked = HEX_RE.replace_all(&ip_masked, "[HEX]");

    // 7. Mask IDs
    let id_masked = ID_RE.replace_all(&hex_masked, "[ID]");

    let result = id_masked.to_string();
    print!("[MASKED] {}", result);

    result
}
