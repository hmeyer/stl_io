

hole_dist = 0.2;


function band() = difference(0.06) {
len = 2;

difference(0.05) {
cube([.3,.2,len],s=0.05);

cr = 0.4;
translate([0,cr+0.03,0])
cylinder(r=cr,h=2*len);
translate([0,-(cr+0.03),0])
cylinder(r=cr,h=2*len);




}

rotate([TAU/4,0,0])
for(i=[-len/2:hole_dist:len/2]) {
translate([0,i,0])
cylinder(r=.07,h=1);
}

}


translate([0,0,hole_dist/2]) band();
rotate([0,0,TAU/4]) band();