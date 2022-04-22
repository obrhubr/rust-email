# Rust Email

I created this tool to learn about the SMTP protocol.

### Usage

Create a `.env` file and enter the information (Remember to enable `Access for less secure apps` in your Google Account to allow password connections). 

Then run: `cargo run`.

```
Connected to the SMTP server!
220 smtp.gmail.com ESMTP xxx-xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx.xx - gsmtp

EHLO smtp.gmail.com
250-smtp.gmail.com at your service, [xxx.xxx.xxx.xxx]
250-SIZE 35882577
250-8BITMIME
250-AUTH LOGIN PLAIN XOAUTH2 PLAIN-CLIENTTOKEN OAUTHBEARER XOAUTH
250-ENHANCEDSTATUSCODES
250-PIPELINING
250-CHUNKING
250 SMTPUTF8

AUTH LOGIN xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx
334 xxxxxxxxxs

xxxxxxxxxxxxxxxxx
MAIL FROM:<xxxxxxx.xxxxxxxx@gmail.com>
235 2.7.0 Accepted

RCPT TO:<xxxxxxx.xxxxxxxx@gmail.com>
250 2.1.0 OK xxx-xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx.xx - gsmtp

DATA
250 2.1.5 OK xxx-xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx.xx - gsmtp

From: "XXX XXX" <xxxxxxx.xxxxxxxx@gmail.com>
To: "XXX XXX" <xxxxxxx.xxxxxxxx@gmail.com>
Subject: Test Rust Client
It's working
.
354  Go ahead xxx-xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx.xx - gsmtp

QUIT
Disconnected from the SMTP server!
```