use rand::{Rng, thread_rng};
use rand::distributions::{Distribution, Uniform, Standard, Alphanumeric};
use rand_distr::{Normal};

//生成随机数
pub fn build_rand() -> () {
    let mut random = thread_rng();
    let num1: f32 = random.gen();
    let num2 = random.gen::<f32>();

    dbg!(num1);
}

//生成范围随机数
//生成的数字是左闭右开区间(和平时的范围一致，使用..=会出现右闭值)
pub fn build_rand_range() -> () {
    let mut random = thread_rng();
    let range_num1 = random.gen_range(0..=2);
    dbg!(range_num1);
}

//使用Uniform进行生成，代码的性能可能会更好
pub fn build_rand_range_better() -> () {
    let mut random = thread_rng();
    let form = Uniform::from(0..12);
    let range_num = form.sample(&mut random);
    dbg!(range_num);
}

//生成给定分布的随机数（正态分布）(线性代数、离散数学)
// [0,2,4,6] => dis = 2 , avg = 3
// [0,1,2,3,4,5,6] => dis = 1 , avg = 3
pub fn build_normal_distribution() -> () {
    let mut random = thread_rng();
    let normal = Normal::new(-1.0, 3.0).unwrap();
    // let rand_num = rand_distr::Distribution::sample(&normal, &mut random);
    let rand_num = normal.sample(&mut random);
    dbg!(rand_num);
}

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    pub fn new() -> Point {
        let mut tmp_rand = thread_rng();
        Point {
            x: tmp_rand.gen::<i32>(),
            y: tmp_rand.gen::<i32>(),
        }
    }
}

impl Distribution<Point> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Point {
        let (x, y) = rng.gen();
        Point {
            x,
            y,
        }
    }
}


//生成自定义类型随机数
pub fn build_rand_type() -> () {
    let mut random = thread_rng();
    let num1 = random.gen::<(u8, i16, f32, bool)>();
    dbg!(num1);
    let p = Point::new();
    let p2 = random.gen::<Point>();
    dbg!(p);
    dbg!(p2);
}

//生成随机字符串
pub fn build_random_string() -> () {
    let mut random = thread_rng();
    let rand_str = random.sample_iter(&Alphanumeric)
        .take(15)
        .map(char::from)
        .collect::<String>();
    dbg!(rand_str);
}

//从自定义范围中生成随机字符串
pub fn build_random_string_bind() -> () {
    let CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                            abcdefghijklmnopqrstuvwxyz\
                            0123456789)(*&^%$#@!~";
    let PASSWORD_LEN: usize = 30;
    let mut rng = rand::thread_rng();

    let password: String = (0..PASSWORD_LEN)
        .map(|_| {
            let idx = rng.gen_range(0..CHARSET.len());
            CHARSET[idx] as char
        })
        .collect();

    println!("{:?}", password);
}
