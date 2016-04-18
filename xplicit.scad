difference(r=0.3) {
  union(r=0.4) {
    translate([-.5, 0, 0]) sphere(.5);
    translate([ .5, 0, 0]) sphere(.8);
  }
  translate([0, 0, .5]) sphere(.5);
}
