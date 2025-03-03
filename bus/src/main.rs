use rand::Rng;

struct Bus{
    id: String,
    capacity: i8,
    passengers: i8,
    stops: i8,
}
/*
fn attendance(buses: &[Bus]){
    for bus in buses{
        println!("Bus {} has showed up for attendance! Capacity: {} | Stops: {}",bus.id,bus.capacity,bus.stops);   
    }
    println!("Work day starting!\n################################################## \n");
}
*/

fn work(bus: &mut Bus){
    let mut rng = rand::thread_rng();
    println!("-- Bus {} --",bus.id);
   for s in 1..=bus.stops{
        let p = rng.gen_range(-1* bus.passengers ..= bus.capacity - bus.passengers);
        bus.passengers = bus.passengers + p;
        println!("Stop {}. Passengers {}/{} => {}",s,bus.passengers,bus.capacity,p);
   }
   println!("{} people left at the last stop.",bus.passengers);
}

fn main() {

let mut buses = [
    Bus { id: String::from("A123"), capacity: 50, passengers: 0, stops: 10 },
    Bus { id: String::from("B456"), capacity: 40, passengers: 0, stops: 8 },
    Bus { id: String::from("C789"), capacity: 60, passengers: 0, stops: 12 },
    Bus { id: String::from("D101"), capacity: 30, passengers: 0, stops: 5 },
];

//attendance(&buses);
for bus in &mut buses{
    work(bus);
}

}

