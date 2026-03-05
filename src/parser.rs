use crate::types::{HttpBody, HttpHeader, HttpMethod};

pub fn header_parser(bytes: &[u8]) -> Result<HttpHeader, String> {
    let mut header = HttpHeader::default();

    if bytes.len() < 4 {
        return Err("Request length small".to_string());
    }

    let header_lines: Vec<&[u8]> = bytes
        .split(|c| *c == 0x0D)
        .map(|l| if l[0] == 0x0A { &l[1..] } else { &l })
        .collect();

    // Process request line
    {
        let request_line = str::from_utf8(header_lines[0]).unwrap();
        let parts: Vec<&str> = request_line.split(|c| c == ' ').collect();

        header.method = HttpMethod::from_string(parts[0]);
        header.path = parts[1].to_string();
        header.version = parts[2].to_string();
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
            header.host = kv.1;
        } else {
            header.headers.insert(kv.0.to_string(), kv.1);
        }
    }

    Ok(header)
}

pub fn body_parser(bytes: &[u8]) -> Result<HttpBody, String> {
    Ok(HttpBody {
        body: str::from_utf8(bytes).unwrap().to_string(),
    })
}
