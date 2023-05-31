mod Person{
    pub struct Personal_info {
        pub age: u8,
        pub education:String,
    }

    impl Personal_info {
        pub fn new(new_edu: &str) -> Self{
            Self {
                education:String::from(new_edu),
                age: 20,
            }
        }
    }
}

pub fn some_person(){
    let mut person1 = Person::Personal_info::new("Bachelors");
    person1.education = String::from("masters");

    let mut person2 = Person::Personal_info{
        age: 20,
        education: "masters",
    };

    person2.age = 33;
}