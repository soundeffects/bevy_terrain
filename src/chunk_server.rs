use rocksdb::DB;

pub struct ChunkServer {
    database: DB,
    generator: u8,
}
