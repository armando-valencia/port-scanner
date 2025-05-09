pub fn service_name(port: u16) -> &'static str {
    match port {
        21   => "FTP",
        22   => "SSH",
        23   => "TELNET",
        25   => "SMTP",
        53   => "DNS",
        80   => "HTTP",
        110  => "POP3",
        143  => "IMAP",
        443  => "HTTPS",
        445  => "SMB",
        3306 => "MySQL",
        5432 => "PostgreSQL",
        6379 => "Redis",
        27017=> "MongoDB",
        _    => "unknown",
    }
}