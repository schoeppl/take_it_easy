extern crate rand;
extern crate time;

use time::PreciseTime;
use std::fmt::Display;
use std::fmt;
use std::fmt::Write;
use rand::Rng;
use std::thread;
use std::default::Default;
use std::sync::mpsc::channel;
/* Tile

| a
|
| 

\ b
 \
  \ 

  / c
 /
/
*/
#[derive(Default, Debug, Copy, Clone)]
struct Tile{
    a : u32,
    b : u32,
    c : u32,
}

#[derive(Default)]
#[derive(Copy, Clone)]
struct Field{
    tile : [Tile;19],
    tile_free : [Tile;8],
}

/* Board Index
        7
    3       12
0       8       16
    4       13
1       9       17
    5       14
2       10      18
    6       15
        11
*/
impl Field{
    fn new(v : &Vec<Tile>) -> Self{ //Find a better way! 
        assert_eq!(v.len(),27);
        let mut pickout = v.clone();
        rand::thread_rng().shuffle(&mut pickout);
        let mut selection : [Tile;19] = Default::default();
        let mut not_selected : [Tile;8] = Default::default();

        for i in 0 .. 19 {
            selection[i] = pickout.pop().unwrap();
        }

        assert_eq!(pickout.len(),8);

        for i in 0 .. 8 {
            not_selected[i] = pickout.pop().unwrap();
        }

        assert_eq!(pickout.len(),0);

        Field{tile : selection, tile_free : not_selected}
    }

    fn health(&self) -> u32{
        Field::health_a(&self) + Field::health_b(&self) + Field::health_c(&self)
    }

    fn health_a(&self) -> u32{
        let mut result : u32 = 0;

        if self.tile[0].a == self.tile[1].a && self.tile[0].a == self.tile[2].a {
            result = result + self.tile[0].a + self.tile[1].a + self.tile[2].a;
        }

        if self.tile[3].a == self.tile[4].a  && self.tile[3].a == self.tile[5].a && self.tile[3].a == self.tile[6].a{
            result = result + self.tile[3].a + self.tile[4].a + self.tile[5].a + self.tile[6].a;
        }

        if self.tile[7].a == self.tile[8].a && self.tile[7].a == self.tile[9].a &&self.tile[7].a == self.tile[10].a &&self.tile[7].a == self.tile[11].a {
            result = result + self.tile[7].a + self.tile[8].a + self.tile[9].a + self.tile[10].a + self.tile[11].a;
        }

        if self.tile[12].a == self.tile[13].a && self.tile[12].a == self.tile[14].a &&self.tile[12].a == self.tile[15].a{
            result = result + self.tile[12].a + self.tile[13].a + self.tile[14].a + self.tile[15].a;
        }

        if self.tile[16].a ==  self.tile[17].a && self.tile[16].a ==  self.tile[18].a{
            result = result + self.tile[16].a + self.tile[17].a + self.tile[18].a;
        }
        
        result
    }

    fn health_b(&self) -> u32{
        let mut result : u32 = 0;

        if self.tile[7].b == self.tile[12].b && self.tile[7].b == self.tile[16].b {
            result = result + self.tile[7].b + self.tile[12].b + self.tile[16].b;
        }

        if self.tile[3].b == self.tile[8].b  && self.tile[3].b == self.tile[13].b && self.tile[3].b == self.tile[17].b{
            result = result + self.tile[3].b + self.tile[8].b + self.tile[13].b + self.tile[17].b;
        }

        if self.tile[0].b == self.tile[4].b && self.tile[0].b == self.tile[9].b &&self.tile[0].b == self.tile[14].b &&self.tile[0].b == self.tile[18].b {
            result = result + self.tile[0].b + self.tile[4].b + self.tile[9].b + self.tile[14].b + self.tile[18].b;
        }

        if self.tile[1].b == self.tile[5].b && self.tile[1].b == self.tile[10].b &&self.tile[1].b == self.tile[15].b{
            result = result + self.tile[1].b + self.tile[5].b + self.tile[10].b + self.tile[15].b;
        }

        if self.tile[2].b ==  self.tile[6].b && self.tile[2].b ==  self.tile[11].b{
            result = result + self.tile[2].b + self.tile[6].b + self.tile[11].b;
        }
        
        result
    }

