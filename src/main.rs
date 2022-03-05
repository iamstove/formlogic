//Author: Steven Turner
/* 
    The purpose of this program is to generate gcode based on a text input that describes the intended output.

    It is broken into a few setions, the first parses the input and turns it into a weighted graph

    The second takes that weighted graph and finds the best path, using dijkstra's

    The third genreates the output gcode
*/

use utils::read_file;
use std::io::BufRead;  

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
        print!("M01\n");
        //turning on/off the laser takes 1 second
        self.total_time += 1.0;
    }

    //displays the current total time in a pretty format, to be used when gcode is finished being generated.
    fn display_time(&self){
        print!("This Gcode should take {:.1} seconds to run", self.total_time);
    }

    //small function to output gcode of current position
    fn output_pos_gcode(&self){
        print!("G01 X{:.2} Y{:.2}\n", self.x_pos, self.y_pos);
    }

    //function to calculate the distance between 2 points
    fn calculate_distance (&mut self, x1: f32, y1: f32, x2: f32, y2: f32) -> f32
    {
        //distance formula: sqrt((x2-x1)^2 + (y2-y1)^2)
        ((x2-x1).powi(2) + (y2-y1).powi(2)).sqrt()
    }
}

fn main() {
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
  laser.display_time()
}

