use prequel::*;

// TODO: OOOOORRRR...... turn the idl files into mini winmd files with minimal validation beyond syntax 
// and then merge the winmd files with full validation and resolving references. Problem with that is that
// we lose the file/line number for error reporting. 
// Or we invent an attribute that provides the file/line for each thing we need to refer to in an error!!

// 1. generate a .winmd (in memory?) for each .idl without resolving references or any validation beyond 
// syntax and including attributes for file/line numbers.
// 2. merge all .winmd files (both in memory and from files) resolving all references and performing 


pub struct Database {
    connection: Connection,
    insert_struct: Statement,
}

impl Database {
    pub fn new() -> Result<Database> {
        let connection = Connection::open(":memory:")?;
        connection.execute(std::include_str!("schema.sql"))?;

        let insert_struct = connection.prepare("insert into Struct (Name, Architecture) values (?, ?)")?;

        Ok(Self{connection, insert_struct})
    }

    pub fn add_struct(&self, name: &str, architecture: i32) -> Result<()> {
        self.insert_struct.reset()?;
        self.insert_struct.bind_str_static(1, name)?;
        self.insert_struct.bind_i32(2, architecture)?;
        self.insert_struct.step()?;
        Ok(())
    }
}