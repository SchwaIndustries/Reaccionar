extern crate redis;
use redits::Commands;
use serenity::model::id::GuildId;

mod botdb {

    struct DB_Entry {
        guild_id: u64,
        message_id: u64, // ID of the message users react to
        role_id: u64 // The ID of the role given
    };

    // Connect to the Redis DB
    pub fn db_init(address: String) -> redis::RedisResult<Connection> {
        let client = redis::Client::open(address);
        let mut con = client.get_connection()?;
        return con;
    }

    pub fn get_guild(connection: redis::RedisResult<Connection>, guild_id: u64) -> DB_Entry {
        let map : HashMap<String, u64> = connection.hgetall(guild_id.to_string())?;
        let entry = DB_Entry;
        entry.guild_id = map.get(&"guild_id");
        entry.message_id = map.get(&"message_id");
        entry.role_id = map.get(&"role_id");

        return entry;
    }

}