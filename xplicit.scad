union(0.15) {

difference(0.25) {
  intersection(0.02) {
    translate([-.5,0,0]) sphere(.6);
    translate([-0.2,0.3,0]) sphere(.5);
  }
  translate([-.5,.4,0]) sphere(.4);
}
sphere(0.4);
translate([0.45,0,0]) sphere(0.3);
translate([0.3,0.25,0]) sphere(0.1);
}
