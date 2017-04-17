
difference(s=2) {
sphere(r=10);
icylinder(r=5);
rotate([TAU/4, 0, 0]) icylinder(r=5);
rotate([0, TAU/4, 0]) icylinder(r=5);
}

cylinder(r=4,h=22, s=2);