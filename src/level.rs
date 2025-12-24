

const LEVEL_UP_AMMOUNTS : [i32;20] = [
     0,
     300,
     900,
     2700, 
     6500,
     14000,
     23000,
     34000,
     48000,
     64000,
     85000,
     100000,
     120000,
     140000,
     165000,
     195000,
     225000,
     265000,
     305000,
     355000
];

fn calculate_current_level(current_experience: i32) -> i32 
{
    LEVEL_UP_AMMOUNTS.partition_point(|x| x < &current_experience).max(1).try_into().unwrap()
}


struct level {
    level: i32,
    is_milestone: bool,

    experience: i32,
    
    profiency_bonus: i32,
}


impl level {
    fn add_experience(&mut self, ammount_to_add: i32) {
        if self.is_milestone {
            return
        }

        self.experience =  ammount_to_add;
    }
}


#[cfg(test)] 
mod tests{
    use super::*;

    #[test]
    fn test_level_calculation() {
        assert_eq!(calculate_current_level(0), 1);
        assert_eq!(calculate_current_level(500), 2);
        assert_eq!(calculate_current_level(500000), 20);
        assert_eq!(calculate_current_level(i32::MAX), 20);


        
    }

}