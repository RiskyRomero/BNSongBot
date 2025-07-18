# BNSongBot
A Discord bot built in Rust that allows users to interact with a song database. Users can select, add, and delete songs from a list or album, retrieve random songs, and more.

## Prerequisites
- **Rust and Cargo:** Install Rust for your respective operating system from [here](https://www.rust-lang.org/).
- **Discord bot token:** Create one at [Discord Developer Portal](https://discord.com/developers/applications).

## Usage
- Clone this project into your local machine using `git clone https://github.com/DYstebo/List-Bot.git`.
- CD into the project directory with `cd BNSongBot`.
- Rename `.env.example` to `.env`.
- Populate `.env` with your `DISCORD_TOKEN`, `GUILD_ID` and `MOD_ROLE_ID`.
- Build the application with `cargo build --release`
- Run the "BNSongBot" executable located in `target/release`

### Note
To get the `MOD_ROLE_ID`, mention it on discord by typing `\@rolename`, e.g. `\@moderator`, which should output as <@&1394872594850779178>; "1394872594850779178" is the role ID in this case.

## Commands
**/ping** - Pings the bot

**/help [command_name (optional)]** - Displays help about a command

**/new [song_title] [song_album]** - Adds a new song to the list

**/list [album (optional)]** - Displays a list of songs

**/random [album (optional)] [amount (optional)]** - Gets a single or multiple random songs

**/delete [song_id]** - Deletes a song from the list by its ID

## Contributing
1. Fork the repository.
2. Create a new branch for your feature or bug fix.
3. Make your changes.
4. Commit your changes with a descriptive message.
5. Open a pull request.