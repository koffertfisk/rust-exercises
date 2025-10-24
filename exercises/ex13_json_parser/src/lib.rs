pub mod user;

pub use user::User;

pub fn calculate_average_age(users: &[User]) -> u32
{
    let mut average: u32 = 0;
    
    if users.len() > 0 {
        let sum: u32 = users.iter().map(|u| u.age as u32).sum::<u32>();
        average = sum / users.len() as u32;
    }

    average
}   