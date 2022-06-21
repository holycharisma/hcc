running the project:

in root dir, direnv allow  to create some local secrets

you will need a `.secrets/.env` file that looks something like:

- todo - automate some tools for generating the emoji encoded security values

```
export HCC_ORIGIN_DOMAIN="127.0.0.7777"
export HCC_SESSION_COOKIE_NAME=hcc.sid
export HCC_SESSION_TTL_HOURS=8
export HCC_POSTGRES_SQL_CONNECTION_URL="postgresql://root:root@127.0.0.1:5432/hcc?sslmode=disable"
export HCC_BIND_URL=127.0.0.1:7777
export HCC_SUPER_USER_EMAIL=dreamnet@holycharisma.com
export HCC_RSA_PRIVATE_KEY_PATH=.secrets/jwtRS256.key
export HCC_RSA_PUBLIC_KEY_PATH=.secrets/jwtRS256.key.pub
export HCC_SUPER_USER_PWHASH_EMOJI=?????
export HCC_ENCRYPTION_KEY_EMOJI=????
export HCC_ENCRYPTION_SALT_EMOJI=????
```

building order

- frontend 

```
cd hcc-client
npm install
npm run build
cd -
    
```

- server

```

cd hcc-server
cargo build --release
cd -
    
```


- database


```
cd hcc-db
cat README.md
# run some tasks listed in the README.md
sea migrate
cd -
    
```

finally, from root dir

```
hcc-server/target/release/hcc-server
```