    fn health_c(&self) -> u32{
        let mut result : u32 = 0;

        if self.tile[0].c == self.tile[3].c && self.tile[0].c == self.tile[7].c {
            result = result + self.tile[0].c + self.tile[3].c + self.tile[7].c;
        }

        if self.tile[1].c == self.tile[4].c  && self.tile[1].c == self.tile[8].c && self.tile[1].c == self.tile[12].c{
            result = result + self.tile[1].c + self.tile[4].c + self.tile[8].c + self.tile[12].c;
        }

        if self.tile[2].c == self.tile[5].c && self.tile[2].c == self.tile[9].c &&self.tile[2].c == self.tile[13].c &&self.tile[2].c == self.tile[16].c {
            result = result + self.tile[2].c + self.tile[5].c + self.tile[9].c + self.tile[13].c + self.tile[16].c;
        }

        if self.tile[6].c == self.tile[10].c && self.tile[6].c == self.tile[14].c &&self.tile[6].c == self.tile[17].c{
            result = result + self.tile[6].c + self.tile[10].c + self.tile[14].c + self.tile[17].c;
        }

        if self.tile[11].c ==  self.tile[15].c && self.tile[11].c ==  self.tile[18].c{
            result = result + self.tile[11].c + self.tile[15].c + self.tile[18].c;
        }
        
        result
    }

    fn prime_key(&self) -> u32{
        let primes = [2,3,5,7,11,13,17,19,23,29,31,37,41,43,47,53,59,61,67];
        let mut res = 0;
        for (p,t) in primes.iter().zip(self.tile.iter()){
            res = p*(t.a * t.b * t.c);
        }
        res
    }
}

impl Display for Tile{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        //write!(f, " |{}\n/:{}   \\:{}", self.a, self.c, self.b)
        write!(f, "a{}b{}c{}", format!("{:02}",self.a) , format!("{:02}",self.b),format!("{:02}",self.c)) 
    }
}

impl Display for Field{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut res = String::new(); 
        for i in 0 .. self.tile.len() {
            write!(&mut res, "{} {}\n", format!("{:02}",i), self.tile[i]).unwrap();
        }
        write!(f,"{}",res)
    }
}

struct Generation{
    boards : Vec<Field>,
    sum : u32,
    avg : u32,
    max : u32,
    min : u32
}

impl Default for Generation{
    fn default() -> Generation {
        Generation {
            boards : Default::default(),
            sum: 0,
            avg: 0,
            max: 0,
            min: <u32>::max_value(),
        }
    }
}
impl Generation{
    fn select(&mut self) -> Vec<Field>{
        let mut survivor : Vec<Field> = self.boards.clone();

        survivor.sort_by_key(|x| MAX_POINTS - x.health());
        
        self.max = survivor[0].health();
        self.min = survivor[survivor.len()-1].health();

        //calc sum and avg of current/last generation
        for f in 0 .. survivor.len(){
            let health = survivor[f].health();
            self.sum = self.sum + health;
        }

        self.avg = self.sum / survivor.len() as u32;

        survivor.dedup_by_key(|d| d.prime_key());

        let survivor_count = (GENERATION_SIZE as f32 * (1 as f32 - MAX_SURVIVOR)) as usize;

        survivor.truncate(survivor_count);
        survivor
    }

    fn generate_first(possible_tile : &Vec<Tile> , size : u32) -> Self{
        let mut result : Vec<Field> = Vec::new();

        for _i in 0 .. size {
            result.push(Field::new(&possible_tile));
        }

        Generation{ boards : result, ..Default::default() }
    }

    fn generate_next(possible_tile : &Vec<Tile> , size : u32, survivor : Vec<Field>) -> Self{
        assert!(survivor.len() > 0);
        let mut rng = rand::thread_rng();
        let mut next_gen : Vec<Field> = Vec::new();

        let count_random = ((size as f32) * AMOUNT_RANDOM) as u32; //example: Population = 1000 -> 50 randomly generated new games will be added

        next_gen.append(&mut survivor.clone());

        //using survivor, randomly pic one, alter it, add to next_gen
        while (next_gen.len() as u32) < (size - count_random){
            let next : Field = Generation::gen_next(rng.choose(&survivor).unwrap());            
            next_gen.push(next);
        }

        //generate some new games and add them to the next_gen
        next_gen.append(&mut Generation::generate_first(&possible_tile,count_random).boards);
        
        //REMOVE THIS!
        while next_gen.len() as u32 > size{ 
            next_gen.pop();
        }

        Generation{boards : next_gen, ..Default::default() }
    }

