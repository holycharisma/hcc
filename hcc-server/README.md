# hcc-server

source repository for holycharisma server


## building

build binaries managed by cargo + rustup 

```
- rustup install stable
- cargo [check|run]
```

you will need a `.env` file that looks something like this:


```
export HCC_ORIGIN_DOMAIN="http://127.0.0.1:7777"
export HCC_SESSION_COOKIE_NAME=sid
export HCC_SESSION_TTL_HOURS=8
export HCC_SESSION_SECRET=1234567890abcdef1234567890abcdef
export HCC_JWT_RSA_PRIVATE_KEY_PATH=/.ssh/my-special-keys/my-RS256-identity.key
export HCC_JWT_RSA_PUBLIC_KEY_PATH=/.ssh/my-special-keys/my-RS256-identity.key.pub
export HCC_POSTGRES_SQL_CONNECTION_URL="postgresql://root@localhost:26257/defaultdb?sslmode=disable"
export HCC_BIND_URL=127.0.0.1:7777
export HCC_SUPER_USER_EMAIL=hello@example.com
export HCC_SUPER_USER_PASSWORD=hunter23@plaintext.lol
```

you will also need a postgres-like database running at the connection url

## server tech stack

- [rust](https://www.rust-lang.org/)
    - safe, efficient, & fun programming lang
- [tide](https://docs.rs/tide/0.17.0-beta.1/tide/)
    - request/response library - server routing & middleware
    - https://http-rs.github.io/tide-book/
    - [async_session](https://docs.rs/async-session/latest/async_session/)
        - db session cookies that lets you associate arbitrary json with http sessions
- [async-std](https://book.async.rs/)
    - futures and async/await style extensions to rust stdlib
- [askama](https://djc.github.io/askama/)
    - server side templating via rust compile time macros
    - jinja2 style but strong template types
- [sea-orm](https://www.sea-ql.org/SeaORM/)
    - active-record-like object relation mapper

yet to be added:

- [lettre](https://github.com/lettre/lettre)
    - smtp mailer - old fashioned E-MAIL 
    
## server goals

- host a list of email verified members
- identity and authorization
- provide chat and cozyweb space for authorized members
    - create a space where people can link up (secure direct messaging)
- host and distribute media
    - sometimes behind an authorization gate
    - sometimes live broadcasts
- direct patronage & community governance
    - banking // payment integration software
    - software managed community trust