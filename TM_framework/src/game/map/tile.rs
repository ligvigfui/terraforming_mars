pub mod mars_tile;
pub mod custom_tile;

pub trait Tile {}

pub trait Placeable {}

pub fn next_to(coords: (i32, i32)) -> Vec<(i32, i32)>{
    vec![(coords.0 +1, coords.1 ),
        (coords.0 , coords.1 +1),
        (coords.0 +1, coords.1 -1),
        (coords.0 -1, coords.1 +1),
        (coords.0 , coords.1 -1),
        (coords.0 -1, coords.1 ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn next_to_test(){
        let next_to = next_to((0, 0));
        assert!(next_to.contains(&(1, 0)));
        assert!(next_to.contains(&(-1, 0)));
        assert!(next_to.contains(&(1, -1)));
        assert!(next_to.contains(&(-1, 1)));
        assert!(next_to.contains(&(0, -1)));
        assert!(next_to.contains(&(0, 1)));
    }
}