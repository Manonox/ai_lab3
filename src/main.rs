
mod chinakers;
use chinakers::Field;

mod astar;
use astar::{AStar, AStarSolution};


fn main() {
    let f = Field::new();

    let mut astar = AStar::new(&f);
    
    let mut result_option: Option<Result<AStarSolution, ()>> = Default::default();

    const MAX_STEPS: i32 = 160000000;
    let mut steps = 0;
    while result_option.is_none() && steps < MAX_STEPS {
        result_option = astar.step();
        // std::thread::sleep(std::time::Duration::from_secs_f32(0.5));
        steps += 1;
    }

    let result = result_option.unwrap();
    
    match result {
        Ok(_) => {
            println!("Solved!");
        },

        Err(()) => {
            println!("Chto-to poshlo ne tak...");
        }
    }
}
