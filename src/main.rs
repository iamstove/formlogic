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
    laser_state: bool
}

impl LaserCutter {
    // constructor: laser off 0,0 position
    fn new() -> LaserCutter {
        LaserCutter {
            x_pos: 0.0,
            y_pos: 0.0,
            laser_state: false      
        }
    }

    // sets the position of the laser head
    fn set_pos(&mut self, x: f32, y: f32) {
        self.x_pos = x;
        self.y_pos = y;
        self.output_pos_gcode();
    }

    // toggles the laser state
    fn toggle_laser(&mut self){
        self.laser_state = !self.laser_state;
        print!("M01\n");
    }

    fn display(&self){
        print!("Position: {},{}\nState: {}\n", self.x_pos, self.y_pos, self.laser_state);
    }

    fn output_pos_gcode(&self){
        print!("G01 X{:.2} Y{:.2}\n", self.x_pos, self.y_pos);
    }

}

fn main() {
  let mut laser = LaserCutter::new();
  laser.set_pos(2.0,1.0);
  laser.toggle_laser();
}

