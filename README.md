# Simple pastebin server 
Made in Rust with `actix web` as the web server, `argon2` as hashing algorithm and the file system as database.

Running in a Dockercontainer

### Small 
 - No DB
 - No other external dependencies

### Dumb 
 - User credentials are stored as a publically accessible paste (user_db) in the format `{USERNAME}:{PASSWORD_HASH}`
 - UUID as file name and paste link

### How does it work?
 - Annonymous pastes get stored in the `content/public` directory as randomly named file (file name equals paste link)
 - User pastes get stored in the respective `content/users/{USERNAME}` directories, same file name convention applies
 - At startup, the server goes through the `content` directory and creates an in memory database of the available pastes, the same is done for the user database by going through the `content/public/user_db` file

 ### How to run?
 1) Rename the `.env.example` file to `.evn` and fill in the required values
 2) TODO update