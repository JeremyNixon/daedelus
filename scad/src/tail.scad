include <lib/vertex.scad>
union() {
  /*
  difference() {
    rotate(90, [0,0,1]) rotate(90, [1,0,0]) linear_extrude(30) difference() {
      square([16.5,30],true);
      square([11.5,25],true);
    }
  }
  */
  translate([0,10,0]) rotate(120, [0,1,0]) translate([-10,0,0]) rotate(90) linear_extrude(15) difference() {
    square([16.5,30],true);
    square([11.5,25],true);
  }
  translate([0,10,0]) rotate(-120, [0,1,0]) translate([10,0,0]) rotate(-90) linear_extrude(15) difference() {
    square([16.5,30],true);
    square([11.5,25],true);
  }

  translate([0,5,0])
  rotate(90, [1,0,0])
  rotate(90)
  linear_extrude(3.25)
  polygon([[0,0],[20,-12],[20,12]]);

  translate([0,19,0])
  rotate(90, [1,0,0])
  rotate(90)
  linear_extrude(3.25)
  polygon([[0,0],[20,-12],[20,12]]);

  translate([-12.5,1.75,19]) linear_extrude(3) square([25,16.5]);

  translate([-5.5,0,0]) vertex([[-2,-12,-1]]);
  vertex([[0,-12,1]]);
  translate([5.5,0,0]) vertex([[2,-12,-1]]);
  
  translate([2,0,30]) vertex([[2,-12,0]]);
  translate([-2,0,30]) vertex([[-2,-12,0]]);

  translate([0,0,2]) linear_extrude(30) square(7,true);
  translate([0,10,20]) linear_extrude(13) square([7,15],true);
}
