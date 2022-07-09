use hyper::client::HttpConnector;
use hyper::Body;
use hyper::Client;
use hyper_proxy::Intercept;
use hyper_proxy::Proxy;
use hyper_proxy::ProxyConnector;

#[tokio::main]
async fn main() {
    let proxy = {
        let proxy_uri = "http://192.168.1.20:3128".parse().unwrap();
        let proxy = Proxy::new(Intercept::All, proxy_uri);
        let connector = hyper::client::HttpConnector::new();
        let proxy_connector = ProxyConnector::from_proxy_unsecured(connector, proxy);
        proxy_connector
    };

    let client: Client<ProxyConnector<HttpConnector<_>>, Body> =
        hyper::Client::builder().build(proxy);

    let req = client.get("http://github.com".parse().unwrap());
    let res = req.await;

    println!("HTTP Uri");
    println!("========");
    match res {
        Ok(body) => println!("{:?}", body),
        Err(x) => eprintln!("{:}", x.message()),
    }

    let req = client.get("https://github.com".parse().unwrap());
    let res = req.await;

    println!("\n\nHTTPS Uri (github)");
    println!("==================");
    match res {
        Ok(body) => println!("{:?}", body),
        Err(x) => eprintln!("{:}", x.message()),
    }

    let req = client.get("https://instagram.com".parse().unwrap());
    let res = req.await;

    println!("\n\nHTTPS Uri (instagram)");
    println!("=====================");
    match res {
        Ok(body) => println!("{:?}", body),
        Err(x) => eprintln!("{:}", x.message()),
    }
}

