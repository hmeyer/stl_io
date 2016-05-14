union(0.2) {

difference(0.5) {
  intersection(0.1) {
    translate([-.5,0,0]) sphere(.6);
    translate([-0.2,0.3,0]) sphere(.5);
  }
  translate([-.5,.4,0]) sphere(.2);
}
sphere(0.4);
translate([0.45,0,0]) sphere(0.3);
translate([0.3,0.25,0]) sphere(0.1);
}
