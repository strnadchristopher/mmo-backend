#[derive(Debug, Copy, Clone)]
pub struct Position {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}
#[derive(Debug, Clone)]
pub struct Player{
    pub position: Position,
    pub name: String,
    pub health: i32,
    pub inventory: Vec<i32>,
    // ...
}
impl Player {
    pub fn new(name: String) -> Player {
        Player {
            position: Position { x: 0, y: 0, z: 0 },
            name,
            health: 100,
            inventory: Vec::new(),
        }
    }
    pub fn move_to(&mut self, x: i32, y: i32, z: i32) {
        self.position.x = x;
        self.position.y = y;
        self.position.z = z;
    }
    fn add_to_inventory(&mut self, item: i32) {
        self.inventory.push(item);
    }
    fn remove_from_inventory(&mut self, item: i32) {
        let index = self.inventory.iter().position(|x| *x == item).unwrap();
        self.inventory.remove(index);
    }
    fn print_inventory(&self) {
        println!("Inventory:");
        for item in &self.inventory {
            println!("{}", item);
        }
    }
}