    fn gen_next(old : &Field) ->  Field{
        let mut rng = rand::thread_rng();

        let mut selection : [Tile;19] = old.tile;
        let mut not_selected : [Tile; 8] = old.tile_free;

        let mut min_moves = 0;
        let mut min_changes = 0;

        old.prime_key();

        if FORCE_MOVE {
            min_moves = 1;
        }

        if FORCE_CHANGE {
            min_changes = 1;
        }

        let moves: u32 = rng.gen_range(min_moves, MAX_MOVES);
        let changes: u32 = rng.gen_range(min_changes, MAX_CHANGE);

        for _i in 0 .. moves{
            let rn_1 = rng.gen_range(0, 19);
            let rn_2 = rng.gen_range(0, 19);
            let tile_1 = selection[rn_1];

            selection[rn_1] = selection[rn_2];
            selection[rn_2] = tile_1;
        }

        for _i in 0 .. changes{
            let rn_cur = rng.gen_range(0, 19);
            let rn_free = rng.gen_range(0, 8);
            let tile_cur = selection[rn_cur];
            let tile_free = not_selected[rn_free];

            not_selected[rn_free] = tile_cur;
            selection[rn_cur] = tile_free;
        }
        Field{tile: selection, tile_free : not_selected}
    }
}

static MAX_POINTS : u32 = 307;

//GENETIC
static MAX_SURVIVOR : f32 = 0.5;
static MAX_MOVES : u32 = 5;
static MAX_CHANGE : u32 = 5;
static AMOUNT_RANDOM : f32 = 0.00;
static FORCE_MOVE : bool = false;
static FORCE_CHANGE : bool = true;

static GENERATION_SIZE : u32 = 100;
static GENERATIONS : u32 =  1000000;

//RANDOM
static MAX_RANDOM: u32 =  10000000;

fn main() {
    let mut possible_tile : Vec<Tile> = Vec::new();

    for a in vec![1,5,9].iter(){
        for b in vec![3,4,8].iter(){
            for c in vec![2,6,7].iter(){
                possible_tile.push(Tile{a:*a,b:*b,c:*c});
            }
        }
    }

    let possible_tile_genertic = possible_tile.clone();
    let possible_tile_random = possible_tile.clone();

    let genetic = thread::spawn(move || {
        genetic_search(&possible_tile_genertic)
    });
    
    let random = thread::spawn(move || {
         random_search(&possible_tile_random)
    });

    
    let gen_res : Field = genetic.join().expect("Error during Genetic Search");
    let rand_res : Field = random.join().expect("Error during Rand Search");

    println!("\n\n......\nGeneration Board:\n\n{} Health:{}", gen_res , gen_res.health());
    println!("\n\n......\nRandom Board:\n\n{} Health:{}", rand_res , rand_res.health());
}

fn genetic_search(possible_tile : &Vec<Tile>) -> Field{
    let start = PreciseTime::now();

    let mut first_generation = Generation::generate_first(&possible_tile, GENERATION_SIZE);
    let mut survivor = first_generation.select();

    for i in 0 .. GENERATIONS{
        let mut next_generation = Generation::generate_next(&possible_tile, GENERATION_SIZE, survivor);
        assert_eq!(next_generation.boards.len() as u32,GENERATION_SIZE);

        survivor = next_generation.select();

        if i % (GENERATIONS as f32 * 0.10) as u32 == 0{
            println!("{}: sum: {}, avg: {}, min: {}, max: {}, survivor: {}",i,next_generation.sum, next_generation.avg, next_generation.min, next_generation.max, survivor.len());
        }

        if next_generation.max == MAX_POINTS{
            break;
        }
    }

    let end = PreciseTime::now();
    println!("{} seconds for genetic search.", start.to(end));
    *survivor.iter().max_by_key(|x| x.health()).unwrap()
}

fn random_search(possible_tile : &Vec<Tile>) -> Field{
    let mut health = 0;
    let mut max = 0;
    let mut i = 0;
    let mut max_f = Default::default();

    let start = PreciseTime::now();

    while health < MAX_POINTS && i < MAX_RANDOM{
        let f = Field::new(&possible_tile);
        health = f.health();
        if health > max{
            max = health;
            max_f = f.clone();
        }
        i = i + 1;
    }

    let end = PreciseTime::now();
    println!("{} seconds for random search.", start.to(end));
    max_f
}
