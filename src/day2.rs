 
 
 fn snow_island_game(game_rec: &str) -> u64 {
    let mut sum = 0;
    const GAME_BAG: [u64; 3] = [12, 13, 14];
    for game in game_rec.lines() {
        let game = game.split(':');
        let game_id: u64 = game[5..].parse().unwrap();
        
    }
    sum
 }
 
 #[cfg(test)]
 mod test{
    #[test]
    fn example(){
    }
}