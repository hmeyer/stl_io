size=20;

bend(5)
translate([0,size*.5,0])
rotate([0,TAU/4,0])
twist(5*4)
cube([size*.4,size*.4,60],s=size*0.1);
