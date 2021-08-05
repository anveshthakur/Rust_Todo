use std::collections::HashMap;
use std::io::Read;
use std::str::FromStr;

struct Todo {
    map: HashMap<String, bool>,
}

impl Todo {
    fn insert(&mut self, key: String) {
        self.map.insert(key, false);
    }

    fn save(self) -> Result<(), std::io::Error> {
        let mut content = String::new();
        for (k, v) in self.map {
            let record = format!("{}\t{}\n", k, v);
            content.push_str(&record)
        }
        std::fs::write("db.txt", content)
    }

    fn new() -> Result<Todo, std::io::Error>{
        let mut f = std::fs::OpenOptions::new()
            .write(true)
            .create(true)
            .read(true)
            .open("db.txt")?;
            let mut content = String::new();
            f.read_to_string(&mut content)?;
            let map : HashMap<String, bool> = content
                .lines() //iterator to the Lines in file
                .map(|line| line.splitn(2, '\t').collect::<Vec<&str>>()) //map takes a closure and then transforms it into some other type
                .map(|v| (v[0], v[1])) // map takes lines  -> vector -> tupple
                .map(|(k, v)| (String::from(k), bool::from_str(v).unwrap()))  // -> hashmap
                .collect();
            Ok(Todo{ map })
    }

    fn complete(&mut self, key: &String) -> Option<()> {
        match self.map.get_mut(key){  //get_mut with give us the mut value associate with this key
            Some(v) => Some(*v = true), //* derefrencing the value and change it
            None => None,
        }
    }

    fn count(self, key : &String) -> (usize, HashMap<String, bool>){
        if key == "-al"{
            return (self.map.len(), HashMap::new());
        }
        else{
            return (self.map.len(), self.map);
        }
    }
}


fn main() {
    let action = std::env::args().nth(1).expect("Please specify an action");
    let item = std::env::args().nth(2).expect("Please specify an item");

    let mut todo = Todo::new().expect("Initialization of db failed!!");

    if action == "help"{
        println!("Thank you for using RUSTODU \n 
                1. Use add [Task-Name] for creating/adding a task \n 
                2. Use complete [Task-Name] to complete the task \n 
                3. Use count -al for all the tasks \n 
                \t [count true] for completed \n 
                \t [count false] for tasks that are not done yet");
    }

    if action == "add" {
        todo.insert(item);
        match todo.save() {
            Ok(_) => println!("Item Added"),
            Err(why) => println!("An error occurred: {}", why),
        }
    }
    else if action == "complete"{
        match todo.complete(&item){
            None => println!("{} Not present here. You either completed it or maybe wrote it on your hand like Joey", item),
            Some(_) => match todo.save(){
                Ok(_) => println!("Saved"),
                Err(why) => println!("An error occured: {}", why),
            }
        }
    }
    else if action == "count"{
        if item == "-al"{   
            let (cnt, _) = todo.count(&item);
            println!("{}", cnt);
        }
        else if item == "true"{
            let(_, cntmap) = todo.count(&item);
            let mut count = 0;
            for(k, v) in cntmap{
                if v == true{
                    count += 1;
                    println!("{}", k);
                }
            }
            println!(" You are done with {} task(s)", count);
        }
        else if item == "false"{
            let(_, cntmap) = todo.count(&item);
            let mut count = 0;
            for(k, v) in cntmap{
                if v == false{
                    count += 1;
                    println!("{}", k);
                }
            }
            println!("You have {} thing(s) to do!!", count);
        }
    }
}