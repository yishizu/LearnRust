use bevy::prelude::*;

pub struct PeoplePlugin;

impl Plugin for PeoplePlugin {
    fn build(&self, app: &mut App) {
        // add things to your app here
        app.add_systems(Startup, setup)
            .add_systems(Update, person_printer)
            .add_systems(Update, person_with_jobs)
            .add_systems(Update, person_with_jobs_and_job)
            .add_systems(Update, person_ready_for_work);
    }
}

fn main() {
    App::new().add_plugins((DefaultPlugins, PeoplePlugin)).run();
}

pub fn hello_world() {
    println!("Hello, Bevy!");
}

pub fn setup(mut commands: Commands) {
    commands.spawn((
        Person {
            name: "Alice".to_string(),
            age: 30,
        },
        Employed { job: Job::Doctor },
    ));

    commands.spawn((
        Person {
            name: "Bob".to_string(),
            age: 31,
        },
        Employed {
            job: Job::FireFighter,
        },
    ));

    commands.spawn((
        Person {
            name: "Charlie".to_string(),
            age: 32,
        },
        Employed { job: Job::Teacher },
    ));
    commands.spawn((Person {
        name: "Dave".to_string(),
        age: 33,
    },));
}
pub fn person_printer(query: Query<&Person>) {
    for person in query.iter() {
        println!("Person: {} is {} years old", person.name, person.age);
    }
}

pub fn person_with_jobs(query: Query<&Person, With<Employed>>) {
    for person in query.iter() {
        println!("Person: {} has a job", person.name);
    }
}
pub fn person_with_jobs_and_job(query: Query<(&Person, &Employed)>) {
    for (person, employed) in query.iter() {
        let job_name = match employed.job {
            Job::Doctor => "Doctor",
            Job::FireFighter => "Fire Fighter",
            Job::Teacher => "Teacher",
            Job::Programmer => "Programmer",
            Job::Nurse => "Nurse",
        };
        println!("Person: {} has a job as a {:?}", person.name, job_name);
    }
}
pub fn person_ready_for_work(query: Query<&Person, Without<Employed>>) {
    for person in query.iter() {
        println!("Person: {} is ready for work", person.name);
    }
}

#[derive(Component)]
pub struct Person {
    pub name: String,
    pub age: u8,
}

#[derive(Component)]
pub struct Employed {
    pub job: Job,
}

#[derive(Debug)]
pub enum Job {
    Doctor,
    FireFighter,
    Teacher,
    Programmer,
    Nurse,
}
