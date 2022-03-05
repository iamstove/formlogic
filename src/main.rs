//Author: Steven Turner
/* 
    The purpose of this program is to generate gcode based on a text input that describes the intended output.

    The program will snake through the given input to produce gcode output.
*/

use std::env;
use utils::read_to_str_vec;

struct LaserCutter {
    x_pos: f32,
    y_pos: f32,
    laser_state: bool,
    total_time: f32
}

impl LaserCutter {
    // constructor: laser off 0,0 position
    fn new() -> LaserCutter {
        LaserCutter {
            x_pos: 0.0,
            y_pos: 0.0,
            laser_state: false,
            total_time: 0.0
        }
    }

    /* 
        Sets the position of the laser head, calculates time to move, and outputs gcode for new position
        takes x and y as f32 for new position
    */
    fn set_pos(&mut self, x: f32, y: f32) {
        self.total_time  += self.calculate_distance(self.x_pos, self.y_pos, x, y);
        self.x_pos = x;
        self.y_pos = y;
        self.output_pos_gcode();
    }

    // toggles the laser state, prints gcode, and adds to the time
    fn toggle_laser(&mut self){
        self.laser_state = !self.laser_state;
        println!("M01");
        //turning on/off the laser takes 1 second
        self.total_time += 1.0;
    }

    //displays the current total time in a pretty format, to be used when gcode is finished being generated.
    fn display_time(&self){
        println!("This Gcode should take {:.1} seconds to run", self.total_time);
    }

    //small function to output gcode of current position
    fn output_pos_gcode(&self){
        println!("G01 X{:.2} Y{:.2}", self.x_pos, self.y_pos);
    }

    //function to calculate the distance between 2 points
    fn calculate_distance (&mut self, x1: f32, y1: f32, x2: f32, y2: f32) -> f32
    {
        //distance formula: sqrt((x2-x1)^2 + (y2-y1)^2)
        ((x2-x1).powi(2) + (y2-y1).powi(2)).sqrt()
    }
}

fn main() {
    // create our laser cutter to perform actions on
    let mut laser = LaserCutter::new();
    // get text file name from command line
    let args: Vec<String> = env::args().collect();
    
    //only takes one command line arg (executing file name is automatically included)
    assert!(args.len() == 2);

    let input = read_to_str_vec(args[1].as_str());
    for i in 0..input.len() {
        if (i % 2) == 0 {
            for c in input[i].char_indices() {
                if c.1 == 'X' && !laser.laser_state{
                    laser.set_pos(i as f32, c.0 as f32);
                    laser.toggle_laser();
                }
                else if c.1 == 'X' && laser.laser_state {
                    //just move to position
                    laser.set_pos(i as f32, c.0 as f32);
                }
                else if laser.laser_state{
                    //if on, turn off
                    laser.toggle_laser();
                }
            }
        }
        else {
            for c in input[i].char_indices().rev() {
                if c.1 == 'X' && !laser.laser_state{
                    laser.set_pos(i as f32, c.0 as f32);
                    laser.toggle_laser();
                }
                else if c.1 == 'X' && laser.laser_state {
                    //just move to position
                    laser.set_pos(i as f32, c.0 as f32);
                }
                else if laser.laser_state{
                    //if on, turn off
                    laser.toggle_laser();
                }
            }
        }
    }
    //we've parsed all the gcode, home the laser
    laser.set_pos(0.0,0.0);
    laser.display_time();
}

mod tests{
    use super::LaserCutter;

    #[test]
    //this test checks that the distance calculation is correct
    fn test_calc_distance() {
        let mut laser = LaserCutter::new();
        assert_eq!(laser.calculate_distance(0.0,0.0,3.0,4.0), 5.0);
    }

    #[test]
    //this test checks that set pos updates correctly
    fn test_set_pos() {
        let mut laser = LaserCutter::new();
        laser.set_pos(5.0, 3.0);
        assert_eq!(laser.x_pos, 5.0);
        assert_eq!(laser.y_pos, 3.0);
        assert_eq!(laser.total_time, 34.0_f32.sqrt());
        
    }

    #[test]
    //this test checks that toggling the laser will add to the total time
    fn test_laser_toggle() {
        let mut laser = LaserCutter::new();
        laser.toggle_laser();
        assert!(laser.laser_state == true);
        assert_eq!(laser.total_time, 1.0);
    }

    #[test]
    //this test will test a full run of gcode to test the total time.
    fn test_full_run() {
        let mut laser = LaserCutter::new();
        laser.set_pos(2.0,1.0);
        laser.toggle_laser();
        laser.set_pos(6.0,1.0);
        laser.toggle_laser();
        laser.set_pos(2.0,3.0);
        laser.toggle_laser();
        laser.set_pos(6.0,3.0);
        laser.toggle_laser();
        laser.set_pos(0.0,0.0);
        
        let time = (laser.total_time * 100.0).round() / 100.0;
        assert_eq!(time, 25.42);
    }
}