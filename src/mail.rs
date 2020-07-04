use lettre::{ClientSecurity, ClientTlsParameters, SmtpClient, Transport};
use lettre::smtp::{authentication::{Credentials, Mechanism}, ConnectionReuseParameters};
use native_tls::{Protocol, TlsConnector};
use lettre_email::Email;
use log::{ info, error };
use std::time::Duration;

pub fn send_mail() {
    let email = Email::builder()
        // Addresses can be specified by the tuple (email, alias)
        .to(("21828604@qq.com", "betta"))
        .from("lightstrawberry@163.com")
        .subject("Hi, Hello world")
        .text("Hello world.")
        .build();

    // Open a local connection on port 25
    let creds = Credentials::new(
        "lightstrawberry@163.com".to_string(),
        "YOYGDONXCFSIPKOL".to_string(),
    );

    let smtp_host = "smtp.163.com".to_string();
    let smtp_port = "25".to_string();

    let addr = String::from(format!("{}:{}", smtp_host, smtp_port));
    // let connector = TlsConnector::new().unwrap();
    // let tls_params = ClientTlsParameters::new(smtp_host, connector);
    // let security = ClientSecurity::Required(tls_params);
    info!("kkkkkk");
    let mut mailer = match SmtpClient::new_simple(&smtp_host) {
    // let mut mailer = match SmtpClient::new(addr, security) {
        Ok(v) => v.credentials(creds).timeout(Some(Duration::new(1, 0))).transport(),
        Err(e) => {
            error!("Connecting to SMTP server was error: {}", e);
            return;
        }
    };

    info!("start send email");
    match mailer.send(email.unwrap().into()) {
        Ok(_) => info!("Sending email success!"),
        Err(e) => error!("Sending email was error: {}", e),
    }
}

