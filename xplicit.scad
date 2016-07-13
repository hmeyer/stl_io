union(0.1) {



bend()
translate([0,0.5,0])
scale([1,0.3,1])
rotate([0,TAU/4,0])
twist(TAU*2/3)
cube([.1,0.3,TAU+1],s=.05);

translate([0,0.3,0])
rotate([0,TAU/4,0]) cylinder(r=.1,h=1.5);
}