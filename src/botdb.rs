extern crate redis;
use redis::Commands;
use serenity::model::id::GuildId;
use std::collections::HashMap;

mod botdb {

    struct DB_Entry {
        guild_id: u64,
        message_id: u64, // ID of the message users react to
        role_id: u64 // The ID of the role given
    }

    // Connect to the Redis DB
    pub fn db_init(address: String) -> redis::RedisResult<Connection> {
        let client = redis::Client::open(address);
        let mut con = client.get_connection()?;
        return con;
    }

    pub fn get_guild(connection: redis::RedisResult<Connection>, guild_id: u64) -> DB_Entry {
        let map : HashMap<String, u64> = connection.hgetall(guild_id.to_string())?;
        let mut entry = DB_Entry;
        entry.guild_id = map.get(&"guild_id");
        entry.message_id = map.get(&"message_id");
        entry.role_id = map.get(&"role_id");

        return entry;
    }

    pub fn set_guild(connection: redis::RedisResult<Connection>, db_entry: DB_Entry) {
        let mut map = HashMap::new();

        map.insert("guild_id", entry.guild_id);
        map.insert("message_id", entry.message_id);
        map.insert("role_id", entry.role_id);

        connection.set(map);
    }

}