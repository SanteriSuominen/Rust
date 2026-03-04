
struct Vector3 
{
    x: f64,
    y: f64,
    z: f64,
}

fn main() 
{
    let mut player_position = Vector3 
    {
        x: 2.0,
        y: 3.0,
        z: 2.0,
    };

    let target = Vector3
    {
        x: 14.0,
        y: 0.0,
        z: 22.0,
    };

    let mut distance = calculate_distance(player_position, target);

    while distance > 0.0
    {
       distance = calculate_distance(player_position, target);
       println!("Distance between player and target is {distance}");

       if player_position.x > 0.0
       {
            player_position.x = player_position.x - 1.0;
       }
       if player_position.y > 0.0
       {
            player_position.y = player_position.y - 1.0;
       }
       if player_position.z > 0.0
       {
            player_position.z = player_position.z - 1.0;
       }
    }
}

fn calculate_distance(player_position: Vector3, target: Vector3) -> f64
{
    let distance = ((player_position.x - target.x).powi(2)
                   +(player_position.y - target.y).powi(2)
                   +(player_position.z - target.z).powi(2)).sqrt();
    
    distance
}
