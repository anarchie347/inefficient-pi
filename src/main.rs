
fn main() {


    let ratio = 1e8;

    let mut state = Simstate {
        v1: 0f64,
        v2: -100f64, // should be independent, must be -ve
        m1: 1f64,
        m2: ratio as f64 //should be m1 * (100^n)
    };
    let mut counter = 0;
    let mut coll_type = false; //true -> wall, false -> block
    while state.v1 > state.v2 || state.v1 < 0f64 {
        let collide = if coll_type {wall_collide} else {block_collide};
        
        state = collide(state);
        counter += 1;
        coll_type = !coll_type;
    }
    print!("ration {}:1, #collisions = {}",  ratio, counter);
}

fn block_collide(state : Simstate) -> Simstate {
    let energy2 = calc_2energy(&state);
    let momentum = calc_momentum(&state);
    //v2 is bigger block, so v is always increasing (-> +ve), so newv2 will always be the positive solution
    let newv2 = solve_quadratic_pos(state.m2.powf(2f64) + state.m2 * state.m1, -(2f64 * state.m2 * momentum),momentum.powf(2f64) - state.m1 * energy2);
    //v1 is smaller block, so for a block collision, v is always decreasing, so newv1 will be negative sol
    let newv1 = solve_quadratic_neg(state.m1.powf(2f64) + state.m1 * state.m2, -(2f64 * state.m1 * momentum), momentum.powf(2f64) - state.m2 * energy2);
    
    Simstate {
        v1: newv1,
        v2: newv2,
        ..state
    }

}



fn wall_collide(state : Simstate) -> Simstate {
    return Simstate {
        v1: -state.v1,
        ..state
    }
}

fn solve_quadratic_pos(a : f64, b : f64, c : f64) -> f64{
    (-b + discriminant(a, b, c).sqrt()) / (2f64 * a)
}

fn solve_quadratic_neg(a : f64, b : f64, c : f64) -> f64{
    (-b - discriminant(a, b, c).sqrt()) / (2f64 * a)
}

fn discriminant(a: f64, b : f64, c : f64) -> f64 {
    b.powf(2f64) - (4f64 * a * c)
}

fn calc_2energy(state : &Simstate) -> f64 {
    state.m1 * state.v1.powf(2f64) + state.m2 * state.v2.powf(2f64)
}

fn calc_momentum(state : &Simstate) -> f64 {
    state.m1 * state.v1 + state.m2 * state.v2
}


struct Simstate {
    pub v1 : f64,
    pub v2 : f64,
    pub m1 : f64,
    pub m2 : f64
}