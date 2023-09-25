use std::str::FromStr;

#[derive(Debug)]
struct URL {
    protocol: Protocol,
    address: String,
    port: u16,
}

impl FromStr for URL {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // "http://127.0.0.1:8686";
        let cut1 = s.split("://").collect::<Vec<&str>>();
        let cut2 = cut1[1].split(":").collect::<Vec<&str>>();

        Ok(URL {
            protocol: Protocol::from_str(cut1[0]).unwrap(),
            address: cut2[0].to_string(),
            port: cut2[1].parse::<u16>().unwrap(),
        })
    }
}

impl From<URL> for String {
    fn from(value: URL) -> Self {
        format!("{}://{}:{}", value.protocol.to_string(), value.address, value.port)
    }
}

impl From<String> for URL{
    fn from(value: String) -> Self {
       URL::try_from(value).unwrap()
    }
}


#[derive(Debug)]
enum Protocol {
    Http,
    Https,
}

impl FromStr for Protocol {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "http" => Ok(Protocol::Http),
            "https" => Ok(Protocol::Https),
            _ => Err("Invalid Convert".to_string())
        }
    }
}

impl ToString for Protocol {
    fn to_string(&self) -> String {
        match self {
            Protocol::Http => String::from("http"),
            Protocol::Https => String::from("https")
        }
    }
}

pub fn deal<T>(_url: T) -> ()
    where T: Into<String>,
{
    println!("into string")
}

pub fn deal2<U>(_s: U) -> ()
    where U: Into<URL>,
{
    println!("into URL")
}

impl TryFrom<String> for URL {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        URL::from_str(value.as_str())
    }
}


fn main() {

    let res = URL::from_str("http://localhost:8080").unwrap();
    deal(res);
    deal2("https://localhost:8080".to_string());
}