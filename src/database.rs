use redis::Commands;

pub struct Database {
    conn: Option<redis::Connection>,
    client: redis::Client,
}

impl Database {
    pub fn new() -> redis::RedisResult<Self> {
        let client = redis::Client::open("redis://127.0.0.1/")?;
        let conn = client.get_connection()?;

        Ok(Self {
            client,
            conn: Some(conn),
        })
    }

    pub fn get_connection(&mut self) -> redis::RedisResult<&mut redis::Connection> {
        if self.conn.is_none() {
            self.conn = Some(self.client.get_connection().unwrap());
        }
        Ok(self.conn.as_mut().unwrap())
    }

    pub fn get_cells(&mut self, sheet_id: &str) -> redis::RedisResult<()> {
        let mut conn = self.get_connection().unwrap();
        let result: Vec<(usize, usize)> = conn.hgetall(sheet_id)?;
        println!("Cells: {:?}", result);

        Ok(())
    }

    pub fn write_all_cells(
        &mut self,
        sheet_id: &str,
        cells: Vec<(String, String)>,
    ) -> redis::RedisResult<()> {
        let mut conn = self.get_connection().unwrap();

        let mut pipe = redis::Pipeline::new();
        for (key, value) in cells {
            pipe.hset(sheet_id, key, value);
        }
        pipe.exec(&mut conn)?;

        Ok(())
    }
}
