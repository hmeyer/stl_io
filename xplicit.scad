union(0.4) for(x=[-1:0.5:1],y=[-1:0.5:1]) {
  xs = (x*x+.5)/2;
  ys = (y*y+.5)/2;
  translate([x,0,y]) sphere(xs*ys);
}
