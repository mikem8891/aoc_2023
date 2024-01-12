 pub fn snow_island_game(game_rec: &str) -> u64 {
    let mut sum = 0;
    const GAME_BAG: [u64; 3] = [12, 13, 14];
    for game in game_rec.lines() {
        let mut game = game.split(':');
        let game_id: u64 = game.next().unwrap()[5..].parse().unwrap();
        let subsets_of_cubes = game.next().unwrap();
        let mut max_colors = [0, 0, 0];
        for subset_of_cubes in subsets_of_cubes.split(';') {
            for num_color in subset_of_cubes.split(',') {
                let mut num_color = num_color.split(' ').filter(|s| !s.is_empty());
                let num: u64 = num_color.next().unwrap().parse().unwrap();
                let color = num_color.next().unwrap();
                match color {
                    "red"   => max_colors[0] = std::cmp::max(max_colors[0], num),
                    "green" => max_colors[1] = std::cmp::max(max_colors[1], num),
                    "blue"  => max_colors[2] = std::cmp::max(max_colors[2], num),
                    _ => panic!("Game {game_id} has an invalid color")
                }
            }
        }
        let used_game_bag = 
            max_colors[0] <= GAME_BAG[0] &&
            max_colors[1] <= GAME_BAG[1] &&
            max_colors[2] <= GAME_BAG[2];
        if used_game_bag {
            sum += game_id;
        }
    }
    sum
 }
 
pub fn snow_island_game_p2(game_rec: &str) -> u64 {
    let mut sum = 0;
    for game in game_rec.lines() {
        let mut game = game.split(':');
        let game_id: u64 = game.next().unwrap()[5..].parse().unwrap();
        let subsets_of_cubes = game.next().unwrap();
        let mut max_colors = [0, 0, 0];
        for subset_of_cubes in subsets_of_cubes.split(';') {
            for num_color in subset_of_cubes.split(',') {
                let mut num_color = num_color.split(' ').filter(|s| !s.is_empty());
                let num: u64 = num_color.next().unwrap().parse().unwrap();
                let color = num_color.next().unwrap();
                match color {
                    "red"   => max_colors[0] = std::cmp::max(max_colors[0], num),
                    "green" => max_colors[1] = std::cmp::max(max_colors[1], num),
                    "blue"  => max_colors[2] = std::cmp::max(max_colors[2], num),
                    _ => panic!("Game {game_id} has an invalid color")
                }
            }
        }
        let power: u64 = max_colors.iter().product();
        sum += power;
    }
    sum
}
 
#[cfg(test)]
mod test{
    use super::*;

    #[test]
    fn example(){
    }
    
    #[test]
    fn example_2(){
        let input =
r#"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"#;
        assert_eq!(2286, snow_island_game_p2(input))
    }
}