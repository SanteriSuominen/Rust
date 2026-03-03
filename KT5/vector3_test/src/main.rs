use rand::RngExt;
use std::thread::sleep;
use std::time::Duration;
struct Vector2 
{
    x: u32,
    y: u32,
}

fn main() 
{
    let mut player_position = Vector2
    {
        x: 0,
        y: 0,
    };

    let mut target = Vector2
    {
        x: 19,
        y: 9,
    };

    let grid = Vector2
    {
        x: 20,
        y: 10,
    };
        update_map(&player_position, &target, &grid);
        sleep(Duration::from_millis(100));
    while true
    {
        while player_position.x != target.x || player_position.y != target.y
        {
            if player_position.x != target.x
            {
                if player_position.x < target.x
                {
                    player_position.x = player_position.x + 1;
                }

                else if player_position.x > target.x
                {
                    player_position.x = player_position.x - 1;
                }   
            }

            if player_position.y != target.y
            {
                if player_position.y < target.y
                {
                    player_position.y = player_position.y + 1;
                }

                else if player_position.y > target.y
                {
                    player_position.y = player_position.y - 1;
                }
            } 
            update_map(&player_position, &target, &grid);
            sleep(Duration::from_millis(100));
        }
        println!("Hit");
        let mut rng = rand::rng();
        target.x = rng.random_range(0..=19);
        target.y = rng.random_range(0..=9);
    }
}

fn update_map(player_position: &Vector2, target: &Vector2, grid: &Vector2)
{
    print!("\x1B[2J\x1B[H");
    let grid_y = grid.y;
    let grid_x = grid.x;

    for y in 0..grid_y
    {
        for x in 0..grid_x
        {
            if player_position.x == x && player_position.y == y
            {
                print!("*");
            }

            else if target.x == x && target.y == y
            {
                print!("o");
            }

            else 
            {
                print!(".");    
            }
        }
        println!("");
    }
}
