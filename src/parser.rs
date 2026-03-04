use crate::types::{HttpMethod, HttpRequest, HttpRequestLine};

pub fn parse_bytes(bytes: Vec<u8>) -> Result<HttpRequest, String> {
    const SEPARATOR: &[u8; 4] = &[0x0D, 0x0A, 0x0D, 0x0A];
    let mut request = HttpRequest::default();

    if bytes.len() < 4 {
        return Err("Request length small".to_string());
    }

    let mut separator_index = 0;
    let mut windows = bytes.windows(4);
    while let Some(window) = windows.next() {
        if window == SEPARATOR {
            break;
        }
        separator_index += 1;
    }

    let header_lines: Vec<&[u8]> = (&bytes[..separator_index])
        .split(|c| *c == 0x0D)
        .map(|l| if l[0] == 0x0A { &l[1..] } else { &l })
        .collect();
    let body = &bytes[separator_index + 4..];

    // Process request line
    {
        let request_line = str::from_utf8(header_lines[0]).unwrap();
        let parts: Vec<&str> = request_line.split(|c| c == ' ').collect();

        request.request_line = HttpRequestLine {
            method: HttpMethod::from_string(parts[0]),
            path: parts[1].to_string(),
            version: parts[2].to_string(),
        }
    }

    for line in &header_lines[1..] {
        let kv = {
            let kv = line.split(|x| *x == 0x3A).collect::<Vec<_>>();
            (
                str::from_utf8(kv[0]).unwrap(),
                str::from_utf8(&kv[1][1..]).unwrap().to_string(),
            )
        };

        if kv.0 == "Host" {
            request.host = kv.1;
        } else {
            request.headers.insert(kv.0.to_string(), kv.1);
        }
    }

    request.body = str::from_utf8(body).unwrap().to_string();

    Ok(request)
}
