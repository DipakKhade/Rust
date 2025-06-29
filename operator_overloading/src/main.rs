
struct Person {
    name: String,
    age:u32,
}

struct Marraige {
    husband: Person,
    wife:Person,
    date:chrono::naive::NaiveDate
}

impl std::ops::Add for Person{
    type Output = Marraige;
    fn add(self, rhs: Self) -> Self::Output {
        Marraige {
            husband:self,
            wife:rhs,
            date: chrono::Local::now().date_naive()
        }
    }
}

fn main() {
    let person1 = Person {
        name:"asd".to_string(),
        age:26
    };
    let person2 = Person {
        name:"xyz".to_string(),
        age:24
    };

    let marrage = person1 + person2;
}