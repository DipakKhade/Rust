use std::vec;

fn main() {
    let v = vec![1, 2, 3];
    let v_ref = &v[1];
    println!("{}", v_ref);

    let v2 = vec![7, 8, 9, 11, 321, 324, 53, 546, 3, 665, 34, 543, 532, 543];
    let tenth_ele = v2.get(10);

    let ans = match tenth_ele {
        Some(ele) => {
            print!("val is this {ele}");
            ele
        }
        None_ => {
            println!("no value found");
            &-1
        }
    };

    print!(" ans is {:?}", ans);

    #[derive(Debug)]
    enum SpreedSheetCell {
        int(i32),
        float(f64),
        text(String),
    }

    let mix_v: Vec<SpreedSheetCell> = vec![
        SpreedSheetCell::text(String::from("asd")),
        SpreedSheetCell::int(2),
    ];

    println!("{:?}", mix_v)
}
